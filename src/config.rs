//! # Configuration Management
//!
//! This module handles configuration loading, template management, and provides fallback
//! templates for the md-pdf tool. It supports hierarchical configuration file search
//! and automatic creation of default configurations.

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

// Include the auto-generated embedded templates module
mod embedded_templates {
  include!(concat!(env!("OUT_DIR"), "/embedded_templates.rs"));
}

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
      default_author: Some("md-pdf".to_string()),
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
          let config_display = default_config_path.canonicalize().unwrap_or_else(|_| default_config_path.to_path_buf());
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

    // Add all embedded templates if not already found in directory
    for embedded_name in embedded_templates::EMBEDDED_TEMPLATE_NAMES {
      if !templates.contains(&embedded_name.to_string()) {
        templates.push(embedded_name.to_string());
      }
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

    // Fallback to embedded templates
    if let Some(content) = embedded_templates::get_embedded_template(template_name) {
      return Ok(content.to_string());
    }

    Err(format!("Template '{template_name}' not found").into())
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

      // Use embedded templates from the build script
      for name in embedded_templates::EMBEDDED_TEMPLATE_NAMES {
        if let Some(content) = embedded_templates::get_embedded_template(name) {
          let template_path = dir.join(format!("{name}.typ"));
          if !template_path.exists() {
            fs::write(&template_path, content)?;
          }
        }
      }
    }

    Ok(())
  }

  /// Refresh templates in the config directory with the latest embedded versions.
  ///
  /// This will overwrite any existing templates with the same names as the embedded templates.
  /// Custom templates with different names will be preserved.
  ///
  /// # Arguments
  ///
  /// * `templates_dir` - The directory where templates should be written
  ///
  /// # Returns
  ///
  /// A vector of template names that were refreshed.
  ///
  /// # Errors
  ///
  /// Returns an error if the directory cannot be created or templates cannot be written.
  pub fn refresh_templates(templates_dir: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    fs::create_dir_all(templates_dir)?;

    let mut refreshed = Vec::new();

    for name in embedded_templates::EMBEDDED_TEMPLATE_NAMES {
      if let Some(content) = embedded_templates::get_embedded_template(name) {
        let template_path = templates_dir.join(format!("{name}.typ"));
        fs::write(&template_path, content)?;
        refreshed.push(name.to_string());
      }
    }

    Ok(refreshed)
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
  fn test_embedded_templates() {
    use super::embedded_templates;

    // Test that all embedded templates are non-empty
    for name in embedded_templates::EMBEDDED_TEMPLATE_NAMES {
      let content = embedded_templates::get_embedded_template(name);
      assert!(content.is_some(), "Template '{}' should exist", name);
      assert!(!content.unwrap().is_empty(), "Template '{}' should not be empty", name);
    }

    // Test that we have at least the expected templates
    assert!(embedded_templates::EMBEDDED_TEMPLATE_NAMES.contains(&"none"));
    assert!(embedded_templates::EMBEDDED_TEMPLATE_NAMES.contains(&"simple"));
    assert!(embedded_templates::EMBEDDED_TEMPLATE_NAMES.contains(&"darko"));
    assert!(embedded_templates::EMBEDDED_TEMPLATE_NAMES.contains(&"brutalist"));
    assert!(embedded_templates::EMBEDDED_TEMPLATE_NAMES.contains(&"playful"));
  }
}
