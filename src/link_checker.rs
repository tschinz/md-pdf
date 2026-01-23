use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::Duration;

/// Check all links in a markdown file and display warnings for unreachable links
pub fn check_links_in_file(md_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(md_file)?;
    let links = extract_links(&content);

    if links.is_empty() {
        println!("ℹ️  No links found in the markdown file");
        return Ok(());
    }

    println!(
        "🔍 Checking {} links in '{}'...",
        links.len(),
        md_file.display()
    );

    let mut successful_links = 0;
    let mut failed_links = 0;

    // Create a blocking HTTP client
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("md-pdf-link-checker/1.0")
        .build()?;

    for link in links {
        match client.head(&link).send() {
            Ok(response) => {
                if response.status().is_success() {
                    println!("✅ {}", link);
                    successful_links += 1;
                } else {
                    println!("❌ {} - HTTP {}", link, response.status());
                    failed_links += 1;
                }
            }
            Err(e) => {
                // Try GET request if HEAD fails (some servers don't support HEAD)
                match client.get(&link).send() {
                    Ok(response) => {
                        if response.status().is_success() {
                            println!("✅ {} (via GET)", link);
                            successful_links += 1;
                        } else {
                            println!("❌ {} - HTTP {}", link, response.status());
                            failed_links += 1;
                        }
                    }
                    Err(_) => {
                        println!("❌ {} - Error: {}", link, e);
                        failed_links += 1;
                    }
                }
            }
        }
    }

    println!("\n📊 Link check summary:");
    println!("  ✅ Successful: {}", successful_links);
    println!("  ❌ Failed: {}", failed_links);

    if failed_links > 0 {
        println!("⚠️  Some links are not reachable. Consider reviewing them.");
    } else {
        println!("🎉 All links are reachable!");
    }

    Ok(())
}

/// Extract all links from markdown content
fn extract_links(content: &str) -> Vec<String> {
    let mut links = HashSet::new();

    // Regex patterns for different markdown link formats
    let link_patterns = [
        // Standard markdown links: [text](url)
        regex::Regex::new(r"\[([^\]]*)\]\(([^)]+)\)").unwrap(),
        // Auto-links: <url>
        regex::Regex::new(r"<(https?://[^>]+)>").unwrap(),
        // Raw URLs (basic detection)
        regex::Regex::new(r"(?:^|\s)(https?://[^\s]+)").unwrap(),
    ];

    for pattern in &link_patterns {
        for captures in pattern.captures_iter(content) {
            // For the first pattern, URL is in capture group 2
            // For others, it's in capture group 1
            let url = if captures.len() > 2 {
                captures.get(2).map(|m| m.as_str())
            } else {
                captures.get(1).map(|m| m.as_str())
            };

            if let Some(url) = url {
                // Filter out relative paths, anchors, and mailto links
                if url.starts_with("http://") || url.starts_with("https://") {
                    // Clean up URL (remove trailing punctuation that might be part of sentence)
                    let cleaned_url = url.trim_end_matches(&['.', ',', ';', ')', ']'][..]);
                    links.insert(cleaned_url.to_string());
                }
            }
        }
    }

    links.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_links_standard() {
        let content = "Here is a [link](https://example.com) and another [one](http://test.org).";
        let links = extract_links(content);
        assert_eq!(links.len(), 2);
        assert!(links.contains(&"https://example.com".to_string()));
        assert!(links.contains(&"http://test.org".to_string()));
    }

    #[test]
    fn test_extract_links_autolink() {
        let content = "Visit <https://example.com> for more info.";
        let links = extract_links(content);
        assert_eq!(links.len(), 1);
        assert!(links.contains(&"https://example.com".to_string()));
    }

    #[test]
    fn test_extract_links_raw_url() {
        let content = "Check out https://example.com and http://test.org for details.";
        let links = extract_links(content);
        assert_eq!(links.len(), 2);
        assert!(links.contains(&"https://example.com".to_string()));
        assert!(links.contains(&"http://test.org".to_string()));
    }

    #[test]
    fn test_extract_links_filters_relative() {
        let content = "Local [link](./local.md) and external [link](https://example.com).";
        let links = extract_links(content);
        assert_eq!(links.len(), 1);
        assert!(links.contains(&"https://example.com".to_string()));
    }

    #[test]
    fn test_extract_links_no_duplicates() {
        let content = "[Link1](https://example.com) and [Link2](https://example.com).";
        let links = extract_links(content);
        assert_eq!(links.len(), 1);
        assert!(links.contains(&"https://example.com".to_string()));
    }
}
