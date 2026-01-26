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
    let has_playful = template_info.iter().any(|(name, _, _)| name == "playful");
    let has_brutalist = template_info.iter().any(|(name, _, _)| name == "brutalist");

    if !has_none {
      let is_default = Some("none") == default_template;
      template_info.push(("none".to_string(), "Built-in basic template with minimal styling".to_string(), is_default));
    }

    if !has_simple {
      let is_default = Some("simple") == default_template;
      template_info.push((
        "simple".to_string(),
        "Built-in simple template with headers and footers".to_string(),
        is_default,
      ));
    }

    if !has_playful {
      let is_default = Some("playful") == default_template;
      template_info.push(("playful".to_string(), "Built-in playful template, colorful and clean".to_string(), is_default));
    }

    if !has_brutalist {
      let is_default = Some("brutalist") == default_template;
      template_info.push((
        "brutalist".to_string(),
        "Built-in brutalist template, raw, geometrical emphasis on function".to_string(),
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
        "brutalist" => Ok(Self::get_fallback_brutalist_template()),
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

      // Create basic playful.typ template file
      let playful_template_path = templates_dir.join("playful.typ");
      if !playful_template_path.exists() {
        fs::write(&playful_template_path, Self::get_fallback_playful_template())?;
      }

      // Create basic brutalist.typ template file
      let brutalist_template_path = templates_dir.join("brutalist.typ");
      if !brutalist_template_path.exists() {
        fs::write(&brutalist_template_path, Self::get_fallback_brutalist_template())?;
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
    r#"// None Template
#import "@preview/cmarker:0.1.8"
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
#let fm-version = sys.inputs.at("fm_version", default: none)

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
  author: if document-author != none {document-author} else {""},
  title: document-title,
  keywords: if fm-tags != none { (if document-author != none {document-author} else {""}, if document-title != none {document-title} else {""}, "md-pdf", ..tags-list) } else { (if document-author != none {document-author} else {""}, if document-title != none {document-title} else {""}, "md-pdf") },
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
  #if fm-version != none { metadata.push(fm-version) }
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
    r#"// Simple Template
#import "@preview/cmarker:0.1.8"
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
#let fm-version = sys.inputs.at("fm_version", default: none)

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
  author: if document-author != none {document-author} else {""},
  title: document-title,
  keywords: if fm-tags != none { (if document-author != none {document-author} else {""}, if document-title != none {document-title} else {""}, "md-pdf", ..tags-list) } else { (if document-author != none {document-author} else {""}, if document-title != none {document-title} else {""}, "md-pdf") },
  date: document-date
)

// basic properties
#set page(margin: (top:3cm, bottom:3cm, left:3cm, right:2.5cm))
// header and footer
#set page(
  header: context(if here().page() >=2 [
    #smallcaps[#document-title]
    ]),
  footer: context( if here().page() >=2 [
      #set text(10pt)
      #if document-author != none {document-author} #h(1fr) #document-date.display() #h(1fr) #context counter(page).display("1 / 1", both: true)
  ]),
)

// font & language
#set text(
  font: (
    "Libertinus Serif",
  ),
  fallback: true,
  lang: language
)

// heading
#show heading: set block(above: 1.2em, below: 1.2em)
#set heading(numbering: "1.1")

// link color
#show link: it => text(fill:blue, it)

// code blocks
#show raw: set text(
  font: (
  "DejaVu Sans Mono",
  ),
fallback: true,)
#show raw.where(block: false): set text(weight: "semibold")
#show raw.where(block: true): set text(size: 8pt)

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
  #if fm-version != none { metadata.push(fm-version) }
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

  /// Fallback brutalist template (raw, bold, stark design)
  fn get_fallback_playful_template() -> String {
    r#"// Playful Template
#import "@preview/cmarker:0.1.8"
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
#let fm-version = sys.inputs.at("fm_version", default: none)

// Dieter Rams inspired color palette
#let rams-white = rgb("f7f8f6ff")
#let rams-light-grey = rgb("d9d2c6ff")
#let rams-dark-grey = rgb("4a4a4aff")
#let rams-black = rgb("1f1f1fff")
#let rams-green = rgb("736b1eff")
#let rams-brown = rgb("8b7355ff")
#let rams-red = rgb("ed3f1cff")
#let rams-orange = rgb("ed8008ff")

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
  let date-str = fm-date
  if date-str.len() == 10 and date-str.contains("-") {
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
  author: if document-author != none {document-author} else {""},
  title: document-title,
  keywords: if fm-tags != none { (if document-author != none {document-author} else {""}, if document-title != none {document-title} else {""}, "md-pdf", ..tags-list) } else { (if document-author != none {document-author} else {""}, if document-title != none {document-title} else {""}, "md-pdf") },
  date: document-date
)

// Clean page layout with generous margins - Rams principle of "as little design as possible"
#set page(
  margin: (top: 3.5cm, bottom: 3cm, left: 3cm, right: 3cm),
  fill: rams-white
)

// Minimal header and footer - functional but unobtrusive
#set page(
  header: context(if here().page() >= 2 [
    #set text(9pt, fill: rams-dark-grey)
    #document-title
    #line(length: 100%, stroke: (paint: rams-light-grey, thickness: 0.5pt))
    #h(1fr)
  ]),
  footer: context(if here().page() >= 2 [
    #line(length: 100%, stroke: (paint: rams-light-grey, thickness: 0.5pt))
    #set text(8pt, fill: rams-dark-grey)
    #v(0.2em)
    #if document-author != none {[#document-author #h(1fr)]} #document-date.display() #h(1fr) #context counter(page).display("1 / 1", both: true)
  ])
)

// Typography - clean, readable, functional
#set text(
  font: ("Fira Sans", "Liberation Sans"),
  fallback: true,
  lang: language,
  size: 10pt,
  fill: rams-black
)

// Moderate heading sizes - not too big, hierarchical but restrained
#show heading: set block(above: 1.4em, below: 0.8em)
#set heading(numbering: "1.1")

#show heading.where(level: 1): set text(
  size: 16pt,
  weight: "medium",
  fill: rams-dark-grey
)
#show heading.where(level: 2): set text(
  size: 14pt,
  weight: "medium",
  fill: rams-brown
)
#show heading.where(level: 3): set text(
  size: 12pt,
  weight: "medium",
  fill: rams-orange
)
#show heading.where(level: 4): set text(
  size: 11pt,
  weight: "semibold",
  fill: rams-red
)

// Functional accent colors for links
#show link: it => text(fill: rams-green, it)

// Clean code styling
#show raw: set text(
  font: ("Fira Code", "DejaVu Sans Mono"),
  fallback: true,
  size: 8pt
)
#show raw.where(block: false): set text(weight: "medium", fill: rams-orange.darken(20%), size: 8pt)
#show raw.where(block: true): block.with(
  fill: rams-brown.lighten(90%),
  inset: 12pt,
  radius: 3pt,
  width: 100%,
  stroke: (paint: rams-brown.lighten(60%), thickness: 0.5pt)
)

// Lists with subtle styling
#set list(indent: 1em, marker: ([•], [◦], [▪]))
#set enum(indent: 1em)

// Subtle emphasis with color accents
#show emph: set text(style: "italic", fill: rams-orange.darken(10%))
#show strong: set text(weight: "semibold", fill: rams-red)

// Colorful tag design with rotating colors
#let badge(content, index: 0) = {
  let colors = (rams-green, rams-brown, rams-orange, rams-red)
  let color = colors.at(calc.rem(index, colors.len()))
  box(
    inset: (x: 6pt, y: 3pt),
    radius: 2pt,
    fill: color.lighten(80%),
    stroke: (paint: color.lighten(40%), thickness: 0.5pt),
  )[
    #text(weight: "medium", size: 9pt, fill: color.darken(30%))[#content]
  ]
}

// Clean document header - functional information display
#if has-frontmatter [
  #v(1em)

  #if fm-title != none [
    #align(left)[
      #text(size: 20pt, weight: "medium", fill: rams-black)[#fm-title]
    ]
    #v(0.5em)
  ]

  #if fm-subtitle != none [
    #align(left)[
      #text(size: 13pt, style: "italic", fill: rams-dark-grey)[#fm-subtitle]
    ]
    #v(0.8em)
  ]

  // Minimal metadata presentation
  #let metadata = ()
  #if document-author != none { metadata.push(document-author) }
  #if document-date != none { metadata.push(document-date.display()) }
  #if fm-version != none { metadata.push("v" + fm-version) }

  #if metadata.len() > 0 [
    #set text(10pt, fill: rams-dark-grey)
    #for (i, data) in metadata.enumerate() [
      #data
      #if i < metadata.len() - 1 [ • ]
    ]
    #v(0.5em)
  ]

  #if fm-tags != none and tags-list.len() > 0 [
    #v(0.3em)
    #for (i, tag) in tags-list.enumerate() [
      #badge(tag.trim(), index: i)
      #if i < tags-list.len() - 1 [ ]
    ]
    #v(0.8em)
  ]

  // Subtle separator line with warm accent
  #line(length: 100%, stroke: (paint: rams-brown.lighten(50%), thickness: 1pt))
  #v(1.5em)
]

// Clean table of contents
#if show-toc [
  #set text(fill: rams-dark-grey)
  #show outline.entry: it => {
    set text(size: 10pt)
    it
  }
  #outline(indent: auto)
  #pagebreak()
]

// Enhanced content styling
#show quote: it => {
  set text(style: "italic", fill: rams-dark-grey)
  block(
    fill: rams-orange.lighten(95%),
    inset: (left: 12pt, rest: 8pt),
    radius: 3pt,
    stroke: (left: 3pt + rams-orange, rest: 0.5pt + rams-orange.lighten(60%))
  )[#it]
}

// Table styling with brown accents
#show table: it => {
  set text(size: 10pt)
  block(
    stroke: 1pt + rams-brown.lighten(60%),
    fill: rams-white,
    radius: 3pt,
    clip: true
  )[#it]
}

// Figure captions with orange accent
#set figure(numbering: "1", supplement: [Figure])
#set figure.caption(separator: " — ")
#show figure.caption: it => {
  set text(size: 10pt, style: "italic", fill: rams-orange.darken(20%))
  block(
    fill: rams-orange.lighten(95%),
    inset: 6pt,
    radius: 2pt,
    stroke: 0.5pt + rams-orange.lighten(50%)
  )[#it]
}

#cmarker.render(
  read(filepath),
  scope: (image: (path, alt: none) => image(path, alt: alt)),
  math: mitex
)
"#.to_string()
  }

  /// Fallback brutalist template (raw, bold, stark design)
  fn get_fallback_brutalist_template() -> String {
    r#"// Brutalist Template
#import "@preview/cmarker:0.1.8"
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
#let fm-version = sys.inputs.at("fm_version", default: none)

// Dieter Rams brutalist color palette
#let rams-white = rgb("f7f8f6ff")
#let rams-light-grey = rgb("d9d2c6ff")
#let rams-dark-grey = rgb("4a4a4aff")
#let rams-black = rgb("1f1f1fff")
#let rams-green = rgb("736b1eff")
#let rams-brown = rgb("8b7355ff")
#let rams-red = rgb("ed3f1cff")
#let rams-orange = rgb("ed8008ff")

// Parse tags from comma-separated string
#let tags-list = if fm-tags != none { fm-tags.split(",") } else { () }

// Extract filename from filepath
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
  let date-str = fm-date
  if date-str.len() == 10 and date-str.contains("-") {
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
  author: if document-author != none {document-author} else {""},
  title: document-title,
  keywords: if fm-tags != none { (if document-author != none {document-author} else {""}, if document-title != none {document-title} else {""}, "md-pdf", ..tags-list) } else { (if document-author != none {document-author} else {""}, if document-title != none {document-title} else {""}, "md-pdf") },
  date: document-date
)

// Raw page setup - minimal margins, stark white background
#set page(
  margin: (top: 2.5cm, bottom: 2.5cm, left: 2.5cm, right: 2.5cm),
  fill: rams-white
)

// Brutal header and footer - stark, functional blocks
#set page(
  header: context(if here().page() >= 2 [
    #set text(size: 9pt, fill: rams-black, font: ("Fira Mono"), weight: "semibold")
    #block(
      fill: rams-black,
      inset: (x: 8pt, y: 4pt),
      radius: 0pt,
      width: 100%
    )[
      #text(fill: rams-white)[#upper(document-title)]
    ]
  ]),
  footer: context(if here().page() >= 2 [
    #v(-2pt)
    #line(length: 100%, stroke: 2pt + rams-dark-grey)
    #v(2pt)
    #set text(size: 8pt, fill: rams-black, font: ("Fira Mono"), weight: "semibold")
    #grid(
      columns: (1fr, auto),
      align: (left, right),
      [#if document-author != none [#upper(document-author) /] #document-date.display()],
      [#context counter(page).display("1 / 1", both: true)]
    )
  ])
)

// Raw typography - monospace for stark, functional feel
#set text(
  font: ("Fira Mono"),
  size: 10pt,
  fill: rams-black,
  lang: language,
  fallback: true
)

// Brutal heading styles - blocky, angular, not too big
#show heading: set block(above: 1.2em, below: 0.8em)
#set heading(numbering: "1.1")

#show heading.where(level: 1): it => {
  set text(size: 16pt, weight: "black", fill: rams-white)
  set block(above: 1.5em, below: 1.2em)
  if it.numbering != none {
    let num = numbering(it.numbering, ..counter(heading).at(it.location()))
    block(
      fill: rams-black,
      inset: (x: 12pt, y: 8pt),
      radius: 0pt,
      width: 100%
    )[
      #text(weight: "black")[#num] #h(8pt) #upper(it.body)
    ]
  } else {
    block(
      fill: rams-black,
      inset: (x: 12pt, y: 8pt),
      radius: 0pt,
      width: 100%
    )[
      #upper(it.body)
    ]
  }
}

#show heading.where(level: 2): it => {
  set text(size: 13pt, weight: "bold", fill: rams-black)
  set block(above: 1.2em, below: 0.8em)
  if it.numbering != none {
    let num = numbering(it.numbering, ..counter(heading).at(it.location()))
    [#text(weight: "black")[#num] #h(8pt) #upper(it.body)]
  } else {
    [#upper(it.body)]
  }
  v(-4pt)
  line(length: 100%, stroke: 3pt + rams-dark-grey)
}

#show heading.where(level: 3): it => {
  set text(size: 11pt, weight: "bold", fill: rams-black)
  if it.numbering != none {
    let num = numbering(it.numbering, ..counter(heading).at(it.location()))
    [#text(weight: "black")[#num] #h(4pt) #upper(it.body)]
  } else {
    [#upper(it.body)]
  }
  v(-2pt)
  line(length: 60%, stroke: 2pt + rams-dark-grey)
}

// Raw link styling - no decoration, functional color
#show link: it => text(fill: rams-green, weight: "bold", it)

// Brutal code blocks - stark monospace with harsh borders
#show raw: set text(
  font: ("Iosevka", "DejaVu Sans Mono"),
  fallback: true
)
#show raw.where(block: false): it => {
  box(
    fill: rams-dark-grey,
    inset: (x: 3pt, y: 2pt),
    radius: 0pt,
    stroke: none
  )[
    #text(fill: rams-white, weight: "bold", size: 9pt)[#it]
  ]
}
#show raw.where(block: true): it => {
  set text(size: 9pt, fill: rams-white)
  block(
    fill: rams-black,
    width: 100%,
    inset: 12pt,
    radius: 0pt,
    stroke: 2pt + rams-dark-grey
  )[
    #it
  ]
}

// Raw lists - minimal styling, functional markers
#set list(indent: 8pt, body-indent: 4pt, marker: ([■], [▪], [▫]))
#show list: it => {
  set text(fill: rams-black)
  it
}

// Brutal emphasis styling
#show emph: set text(style: "italic", fill: rams-dark-grey, weight: "bold")
#show strong: set text(weight: "black", fill: rams-black)

// Raw quotes - stark left border
#show quote: it => {
  set text(fill: rams-black, weight: "bold")
  block(
    fill: rams-light-grey,
    inset: (left: 12pt, rest: 8pt),
    radius: 0pt,
    stroke: (left: 4pt + rams-red, rest: 2pt + rams-dark-grey)
  )[#it]
}

// Brutal tables - grid-like, stark
#show table: it => {
  set text(size: 9pt, weight: "bold")
  block(
    stroke: 2pt + rams-black,
    fill: rams-white,
    radius: 0pt,
  )[#it]
}

// Raw figure captions
#set figure(numbering: "1", supplement: [FIG])
#set figure.caption(separator: " - ")
#show figure.caption: it => {
  set text(size: 9pt, weight: "bold", fill: rams-white)
  block(
    fill: rams-black,
    inset: 6pt,
    width: 100%,
    radius: 0pt
  )[
    #it
  ]
}

// Brutal tag function - stark rectangular badges
#let brutal-tag(content) = {
  box(
    fill: rams-orange,
    inset: (x: 6pt, y: 3pt),
    radius: 0pt,
    stroke: 2pt + rams-black
  )[
    #text(weight: "black", size: 8pt, fill: rams-white)[#upper(content)]
  ]
}

// Brutal document header
#if has-frontmatter [
  // Title section - stark block
  #if fm-title != none [
    #align(left)[
      #block(
        fill: rams-black,
        inset: (x: 16pt, y: 12pt),
        radius: 0pt,
        width: 100%,
        stroke: none
      )[
        #text(size: 22pt, weight: "black", fill: rams-white)[#upper(fm-title)]
      ]
    ]
    #v(0.8em)
  ]

  #if fm-subtitle != none [
    #align(left)[
      #block(
        fill: rams-dark-grey,
        inset: (x: 12pt, y: 6pt),
        radius: 0pt,
        stroke: 2pt + rams-black
      )[
        #text(size: 14pt, weight: "bold", fill: rams-white)[#upper(fm-subtitle)]
      ]
    ]
    #v(0.8em)
  ]

  // Metadata section - stark grid
  #let metadata = ()
  #if document-author != none { metadata.push([#upper(document-author)]) }
  #if document-date != none { metadata.push([#document-date.display()]) }
  #if fm-version != none { metadata.push([V#fm-version]) }

  #if metadata.len() > 0 [
    #set text(size: 10pt, fill: rams-black, weight: "bold")
    #grid(
      columns: metadata.len(),
      column-gutter: 12pt,
      ..metadata.map(data => block(
        fill: rams-light-grey,
        inset: 6pt,
        stroke: 1pt + rams-dark-grey,
        radius: 0pt
      )[#data])
    )
    #v(0.8em)
  ]

  // Tags - rectangular blocks
  #if fm-tags != none and tags-list.len() > 0 [
    #for (i, tag) in tags-list.enumerate() [
      #brutal-tag(tag.trim())
      #if i < tags-list.len() - 1 [ #h(4pt) ]
    ]
    #v(0.8em)
  ]

  // Separator - thick line
  #line(length: 100%, stroke: 4pt + rams-black)
  #v(1.5em)
]

// Brutal table of contents
#if show-toc [
  #text(size: 16pt, weight: "black", fill: rams-black)[CONTENTS]
  #v(0.5em)
  #line(length: 100%, stroke: 2pt + rams-black)
  #v(0.5em)
  #set text(weight: "semibold", fill: rams-black)
  #outline(indent: auto)
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
    assert!(none_template.contains("None"));

    let simple_template = config.get_template_content("simple").unwrap();
    assert!(simple_template.contains("Simple"));

    let playful_template = config.get_template_content("playful").unwrap();
    assert!(playful_template.contains("Playful"));

    let brutalist_template = config.get_template_content("brutalist").unwrap();
    assert!(brutalist_template.contains("Brutalist"));
  }
}
