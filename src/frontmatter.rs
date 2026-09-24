//! # Front Matter Processing
//!
//! This module handles parsing and processing of YAML front matter in Markdown files.
//! Front matter allows users to specify document metadata, styling preferences, and
//! custom variables that are passed to the Typst templating system for PDF generation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Represents the front matter metadata extracted from a markdown file.
///
/// All fields are optional to allow for flexible document configuration.
///
/// # Examples
///
/// ```
/// use md_pdf::frontmatter::FrontMatter;
/// use std::collections::HashMap;
///
/// let front_matter = FrontMatter {
///     title: Some("User Guide".to_string()),
///     author: Some("Documentation Team".to_string()),
///     language: Some("en".to_string()),
///     toc: Some(true),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct FrontMatter {
  /// A short string identifying the document type (e.g. `report`, `letter`,
  /// `guide`).
  #[serde(default, rename = "type")]
  pub doc_type: Option<String>,
  /// Document title
  pub title: Option<String>,
  /// One-line description. `subtitle` is accepted as an alias.
  #[serde(alias = "subtitle")]
  pub description: Option<String>,
  /// Document author
  pub author: Option<String>,
  /// Document timestamp. `date` is accepted as an alias. Accepts a `YYYY-MM-DD`
  /// string, a `YYYYMMDD` or `YYYY` number, or an ISO 8601 timestamp
  /// (e.g. `2026-09-21T14:30:00Z`, rendered as its date).
  #[serde(default, alias = "date")]
  pub timestamp: Option<DateValue>,
  /// Tags for the document. `keywords` is accepted as an alias.
  #[serde(default, alias = "keywords")]
  pub tags: Vec<String>,
  /// Lifecycle status (e.g. `draft`, `review`, `final`).
  #[serde(default)]
  pub status: Option<String>,
  /// A URI that the document describes.
  #[serde(default)]
  pub resource: Option<String>,
  /// Document language. `lang` is accepted as an alias.
  #[serde(alias = "lang")]
  pub language: Option<String>,
  /// Table of contents flag
  pub toc: Option<bool>,
  /// Template selection
  pub template: Option<String>,
  /// Document version
  pub version: Option<String>,
  /// Additional custom fields. Any other key — `sources`, `generated`,
  /// `verified`, `stale_after`, the letter `recipient_*`/`sender_*` fields,
  /// etc. — is preserved and passed through to the template unchanged.
  #[serde(flatten)]
  pub extra: HashMap<String, serde_yaml::Value>,
}

/// Represents different date formats supported in front matter.
///
/// Supports both string dates and numeric dates (YYYYMMDD format).
///
/// # Examples
///
/// ```
/// use md_pdf::frontmatter::DateValue;
///
/// // String date (used as-is)
/// let string_date = DateValue::String("2024-01-23".to_string());
/// assert_eq!(string_date.to_string(), "2024-01-23");
///
/// // Numeric date (formatted automatically)
/// let numeric_date = DateValue::Number(20240123);
/// assert_eq!(numeric_date.to_string(), "2024-01-23");
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DateValue {
  String(String),
  Number(u32),
}

impl DateValue {
  /// Date-only rendering (`YYYY-MM-DD` where possible), suitable for display.
  ///
  /// String values keep only the part before an ISO 8601 `T` separator, so a
  /// timestamp such as `2026-09-21T14:30:00Z` displays as `2026-09-21`.
  /// Numeric values are formatted as `YYYY-MM-DD` (from `YYYYMMDD`) or
  /// `YYYY-01-01` (from a bare year).
  pub fn to_date_string(&self) -> String {
    match self {
      DateValue::String(s) => s.split('T').next().unwrap_or(s).to_string(),
      DateValue::Number(n) => {
        let s = n.to_string();
        if s.len() == 8 {
          format!("{}-{}-{}", &s[0..4], &s[4..6], &s[6..8])
        } else if s.len() == 4 {
          format!("{s}-01-01")
        } else {
          s
        }
      }
    }
  }
}

impl fmt::Display for DateValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // Display keeps the date-only form for backwards compatibility.
    write!(f, "{}", self.to_date_string())
  }
}

/// Parse YAML front matter from markdown content.
///
/// Extracts YAML metadata from the beginning of a markdown file and
/// separates it from the main content. Front matter must be enclosed in `---`
/// delimiters at the very start of the file.
///
/// # Arguments
///
/// * `content` - The complete markdown file content as a string
///
/// # Returns
///
/// Returns a tuple containing:
/// - `Option<FrontMatter>` - Parsed front matter metadata, or `None` if not present
/// - `String` - The remaining markdown content without front matter
///
/// # Examples
///
/// ```
/// use md_pdf::frontmatter::parse_frontmatter;
///
/// let content = r#"---
/// title: "My Document"
/// author: "John Doe"
/// ---
/// # Main Content
/// This is the document body."#;
///
/// let (front_matter, markdown) = parse_frontmatter(content).unwrap();
/// assert!(front_matter.is_some());
/// assert!(markdown.trim().starts_with("# Main Content"));
/// ```
///
/// # Errors
///
/// Returns an error only for severe parsing failures. Invalid YAML in front matter
/// generates warnings but doesn't prevent processing of the markdown content.
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

/// Convert front matter into Typst input variables.
///
/// Transforms a `FrontMatter` structure into a vector of key-value pairs
/// that can be passed as input variables to the Typst typesetting system.
///
/// # Arguments
///
/// * `front_matter` - The parsed front matter structure to convert
///
/// # Returns
///
/// A vector of tuples containing variable names and their string values, ready
/// for use as Typst `--input` arguments.
///
/// # Examples
///
/// ```
/// use md_pdf::frontmatter::{FrontMatter, frontmatter_to_typst_vars, DateValue};
/// use std::collections::HashMap;
///
/// let fm = FrontMatter {
///     title: Some("Test Doc".to_string()),
///     author: Some("Jane Doe".to_string()),
///     date: Some(DateValue::String("2024-01-23".to_string())),
///     tags: vec!["tech".to_string(), "docs".to_string()],
///     ..Default::default()
/// };
///
/// let vars = frontmatter_to_typst_vars(&fm);
/// // Results in variables like:
/// // ("fm_title", "Test Doc")
/// // ("fm_author", "Jane Doe")
/// // ("fm_tags", "tech,docs")
/// // ("has_frontmatter", "true")
/// ```
pub fn frontmatter_to_typst_vars(front_matter: &FrontMatter) -> Vec<(String, String)> {
  let mut vars = Vec::new();

  // Document type
  if let Some(doc_type) = &front_matter.doc_type {
    vars.push(("fm_type".to_string(), doc_type.clone()));
  }

  // Add title if present
  if let Some(title) = &front_matter.title {
    vars.push(("fm_title".to_string(), title.clone()));
  }

  // Add description if present
  if let Some(description) = &front_matter.description {
    vars.push(("fm_description".to_string(), description.clone()));
  }

  // Add author if present
  if let Some(author) = &front_matter.author {
    vars.push(("fm_author".to_string(), author.clone()));
  }

  // Add timestamp if present, rendered as a date for display.
  if let Some(timestamp) = &front_matter.timestamp {
    vars.push(("fm_timestamp".to_string(), timestamp.to_date_string()));
  }

  // Add tags as comma-separated string
  if !front_matter.tags.is_empty() {
    vars.push(("fm_tags".to_string(), front_matter.tags.join(",")));
  }

  // Lifecycle status
  if let Some(status) = &front_matter.status {
    vars.push(("fm_status".to_string(), status.clone()));
  }

  // Described resource URI
  if let Some(resource) = &front_matter.resource {
    vars.push(("fm_resource".to_string(), resource.clone()));
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
            _ => format!("{v:?}"),
          })
          .collect::<Vec<_>>()
          .join(",")
      }
      _ => format!("{value:?}"),
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
    assert_eq!(fm.description, Some("A Subtitle".to_string())); // `subtitle` alias
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

    if let Some(DateValue::Number(date)) = fm.timestamp {
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
      description: Some("Subtitle".to_string()),
      author: Some("Author".to_string()),
      timestamp: Some(DateValue::String("2024-01-22".to_string())),
      tags: vec!["tag1".to_string(), "tag2".to_string()],
      language: Some("en".to_string()),
      toc: Some(true),
      template: Some("elegant".to_string()),
      version: Some("1.0.0".to_string()),
      extra,
      ..Default::default()
    };

    let vars = frontmatter_to_typst_vars(&fm);

    assert!(vars.contains(&("fm_title".to_string(), "Test".to_string())));
    assert!(vars.contains(&("fm_description".to_string(), "Subtitle".to_string())));
    assert!(vars.contains(&("fm_author".to_string(), "Author".to_string())));
    assert!(vars.contains(&("fm_timestamp".to_string(), "2024-01-22".to_string())));
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
  fn test_legacy_aliases_parse() {
    // Legacy key spellings should map onto the canonical fields.
    let content = r#"---
type: report
title: "Doc"
description: "One-line summary"
timestamp: "2026-09-21T14:30:00Z"
keywords: ["alpha", "beta"]
lang: "de"
status: draft
resource: "https://example.com/asset"
---
# Body"#;

    let (fm, _) = parse_frontmatter(content).unwrap();
    let fm = fm.unwrap();

    assert_eq!(fm.doc_type.as_deref(), Some("report"));
    assert_eq!(fm.description.as_deref(), Some("One-line summary"));
    assert_eq!(fm.tags, vec!["alpha".to_string(), "beta".to_string()]); // keywords -> tags
    assert_eq!(fm.language.as_deref(), Some("de")); // lang -> language
    assert_eq!(fm.status.as_deref(), Some("draft"));
    assert_eq!(fm.resource.as_deref(), Some("https://example.com/asset"));
    assert_eq!(fm.timestamp.as_ref().map(|d| d.to_date_string()), Some("2026-09-21".to_string()));
    // No known/aliased key should leak into `extra`
    for k in ["type", "description", "subtitle", "timestamp", "date", "keywords", "lang", "status", "resource"] {
      assert!(!fm.extra.contains_key(k), "{k} should not be in extra");
    }
  }

  #[test]
  fn test_subtitle_and_date_aliases() {
    // The legacy `subtitle` and `date` keys resolve to description/timestamp.
    let content = r#"---
subtitle: "Legacy subtitle"
date: "2024-01-22"
---
# Body"#;
    let (fm, _) = parse_frontmatter(content).unwrap();
    let fm = fm.unwrap();
    assert_eq!(fm.description.as_deref(), Some("Legacy subtitle"));
    assert_eq!(fm.timestamp.as_ref().map(|d| d.to_date_string()), Some("2024-01-22".to_string()));
  }

  #[test]
  fn test_date_display_forms() {
    // ISO 8601 timestamp renders as a date; numeric styles are formatted.
    let iso = DateValue::String("2026-09-21T14:30:00Z".to_string());
    assert_eq!(iso.to_date_string(), "2026-09-21");
    assert_eq!(DateValue::Number(20260321).to_date_string(), "2026-03-21");
    assert_eq!(DateValue::Number(2026).to_date_string(), "2026-01-01");
  }

  #[test]
  fn test_vars_emitted_canonical_names() {
    let content = r#"---
title: "Names"
description: "Desc"
timestamp: "2026-09-21T14:30:00Z"
keywords: ["x", "y"]
lang: "fr"
type: guide
status: final
---
# Body"#;
    let (fm, _) = parse_frontmatter(content).unwrap();
    let vars = frontmatter_to_typst_vars(&fm.unwrap());

    assert!(vars.contains(&("fm_type".to_string(), "guide".to_string())));
    assert!(vars.contains(&("fm_description".to_string(), "Desc".to_string())));
    assert!(vars.contains(&("fm_timestamp".to_string(), "2026-09-21".to_string())));
    assert!(vars.contains(&("fm_tags".to_string(), "x,y".to_string())));
    assert!(vars.contains(&("fm_language".to_string(), "fr".to_string())));
    assert!(vars.contains(&("fm_status".to_string(), "final".to_string())));
    // Legacy variable names are no longer emitted.
    assert!(!vars.iter().any(|(k, _)| k == "fm_subtitle" || k == "fm_date" || k == "keywords" || k == "lang"));
  }
}
