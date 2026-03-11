//! # Configuration Management
//!
//! This module handles configuration loading, template management, and provides fallback
//! templates for the md-pdf tool. It supports hierarchical configuration file search
//! and automatic creation of default configurations.

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Type alias for template information: (name, description, is_default)
pub type TemplateInfo = Vec<(String, String, bool)>;

/// Configuration structure for md-pdf application settings.
///
/// All fields are optional to allow for partial configuration and fallback to defaults.
///
/// # Examples
///
/// ```
/// use md_pdf::config::Config;
///
/// let config = Config::load().unwrap();
/// println!("Default template: {:?}", config.default_template);
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
  /// Directory containing custom template files
  pub templates_dir: Option<PathBuf>,
  /// Default template to use when none is specified
  pub default_template: Option<String>,
  /// Default language code for documents
  pub default_language: Option<String>,
  /// Default table of contents generation setting
  pub default_toc: Option<bool>,
  /// Default author name for documents
  pub default_author: Option<String>,
}

impl Default for Config {
  /// Create a default configuration with sensible defaults.
  ///
  /// # Examples
  ///
  /// ```
  /// use md_pdf::config::Config;
  ///
  /// let config = Config::default();
  /// assert_eq!(config.default_template, Some("simple".to_string()));
  /// ```
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
  /// Load configuration from the file system with hierarchical search.
  ///
  /// Searches for configuration files in priority order and loads the first one found.
  /// If no configuration file exists, creates a default one in `~/.config/md-pdf/config.ron`.
  ///
  /// # Examples
  ///
  /// ```
  /// use md_pdf::config::Config;
  ///
  /// let config = Config::load().unwrap();
  /// println!("Using template: {:?}", config.default_template);
  /// ```
  ///
  /// # Errors
  ///
  /// Returns an error if file I/O operations fail or configuration format is invalid.
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
          let config_display = default_config_path
            .canonicalize()
            .unwrap_or_else(|_| default_config_path.to_path_buf());
          println!("No configuration file found, created default at: {}", config_display.display());
          let templates_display = config_display.parent().unwrap_or(&config_display).join("templates");
          println!("Default templates created at: {}", templates_display.display());
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

  /// Get list of configuration file locations in priority order.
  ///
  /// # Returns
  ///
  /// A vector of potential configuration file paths ordered by search priority.
  ///
  /// # Errors
  ///
  /// Returns an error if the current directory or executable path cannot be determined.
  fn get_config_locations() -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut locations = Vec::new();

    // 1. Current execution directory as md-pdf.ron
    if let Ok(current_dir) = env::current_dir() {
      locations.push(current_dir.join("md-pdf.ron"));
    }

    // 2. Binary location as md-pdf.ron
    if let Ok(exe_path) = env::current_exe()
      && let Some(exe_dir) = exe_path.parent()
    {
      locations.push(exe_dir.join("md-pdf.ron"));
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

  /// Get the templates directory path with intelligent fallback strategy.
  ///
  /// # Examples
  ///
  /// ```
  /// use md_pdf::config::Config;
  ///
  /// let config = Config::load().unwrap();
  /// let templates_dir = config.get_templates_dir();
  /// println!("Templates directory: {}", templates_dir.display());
  /// ```
  pub fn get_templates_dir(&self) -> PathBuf {
    if let Some(templates_dir) = &self.templates_dir
      && templates_dir.exists()
    {
      return templates_dir.clone();
    }

    // Fallback: check for templates directory relative to current directory
    let current_templates = PathBuf::from("templates");
    if current_templates.exists() {
      return current_templates;
    }

    // Fallback: check for templates directory relative to binary
    if let Ok(exe_path) = env::current_exe()
      && let Some(exe_dir) = exe_path.parent()
    {
      let binary_templates = exe_dir.join("templates");
      if binary_templates.exists() {
        return binary_templates;
      }
    }

    // Final fallback: current directory
    PathBuf::from(".")
  }

  /// List all available template names from the templates directory.
  ///
  /// Scans for `.typ` files and includes built-in templates.
  ///
  /// # Examples
  ///
  /// ```
  /// use md_pdf::config::Config;
  ///
  /// let config = Config::load().unwrap();
  /// let templates = config.list_available_templates().unwrap();
  /// for template in templates {
  ///     println!("Available template: {}", template);
  /// }
  /// ```
  ///
  /// # Errors
  ///
  /// Returns an error if the templates directory cannot be read.
  pub fn list_available_templates(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let templates_dir = self.get_templates_dir();
    let mut templates = Vec::new();

    // Scan templates directory for .typ files
    if templates_dir.exists() && templates_dir.is_dir() {
      for entry in fs::read_dir(&templates_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file()
          && let Some(extension) = path.extension()
          && extension == "typ"
          && let Some(name) = path.file_stem()
          && let Some(name_str) = name.to_str()
        {
          templates.push(name_str.to_string());
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

  /// Get detailed template information including descriptions.
  ///
  /// # Examples
  ///
  /// ```
  /// use md_pdf::config::Config;
  ///
  /// let config = Config::load().unwrap();
  /// let template_info = config.get_template_info().unwrap();
  /// for (name, description, is_default) in template_info {
  ///     println!("{}: {} {}", name, description, if is_default { "(default)" } else { "" });
  /// }
  /// ```
  ///
  /// # Errors
  ///
  /// Returns an error if template information cannot be determined.
  pub fn get_template_info(&self) -> Result<TemplateInfo, Box<dyn std::error::Error>> {
    let templates = self.list_available_templates()?;
    let mut template_info = Vec::new();
    let default_template = self.default_template.as_deref();

    for template in templates {
      let is_default = Some(template.as_str()) == default_template;
      let description = match template.as_str() {
        "none" => "Minimal styling".to_string(),
        "simple" => "Professional with headers/footers".to_string(),
        "playful" => "Colorful inspired by Dieter Rams".to_string(),
        "brutalist" => "Raw, bold, stark design with high contrast".to_string(),
        "darko" => "Dark theme template".to_string(),
        _ => "Custom template".to_string(),
      };
      template_info.push((template, description, is_default));
    }

    Ok(template_info)
  }

  /// Get template content for the specified template name.
  ///
  /// Returns either file content or built-in fallback template.
  ///
  /// # Examples
  ///
  /// ```
  /// use md_pdf::config::Config;
  ///
  /// let config = Config::load().unwrap();
  /// let template_content = config.get_template_content("simple").unwrap();
  /// println!("Template length: {} bytes", template_content.len());
  /// ```
  ///
  /// # Errors
  ///
  /// Returns an error if the template cannot be found or loaded.
  pub fn get_template_content(&self, template_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let templates_dir = self.get_templates_dir();
    let template_file = templates_dir.join(format!("{template_name}.typ"));

    // Try to load from file first
    if template_file.exists() {
      return Ok(fs::read_to_string(&template_file)?);
    }

    // Fallback to built-in templates
    match template_name {
      "none" => Ok(Self::get_fallback_none_template()),
      "simple" => Ok(Self::get_fallback_simple_template()),
      "playful" => Ok(Self::get_fallback_playful_template()),
      "brutalist" => Ok(Self::get_fallback_brutalist_template()),
      "darko" => Ok(Self::get_fallback_darko_template()),
      _ => Err(format!("Template '{template_name}' not found").into()),
    }
  }

  /// Create a default configuration file at the specified path.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use md_pdf::config::Config;
  /// use std::path::Path;
  ///
  /// let config_path = Path::new("./md-pdf.ron");
  /// Config::create_default_config(config_path).unwrap();
  /// ```
  ///
  /// # Errors
  ///
  /// Returns an error if the file cannot be created or written.
  pub fn create_default_config(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
      fs::create_dir_all(parent)?;
    }

    // Resolve templates directory relative to the config file location
    let templates_dir = path.parent().map(|p| p.join("templates"));

    // Create default config with templates_dir set
    let default_config = Config {
      templates_dir: templates_dir.clone(),
      ..Config::default()
    };

    // Serialize to RON format
    let config_content = ron::ser::to_string_pretty(&default_config, ron::ser::PrettyConfig::default())?;

    // Write to file
    fs::write(path, config_content)?;

    // Create templates directory and write default template files
    if let Some(dir) = templates_dir {
      fs::create_dir_all(&dir)?;

      let builtin_templates = [
        ("none", Self::get_fallback_none_template()),
        ("simple", Self::get_fallback_simple_template()),
        ("playful", Self::get_fallback_playful_template()),
        ("brutalist", Self::get_fallback_brutalist_template()),
        ("darko", Self::get_fallback_darko_template()),
      ];

      for (name, content) in builtin_templates {
        let template_path = dir.join(format!("{name}.typ"));
        if !template_path.exists() {
          fs::write(template_path, content)?;
        }
      }
    }

    Ok(())
  }

  /// Get the fallback "none" template content.
  fn get_fallback_none_template() -> String {
    concat!(
      "#set page(paper: \"a4\", margin: (top: 1.5cm, bottom: 1.5cm, left: 2cm, right: 2cm))\n",
      "#set text(font: \"Times New Roman\", size: 11pt)\n",
      "#set par(justify: true, leading: 0.55em)\n",
      "#set heading(numbering: \"1.\")\n\n",
      "#show raw.where(block: true): block.with(\n",
      "  fill: luma(240),\n",
      "  inset: 8pt,\n",
      "  radius: 4pt,\n",
      "  width: 100%\n",
      ")\n\n",
      "#show link: underline\n\n",
      "#include filepath\n"
    )
    .to_string()
  }

  /// Get the fallback "simple" template content.
  fn get_fallback_simple_template() -> String {
    concat!(
      "#set page(paper: \"a4\", margin: (top: 2.5cm, bottom: 2.5cm, left: 2cm, right: 2cm))\n",
      "#set text(font: \"Times New Roman\", size: 11pt)\n",
      "#set par(justify: true, leading: 0.55em)\n",
      "#set heading(numbering: \"1.\")\n\n",
      "// Header and footer\n",
      "#set page(\n",
      "  header: locate(loc => {\n",
      "    let headings = query(heading.where(level: 1), loc)\n",
      "    if headings.len() > 0 {\n",
      "      align(right)[#text(size: 9pt, style: \"italic\")[Chapter #headings.last().numbering]]\n",
      "    }\n",
      "  }),\n",
      "  footer: locate(loc => {\n",
      "    align(center)[#text(size: 9pt)[#counter(page).display()]]\n",
      "  })\n",
      ")\n\n",
      "#show raw.where(block: true): block.with(\n",
      "  fill: luma(240),\n",
      "  inset: 8pt,\n",
      "  radius: 4pt,\n",
      "  width: 100%\n",
      ")\n\n",
      "#show link: underline\n\n",
      "#include filepath\n"
    )
    .to_string()
  }

  /// Get the fallback "playful" template content.
  fn get_fallback_playful_template() -> String {
    concat!(
      "#set page(paper: \"a4\", margin: (top: 2cm, bottom: 2cm, left: 2cm, right: 2cm))\n",
      "#set text(font: (\"Helvetica\", \"Arial\"), size: 11pt)\n",
      "#set par(justify: true, leading: 0.6em)\n\n",
      "#set heading(numbering: \"1.\")\n",
      "#show heading.where(level: 1): it => [\n",
      "  #set text(fill: orange, size: 18pt, weight: \"bold\")\n",
      "  #block(spacing: 1.5em)[#it]\n",
      "]\n\n",
      "#show heading.where(level: 2): it => [\n",
      "  #set text(fill: blue, size: 14pt, weight: \"bold\")\n",
      "  #block(spacing: 1em)[#it]\n",
      "]\n\n",
      "#show raw.where(block: true): block.with(\n",
      "  fill: luma(248),\n",
      "  stroke: (left: 3pt + orange),\n",
      "  inset: 12pt,\n",
      "  radius: 4pt,\n",
      "  width: 100%\n",
      ")\n\n",
      "#show link: it => [\n",
      "  #set text(fill: blue)\n",
      "  #underline(it)\n",
      "]\n\n",
      "#include filepath\n"
    )
    .to_string()
  }

  /// Get the fallback "brutalist" template content.
  fn get_fallback_brutalist_template() -> String {
    concat!(
      "#set page(paper: \"a4\", margin: (top: 1cm, bottom: 1cm, left: 1cm, right: 1cm))\n",
      "#set text(font: \"Courier New\", size: 10pt)\n",
      "#set par(justify: false, leading: 0.5em)\n\n",
      "#set heading(numbering: \"1.\")\n",
      "#show heading.where(level: 1): it => [\n",
      "  #set text(size: 16pt, weight: \"bold\")\n",
      "  #block(\n",
      "    fill: black,\n",
      "    inset: 8pt,\n",
      "    width: 100%\n",
      "  )[\n",
      "    #text(fill: white)[#it.body]\n",
      "  ]\n",
      "  #v(0.5em)\n",
      "]\n\n",
      "#show raw.where(block: true): block.with(\n",
      "  fill: black,\n",
      "  inset: 8pt,\n",
      "  radius: 0pt,\n",
      "  width: 100%\n",
      ")\n\n",
      "#show raw: it => [\n",
      "  #set text(fill: white, font: \"Courier New\")\n",
      "  #it\n",
      "]\n\n",
      "#show link: it => [\n",
      "  #set text(weight: \"bold\")\n",
      "  #rect(stroke: 1pt + black, inset: 2pt)[#it]\n",
      "]\n\n",
      "#include filepath\n"
    )
    .to_string()
  }

  /// Get the fallback "darko" template content.
  fn get_fallback_darko_template() -> String {
    concat!(
      "#set page(\n",
      "  paper: \"a4\",\n",
      "  margin: (top: 2cm, bottom: 2cm, left: 2cm, right: 2cm),\n",
      "  fill: rgb(\"1a1a1a\")\n",
      ")\n",
      "#set text(font: (\"Helvetica\", \"Arial\"), size: 11pt, fill: white)\n",
      "#set par(justify: true, leading: 0.6em)\n\n",
      "#set heading(numbering: \"1.\")\n",
      "#show heading.where(level: 1): it => [\n",
      "  #set text(fill: purple, size: 18pt, weight: \"bold\")\n",
      "  #block(spacing: 1.5em)[#it]\n",
      "]\n\n",
      "#show heading.where(level: 2): it => [\n",
      "  #set text(fill: teal, size: 14pt, weight: \"bold\")\n",
      "  #block(spacing: 1em)[#it]\n",
      "]\n\n",
      "#show raw.where(block: true): block.with(\n",
      "  fill: luma(40),\n",
      "  stroke: (left: 3pt + purple),\n",
      "  inset: 12pt,\n",
      "  radius: 6pt,\n",
      "  width: 100%\n",
      ")\n\n",
      "#show raw: it => [\n",
      "  #set text(fill: white)\n",
      "  #it\n",
      "]\n\n",
      "#show link: it => [\n",
      "  #set text(fill: teal)\n",
      "  #underline(it)\n",
      "]\n\n",
      "#include filepath\n"
    )
    .to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default_config() {
    let config = Config::default();
    assert_eq!(config.default_template, Some("simple".to_string()));
    assert_eq!(config.default_language, Some("en".to_string()));
    assert_eq!(config.default_toc, Some(true));
  }

  #[test]
  fn test_config_serialization() {
    let config = Config::default();
    let serialized = ron::ser::to_string(&config).unwrap();
    let _deserialized: Config = ron::from_str(&serialized).unwrap();
  }

  #[test]
  fn test_fallback_templates() {
    assert!(!Config::get_fallback_none_template().is_empty());
    assert!(!Config::get_fallback_simple_template().is_empty());
    assert!(!Config::get_fallback_playful_template().is_empty());
    assert!(!Config::get_fallback_brutalist_template().is_empty());
    assert!(!Config::get_fallback_darko_template().is_empty());
  }
}
