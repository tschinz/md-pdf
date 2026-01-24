use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Represents the front matter metadata from a markdown file
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct FrontMatter {
  /// Document title
  pub title: Option<String>,
  /// Document subtitle
  pub subtitle: Option<String>,
  /// Document author
  pub author: Option<String>,
  /// Document date (can be string or number)
  #[serde(default)]
  pub date: Option<DateValue>,
  /// Tags/keywords for the document
  #[serde(default)]
  pub tags: Vec<String>,
  /// Document language
  pub language: Option<String>,
  /// Table of contents flag
  pub toc: Option<bool>,
  /// Template selection
  pub template: Option<String>,
  /// Document version
  pub version: Option<String>,
  /// Additional custom fields
  #[serde(flatten)]
  pub extra: HashMap<String, serde_yaml::Value>,
}

/// Represents different date formats that can be in front matter
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DateValue {
  String(String),
  Number(u32),
}

impl fmt::Display for DateValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      DateValue::String(s) => write!(f, "{s}"),
      DateValue::Number(n) => {
        // Try to parse as YYYYMMDD format
        let s = n.to_string();
        if s.len() == 8 {
          write!(f, "{}-{}-{}", &s[0..4], &s[4..6], &s[6..8])
        } else if s.len() == 4 {
          // Just a year
          write!(f, "{s}-01-01")
        } else {
          write!(f, "{s}")
        }
      }
    }
  }
}

/// Parse front matter from markdown content
pub fn parse_frontmatter(content: &str) -> Result<(Option<FrontMatter>, String), Box<dyn std::error::Error>> {
  let content = content.trim_start();

  // Check if content starts with front matter delimiter
  if !content.starts_with("---") {
    return Ok((None, content.to_string()));
  }

  // Find the end of front matter
  let lines: Vec<&str> = content.lines().collect();
  if lines.len() < 2 {
    return Ok((None, content.to_string()));
  }

  // Look for closing --- delimiter
  let mut end_line = None;
  for (i, line) in lines.iter().enumerate().skip(1) {
    if line.trim() == "---" {
      end_line = Some(i);
      break;
    }
  }

  let end_line = match end_line {
    Some(line) => line,
    None => return Ok((None, content.to_string())), // No closing delimiter found
  };

  // Extract front matter YAML
  let yaml_lines = &lines[1..end_line];
  let yaml_content = yaml_lines.join("\n");

  // Parse YAML front matter
  let front_matter = if yaml_content.trim().is_empty() {
    FrontMatter::default()
  } else {
    match serde_yaml::from_str::<FrontMatter>(&yaml_content) {
      Ok(fm) => fm,
      Err(e) => {
        eprintln!("Warning: Failed to parse front matter YAML: {e}");
        FrontMatter::default()
      }
    }
  };

  // Extract remaining markdown content
  let remaining_lines = &lines[end_line + 1..];
  let markdown_content = remaining_lines.join("\n");

  Ok((Some(front_matter), markdown_content))
}

/// Generate Typst variables from front matter
pub fn frontmatter_to_typst_vars(front_matter: &FrontMatter) -> Vec<(String, String)> {
  let mut vars = Vec::new();

  // Add title if present
  if let Some(title) = &front_matter.title {
    vars.push(("fm_title".to_string(), title.clone()));
  }

  // Add subtitle if present
  if let Some(subtitle) = &front_matter.subtitle {
    vars.push(("fm_subtitle".to_string(), subtitle.clone()));
  }

  // Add author if present
  if let Some(author) = &front_matter.author {
    vars.push(("fm_author".to_string(), author.clone()));
  }

  // Add date if present
  if let Some(date) = &front_matter.date {
    vars.push(("fm_date".to_string(), date.to_string()));
  }

  // Add tags as comma-separated string
  if !front_matter.tags.is_empty() {
    let tags_str = front_matter.tags.join(",");
    vars.push(("fm_tags".to_string(), tags_str));
  }

  // Add language if present
  if let Some(language) = &front_matter.language {
    vars.push(("fm_language".to_string(), language.clone()));
  }

  // Add toc flag if present
  if let Some(toc) = &front_matter.toc {
    vars.push(("fm_toc".to_string(), toc.to_string()));
  }

  // Add template if present
  if let Some(template) = &front_matter.template {
    vars.push(("fm_template".to_string(), template.clone()));
  }

  // Add version if present
  if let Some(version) = &front_matter.version {
    vars.push(("fm_version".to_string(), version.clone()));
  }

  // Add all extra/custom fields
  for (key, value) in &front_matter.extra {
    let value_str = match value {
      serde_yaml::Value::String(s) => s.clone(),
      serde_yaml::Value::Number(n) => n.to_string(),
      serde_yaml::Value::Bool(b) => b.to_string(),
      serde_yaml::Value::Sequence(seq) => {
        // Convert array to comma-separated string
        seq
          .iter()
          .map(|v| match v {
            serde_yaml::Value::String(s) => s.clone(),
            serde_yaml::Value::Number(n) => n.to_string(),
            serde_yaml::Value::Bool(b) => b.to_string(),
            _ => format!("{:?}", v),
          })
          .collect::<Vec<_>>()
          .join(",")
      }
      _ => format!("{:?}", value),
    };
    vars.push((key.clone(), value_str));
  }

  // Add boolean flag to indicate front matter is present
  vars.push(("has_frontmatter".to_string(), "true".to_string()));

  vars
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_frontmatter_basic() {
    let content = r#"---
title: "Test Document"
subtitle: "A Subtitle"
author: "John Doe"
date: "2024-01-22"
tags:
  - test
  - example
---

# Main Content

This is the markdown content."#;

    let (front_matter, markdown) = parse_frontmatter(content).unwrap();
    let fm = front_matter.unwrap();

    assert_eq!(fm.title, Some("Test Document".to_string()));
    assert_eq!(fm.subtitle, Some("A Subtitle".to_string()));
    assert_eq!(fm.author, Some("John Doe".to_string()));
    assert_eq!(fm.tags, vec!["test", "example"]);
    assert!(markdown.trim().starts_with("# Main Content"));
  }

  #[test]
  fn test_parse_frontmatter_numeric_date() {
    let content = r#"---
date: 20240122
tags:
  - test
---

Content here"#;

    let (front_matter, _) = parse_frontmatter(content).unwrap();
    let fm = front_matter.unwrap();

    if let Some(DateValue::Number(date)) = fm.date {
      assert_eq!(date, 20240122);
      assert_eq!(DateValue::Number(date).to_string(), "2024-01-22");
    } else {
      panic!("Expected numeric date");
    }
  }

  #[test]
  fn test_no_frontmatter() {
    let content = "# Just a title\n\nNo front matter here.";
    let (front_matter, markdown) = parse_frontmatter(content).unwrap();

    assert!(front_matter.is_none());
    assert_eq!(markdown, content);
  }

  #[test]
  fn test_frontmatter_to_typst_vars() {
    let mut extra = HashMap::new();
    extra.insert("client".to_string(), serde_yaml::Value::String("ACME Corp".to_string()));
    extra.insert("budget".to_string(), serde_yaml::Value::Number(serde_yaml::Number::from(50000)));
    extra.insert("approved".to_string(), serde_yaml::Value::Bool(true));

    let fm = FrontMatter {
      title: Some("Test".to_string()),
      subtitle: Some("Subtitle".to_string()),
      author: Some("Author".to_string()),
      date: Some(DateValue::String("2024-01-22".to_string())),
      tags: vec!["tag1".to_string(), "tag2".to_string()],
      language: Some("en".to_string()),
      toc: Some(true),
      template: Some("elegant".to_string()),
      version: Some("1.0.0".to_string()),
      extra,
    };

    let vars = frontmatter_to_typst_vars(&fm);

    assert!(vars.contains(&("fm_title".to_string(), "Test".to_string())));
    assert!(vars.contains(&("fm_subtitle".to_string(), "Subtitle".to_string())));
    assert!(vars.contains(&("fm_author".to_string(), "Author".to_string())));
    assert!(vars.contains(&("fm_date".to_string(), "2024-01-22".to_string())));
    assert!(vars.contains(&("fm_tags".to_string(), "tag1,tag2".to_string())));
    assert!(vars.contains(&("fm_language".to_string(), "en".to_string())));
    assert!(vars.contains(&("fm_toc".to_string(), "true".to_string())));
    assert!(vars.contains(&("fm_template".to_string(), "elegant".to_string())));
    assert!(vars.contains(&("fm_version".to_string(), "1.0.0".to_string())));
    assert!(vars.contains(&("has_frontmatter".to_string(), "true".to_string())));

    // Test custom fields
    assert!(vars.contains(&("client".to_string(), "ACME Corp".to_string())));
    assert!(vars.contains(&("budget".to_string(), "50000".to_string())));
    assert!(vars.contains(&("approved".to_string(), "true".to_string())));
  }

  #[test]
  fn test_custom_frontmatter_fields() {
    let content = r#"---
title: "Test Document"
client: "ACME Corporation"
contract_number: "CON-2024-001"
budget: 50000
approved: true
reviewers:
  - "Alice Johnson"
  - "Bob Smith"
---

# Content"#;

    let (front_matter, _) = parse_frontmatter(content).unwrap();
    let fm = front_matter.unwrap();
    let vars = frontmatter_to_typst_vars(&fm);

    // Check that custom fields are available
    assert!(vars.contains(&("client".to_string(), "ACME Corporation".to_string())));
    assert!(vars.contains(&("contract_number".to_string(), "CON-2024-001".to_string())));
    assert!(vars.contains(&("budget".to_string(), "50000".to_string())));
    assert!(vars.contains(&("approved".to_string(), "true".to_string())));
    assert!(vars.contains(&("reviewers".to_string(), "Alice Johnson,Bob Smith".to_string())));
  }
}
