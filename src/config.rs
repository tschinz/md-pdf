use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Type alias for template information: (name, description, is_default)
pub type TemplateInfo = Vec<(String, String, bool)>;

/// Configuration structure for md-pdf
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
  /// Directory containing template files
  pub templates_dir: Option<PathBuf>,
  /// Default template to use
  pub default_template: Option<String>,
  /// Default language
  pub default_language: Option<String>,
  /// Default TOC setting
  pub default_toc: Option<bool>,
  /// Default author
  pub default_author: Option<String>,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      templates_dir: None,
      default_template: Some("simple".to_string()),
      default_language: Some("en".to_string()),
      default_toc: Some(true),
      default_author: Some("ZaS".to_string()),
    }
  }
}

impl Config {
  /// Load configuration from file system with priority order
  pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
    let config_locations = Self::get_config_locations()?;

    for location in config_locations {
      if location.exists() {
        println!(
          "Loading configuration from: {}",
          location.canonicalize().unwrap_or_else(|_| location.to_path_buf()).display()
        );
        let content = fs::read_to_string(&location)?;
        let config: Config = ron::from_str(&content)?;
        return Ok(config);
      }
    }

    // No config file found, create default at ~/.config/md-pdf/config.ron
    if let Some(home_dir) = dirs::home_dir() {
      let default_config_path = home_dir.join(".config").join("md-pdf").join("config.ron");

      // Create the default config file
      match Self::create_default_config(&default_config_path) {
        Ok(_) => {
          println!(
            "No configuration file found, created default at: {}",
            default_config_path
              .canonicalize()
              .unwrap_or_else(|_| default_config_path.to_path_buf())
              .display()
          );
          // Load the newly created config
          let content = fs::read_to_string(&default_config_path)?;
          let config: Config = ron::from_str(&content)?;
          return Ok(config);
        }
        Err(e) => {
          println!("Warning: Could not create default config file: {e}");
          println!("Using built-in defaults");
        }
      }
    } else {
      println!("Warning: Could not determine home directory for config file");
      println!("Using built-in defaults");
    }
    Ok(Config::default())
  }

  /// Get list of configuration file locations in priority order
  fn get_config_locations() -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut locations = Vec::new();

    // 1. Current execution directory as md-pdf.ron
    if let Ok(current_dir) = env::current_dir() {
      locations.push(current_dir.join("md-pdf.ron"));
    }

    // 2. Binary location as md-pdf.ron
    if let Ok(exe_path) = env::current_exe() {
      if let Some(exe_dir) = exe_path.parent() {
        locations.push(exe_dir.join("md-pdf.ron"));
      }
    }

    // 3-6. Home directory variants
    if let Some(home_dir) = dirs::home_dir() {
      // 3. ~/ directory as md-pdf.ron
      locations.push(home_dir.join("md-pdf.ron"));

      // 4. ~/.config directory as md-pdf.ron
      locations.push(home_dir.join(".config").join("md-pdf.ron"));

      // 5. ~/.config/md-pdf directory as config.ron
      locations.push(home_dir.join(".config").join("md-pdf").join("config.ron"));

      // 6. ~/.config/zas directory as md-pdf.ron
      locations.push(home_dir.join(".config").join("zas").join("md-pdf.ron"));
    }

    Ok(locations)
  }

  /// Get the templates directory, with fallback to bundled templates
  pub fn get_templates_dir(&self) -> PathBuf {
    if let Some(templates_dir) = &self.templates_dir {
      if templates_dir.exists() {
        return templates_dir.clone();
      }
    }

    // Fallback: check for templates directory relative to current directory
    let current_templates = PathBuf::from("templates");
    if current_templates.exists() {
      return current_templates;
    }

    // Fallback: check for templates directory relative to binary
    if let Ok(exe_path) = env::current_exe() {
      if let Some(exe_dir) = exe_path.parent() {
        let binary_templates = exe_dir.join("templates");
        if binary_templates.exists() {
          return binary_templates;
        }
      }
    }

    // Final fallback: current directory
    PathBuf::from(".")
  }

  /// List available templates from the templates directory
  pub fn list_available_templates(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let templates_dir = self.get_templates_dir();
    let mut templates = Vec::new();

    // Scan templates directory for .typ files
    if templates_dir.exists() && templates_dir.is_dir() {
      for entry in fs::read_dir(&templates_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
          if let Some(extension) = path.extension() {
            if extension == "typ" {
              if let Some(name) = path.file_stem() {
                if let Some(name_str) = name.to_str() {
                  templates.push(name_str.to_string());
                }
              }
            }
          }
        }
      }
    }

    // Add built-in templates if not found in directory
    if !templates.contains(&"none".to_string()) {
      templates.push("none".to_string());
    }
    if !templates.contains(&"simple".to_string()) {
      templates.push("simple".to_string());
    }

    // Sort with default template first
    if let Some(default_template) = &self.default_template {
      templates.sort_by(|a, b| {
        if a == default_template {
          std::cmp::Ordering::Less
        } else if b == default_template {
          std::cmp::Ordering::Greater
        } else {
          a.cmp(b)
        }
      });
    } else {
      templates.sort();
    }

    Ok(templates)
  }

  /// Get detailed information about available templates
  pub fn get_template_info(&self) -> Result<TemplateInfo, Box<dyn std::error::Error>> {
    let templates_dir = self.get_templates_dir();
    let mut template_info = Vec::new();
    //let default_template = self.default_template.as_ref().map(|s| s.as_str());
    let default_template = self.default_template.as_deref();

    // Scan templates directory for .typ files
    if templates_dir.exists() && templates_dir.is_dir() {
      for entry in fs::read_dir(&templates_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
          if let Some(extension) = path.extension() {
            if extension == "typ" {
              if let Some(name) = path.file_stem() {
                if let Some(name_str) = name.to_str() {
                  // Try to read first few lines for description comment
                  let description = match fs::read_to_string(&path) {
                    Ok(content) => {
                      let lines: Vec<&str> = content.lines().take(10).collect();
                      let mut found_description = None;
                      for line in lines {
                        let trimmed = line.trim();
                        if trimmed.starts_with("//") && trimmed.len() > 5 {
                          let comment = trimmed[2..].trim();
                          if !comment.is_empty()
                            && (comment.contains("template")
                              || comment.contains("Template")
                              || comment.contains("description")
                              || comment.contains("Description"))
                          {
                            found_description = Some(comment.to_string());
                            break;
                          }
                        }
                      }
                      found_description.unwrap_or_else(|| format!("Template file: {}", path.canonicalize().unwrap_or_else(|_| path.to_path_buf()).display()))
                    }
                    Err(_) => format!("Template file: {}", path.canonicalize().unwrap_or_else(|_| path.to_path_buf()).display()),
                  };

                  let is_default = Some(name_str) == default_template;
                  template_info.push((name_str.to_string(), description, is_default));
                }
              }
            }
          }
        }
      }
    }

    // Add built-in templates that aren't in the directory
    let has_none = template_info.iter().any(|(name, _, _)| name == "none");
    let has_simple = template_info.iter().any(|(name, _, _)| name == "simple");

    if !has_none {
      let is_default = Some("none") == default_template;
      template_info.push(("none".to_string(), "Built-in basic template with minimal styling".to_string(), is_default));
    }

    if !has_simple {
      let is_default = Some("simple") == default_template;
      template_info.push((
        "simple".to_string(),
        "Built-in professional template with headers and footers".to_string(),
        is_default,
      ));
    }

    // Sort by name, but put default first
    template_info.sort_by(|a, b| {
      match (a.2, b.2) {
        (true, false) => std::cmp::Ordering::Less,    // default comes first
        (false, true) => std::cmp::Ordering::Greater, // default comes first
        _ => a.0.cmp(&b.0),                           // alphabetical for non-defaults
      }
    });

    Ok(template_info)
  }

  /// Get template content by name
  pub fn get_template_content(&self, template_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let templates_dir = self.get_templates_dir();
    let template_path = templates_dir.join(format!("{template_name}.typ"));

    if template_path.exists() {
      let content = fs::read_to_string(&template_path)?;
      Ok(content)
    } else {
      // Fallback to built-in templates only for none and simple
      match template_name {
        "none" => Ok(Self::get_fallback_none_template()),
        "simple" => Ok(Self::get_fallback_simple_template()),
        _ => Err(
          format!(
            "Template '{}' not found in templates directory: {}",
            template_name,
            templates_dir.canonicalize().unwrap_or_else(|_| templates_dir.to_path_buf()).display()
          )
          .into(),
        ),
      }
    }
  }

  /// Create a default configuration file
  pub fn create_default_config(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create config with proper default templates directory
    let mut default_config = Config::default();

    // Set templates directory to be alongside the config file
    if let Some(config_parent) = path.parent() {
      let templates_path = config_parent.join("templates");
      default_config.templates_dir = Some(templates_path);
    }
    let config_content = format!(
      r#"// md-pdf configuration file
// This file uses RON (Rust Object Notation) format
//
// Configuration file locations (in priority order):
//   1. ./md-pdf.ron (project-specific)
//   2. <binary-dir>/md-pdf.ron (installation directory)
//   3. ~/md-pdf.ron (user home)
//   4. ~/.config/md-pdf.ron (standard config)
//   5. ~/.config/md-pdf/config.ron (this file)
//   6. ~/.config/zas/md-pdf.ron (workflow integration)

{}
"#,
      ron::ser::to_string_pretty(&default_config, ron::ser::PrettyConfig::default())?
    );

    // Create parent directory if it doesn't exist
    if let Some(parent) = path.parent() {
      fs::create_dir_all(parent)?;
    }

    // Create templates directory and basic template files
    if let Some(templates_dir) = &default_config.templates_dir {
      fs::create_dir_all(templates_dir)?;

      // Create basic none.typ template file
      let none_template_path = templates_dir.join("none.typ");
      if !none_template_path.exists() {
        fs::write(&none_template_path, Self::get_fallback_none_template())?;
      }

      // Create basic simple.typ template file
      let simple_template_path = templates_dir.join("simple.typ");
      if !simple_template_path.exists() {
        fs::write(&simple_template_path, Self::get_fallback_simple_template())?;
      }
    }

    fs::write(path, config_content)?;
    println!(
      "Created default configuration at: {}",
      path.canonicalize().unwrap_or_else(|_| path.to_path_buf()).display()
    );
    Ok(())
  }

  /// Fallback none template (minimal)
  fn get_fallback_none_template() -> String {
    r#"
          #import "@preview/cmarker:0.1.7"
          #import "@preview/mitex:0.2.6": mitex

          // Get system inputs
          #let filepath = sys.inputs.at("filepath", default: "input.md")
          #let language = sys.inputs.at("language", default: "en")
          #let show-toc = sys.inputs.at("toc", default: "false") == "true"

          // Front matter inputs
          #let has-frontmatter = sys.inputs.at("has_frontmatter", default: "false") == "true"
          #let fm-title = sys.inputs.at("fm_title", default: none)
          #let fm-subtitle = sys.inputs.at("fm_subtitle", default: none)
          #let fm-author = sys.inputs.at("fm_author", default: none)
          #let fm-date = sys.inputs.at("fm_date", default: none)
          #let fm-tags = sys.inputs.at("fm_tags", default: none)
          #let fm_version = sys.inputs.at("fm_version", default: none)

          // Parse tags from comma-separated string
          #let tags-list = if fm-tags != none { fm-tags.split(",") } else { () }

          // Extract filename from filepath (remove path and .md extension)
          #let filename = {
            let path-parts = filepath.split("/")
            let file = path-parts.last()
            if file.ends-with(".md") {
              file.slice(0, file.len() - 3)
            } else if file.ends-with(".temp.md") {
              file.slice(0, file.len() - 8)
            } else {
              file
            }
          }

          // Use front matter data or defaults
          #let document-author = if fm-author != none { fm-author } else { default_author }
          #let document-title = if fm-title != none { fm-title } else { filename }
          #let document-subtitle = if fm-subtitle != none { fm-subtitle } else { filename }

          // Parse date
          #let document-date = if fm-date != none {
            // Try to parse the date string
            let date-str = fm-date
            if date-str.len() == 10 and date-str.contains("-") {
              // Format: YYYY-MM-DD
              let parts = date-str.split("-")
              if parts.len() == 3 {
                datetime(year: int(parts.at(0)), month: int(parts.at(1)), day: int(parts.at(2)))
              } else {
                datetime.today()
              }
            } else {
              datetime.today()
            }
          } else {
            datetime.today()
          }

          // Set document properties
          #set document(
            author: if document-author != none { document-author } else { "" },
            title: document-title,
            keywords: if fm-tags != none { (if document-author != none { document-author } else { "" }, document-title, "md-pdf", ..tags-list) } else { (if document-author != none { document-author } else { "" }, document-title, "md-pdf") },
            date: document-date
          )

          // Set document language
          #set text(lang: language)

          // Function to create tag labels
          #let badge(content) = {
            let color = rgb("888888")
            let textcolor = rgb("222222")
            box(
              inset: (x: 3pt, y: 2pt),
              radius: 4pt,
              fill: color.lighten(70%),
              stroke: (paint: color, thickness: 0.5pt),
            )[
              #text(weight: "bold", size: 6pt, fill:textcolor)[#content]
          ]
          }

          // Show basic document metadata if front matter exists
          #if has-frontmatter [
            #if fm-title != none [
              #align(center)[
                #text(size: 18pt, weight: "bold")[#fm-title]
              ]
              #v(0.3em)
            ]
            #if fm-subtitle != none [
              #align(center)[
                #text(size: 14pt, style: "italic")[#fm-subtitle]
              ]
              #v(0.3em)
            ]
            #let metadata = ()
            #if document-author != none { metadata.push(document-author) }
            #if document-date != none { metadata.push(document-date.display()) }
            #if fm_version != none { metadata.push(fm_version) }
            #align(center)[
              #for (i, data) in metadata.enumerate() [
                #data
                #if i < metadata.len() - 1 [ \- ]
              ]
            ]

            #if fm-tags != none and tags-list.len() > 0 [
              #align(center)[
                #for (i, tag) in tags-list.enumerate() [
                  #badge(tag.trim())
                ]
              ]
            ]
            #line(length: 100%, stroke: 0.5pt)
          ]

          // Show table of contents if requested
          #if show-toc [
            #outline()
            #pagebreak()
          ]

          #cmarker.render(
            read(filepath),
            scope: (image: (path, alt: none) => image(path, alt: alt)),
            math: mitex
          )
"#.to_string()
  }

  /// Fallback simple template
  fn get_fallback_simple_template() -> String {
    r#"#import "@preview/cmarker:0.1.8"
        #import "@preview/mitex:0.2.6": mitex
        #import "@preview/hei-synd-thesis:0.2.3": *

        // Get system inputs
        #let filepath = sys.inputs.at("filepath", default: "input.md")
        #let language = sys.inputs.at("language", default: "en")
        #let show-toc = sys.inputs.at("toc", default: "false") == "true"

        // Front matter inputs
        #let has-frontmatter = sys.inputs.at("has_frontmatter", default: "false") == "true"
        #let fm-title = sys.inputs.at("fm_title", default: none)
        #let fm-subtitle = sys.inputs.at("fm_subtitle", default: none)
        #let fm-author = sys.inputs.at("fm_author", default: none)
        #let fm-date = sys.inputs.at("fm_date", default: none)
        #let fm-tags = sys.inputs.at("fm_tags", default: none)
        #let fm_version = sys.inputs.at("fm_version", default: none)

        // Parse tags from comma-separated string
        #let tags-list = if fm-tags != none { fm-tags.split(",") } else { () }

        // Extract filename from filepath (remove path and .md extension)
        #let filename = {
          let path-parts = filepath.split("/")
          let file = path-parts.last()
          if file.ends-with(".md") {
            file.slice(0, file.len() - 3)
          } else if file.ends-with(".temp.md") {
            file.slice(0, file.len() - 8)
          } else {
            file
          }
        }

        // Use front matter data or defaults
        #let document-author = if fm-author != none { fm-author } else { none }
        #let document-title = if fm-title != none { fm-title } else { filename }
        #let document-subtitle = if fm-subtitle != none { fm-subtitle } else { none }

        // Parse date
        #let document-date = if fm-date != none {
          // Try to parse the date string
          let date-str = fm-date
          if date-str.len() == 10 and date-str.contains("-") {
            // Format: YYYY-MM-DD
            let parts = date-str.split("-")
            if parts.len() == 3 {
              datetime(year: int(parts.at(0)), month: int(parts.at(1)), day: int(parts.at(2)))
            } else {
              datetime.today()
            }
          } else {
            datetime.today()
          }
        } else {
          datetime.today()
        }

        // Set document properties
        #set document(
          author: if document-author != none { document-author } else { "" },
          title: document-title,
          keywords: if fm-tags != none { (if document-author != none { document-author } else { "" }, document-title, "md-pdf", ..tags-list) } else { (document-author, document-title, "md-pdf") },
          date: document-date
        )

        // basic properties
        #set page(margin: (top:3cm, bottom:3cm, left:3cm, right:2.5cm))

        // header and footer
        #set page(
          header: context(if here().page() >=2 [
            #set text(small)
            #smallcaps[#document-title] #if document-subtitle != none {[\/ #document-subtitle ]}
            //#line(start: (-0.5em, 0cm), length: 85%, stroke: 0.5pt)
            #line(start: (-0.5em, 0cm), length: 101%, stroke: 0.5pt)
            ]),
          footer: context( if here().page() >=2 [
              #set text(small)
              #line(start: (85%, 0cm), length: 15%, stroke: 0.5pt)

              #document-author / #document-date.display() #h(1fr) #context counter(page).display("1 / 1", both: true)
          ]),
        )

        // font & language
        #set text(
          font: (
            "Libertinus Serif",
            "Fira Sans",
          ),
          fallback: true,
          lang: language
        )

        // heading
        #show heading: set block(above: 1.2em, below: 1.2em)
        #set heading(numbering: "1.1")

        #show heading.where(level: 1): (it) => {
          set text(size: larger-p )
          set block(above: 1.2em, below: 1.2em)
          if it.numbering != none {
            let num = numbering(it.numbering, ..counter(heading).at(it.location()))
            let prefix = num + h(0.5em) + text(code-border)[|] + h(0.5em)
            unshift-prefix(prefix, it.body)
          } else {
            it
          }
        }

        #show heading.where(level: 2): (it) => {
          if it.numbering != none {
            let num = numbering(it.numbering, ..counter(heading).at(it.location()))
            unshift-prefix(num + h(0.8em), it.body)
          }
        }

        // link color
        #show link: it => text(fill:hei-blue, it)

        // code blocks
        #show raw: set text(
          font: (
          "Iosevka",
          "Fira Code",
          "JetBrains Mono",
          "DejaVu Sans Mono",
          ),
        fallback: true,)
        #show raw.where(block: false): set text(weight: "semibold")
        #show raw.where(block: true): set text(size: tiny)
        #show raw.where(block: true): it => {
          block(
            fill: code-bg,
            width:100%,
            inset: 7pt,
            radius: (left:0pt, right: 4pt),
            stroke: (left: 3pt + luma(80%), rest: 0.1pt + code-border),
            it,
          )
        }
        #show: codly-init.with()
        #codly(
          display-icon: false,
          languages: codly-languages,
          zebra-fill: none,
          stroke: 0.1pt + code-border,
          radius: 4pt,
          number-format: (number) => text(luma(210), size:7pt, [#h(1em)#number]),
          inset: (left:-0.0em, rest:0.3em),
          fill: code-bg,
        )

        // Captions
        #set figure(numbering: "1", supplement: get-supplement)
        #set figure.caption(separator: " - ") // With a nice separator
        #set math.equation(numbering: "(1)", supplement: i18n("equation-name"))

        #show: word-count

        // Function to create tag labels
        #let badge(content) = {
          let color = rgb("888888")
          let textcolor = rgb("222222")
          box(
            inset: (x: 3pt, y: 2pt),
            radius: 4pt,
            fill: color.lighten(70%),
            stroke: (paint: color, thickness: 0.5pt),
          )[
            #text(weight: "bold", size: 8pt, fill:textcolor)[#content]
        ]
        }

        // Show basic document metadata if front matter exists
        #if has-frontmatter [
          #if fm-title != none [
            #align(center)[
              #text(size: 18pt, weight: "bold")[#fm-title]
            ]
            #v(0.3em)
          ]
          #if fm-subtitle != none [
            #align(center)[
              #text(size: 14pt, style: "italic")[#fm-subtitle]
            ]
            #v(0.3em)
          ]
          #let metadata = ()
          #if document-author != none { metadata.push(document-author) }
          #if document-date != none { metadata.push(document-date.display()) }
          #if fm_version != none { metadata.push(fm_version) }
          #align(center)[
            #for (i, data) in metadata.enumerate() [
              #data
              #if i < metadata.len() - 1 [ \- ]
            ]
          ]

          #if fm-tags != none and tags-list.len() > 0 [
            #align(center)[
              #for (i, tag) in tags-list.enumerate() [
                #badge(tag.trim())
              ]
            ]
          ]
          #line(length: 100%, stroke: 0.5pt)
        ]

        // Show table of contents if requested
        #if show-toc [
          #outline()
          #pagebreak()
        ]

        #cmarker.render(
          read(filepath),
          scope: (image: (path, alt: none) => image(path, alt: alt)),
          math: mitex
        )
"#.to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;
  use tempfile::TempDir;

  #[test]
  fn test_default_config() {
    let config = Config::default();
    assert_eq!(config.default_template, Some("simple".to_string()));
    assert_eq!(config.default_language, Some("en".to_string()));
    assert_eq!(config.default_toc, Some(true));
    assert!(config.templates_dir.is_none());
  }

  #[test]
  fn test_config_serialization() {
    let config = Config::default();
    let serialized = ron::ser::to_string(&config).unwrap();
    let deserialized: Config = ron::from_str(&serialized).unwrap();

    assert_eq!(config.default_template, deserialized.default_template);
    assert_eq!(config.default_language, deserialized.default_language);
  }

  #[test]
  fn test_create_config_file() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test-config.ron");

    Config::create_default_config(&config_path).unwrap();
    assert!(config_path.exists());

    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("templates_dir"));
    assert!(content.contains("default_template"));
  }

  #[test]
  fn test_fallback_templates() {
    let config = Config::default();

    let none_template = config.get_template_content("none").unwrap();
    assert!(none_template.contains("#import \"@preview/cmarker:0.1.7\""));

    let simple_template = config.get_template_content("simple").unwrap();
    assert!(simple_template.contains("badge(tag.trim())"));
  }
}
