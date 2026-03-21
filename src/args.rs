//! # Command-Line Argument Parsing
//!
//! This module defines the command-line interface for the md-pdf tool using `clap`.
//! It handles all user inputs including file paths, options, and flags for various operations.

use clap::Parser;

/// Command-line arguments for the md-pdf tool.
///
/// # Examples
///
/// ```
/// use md_pdf::args::Args;
///
/// let args = Args::parse();
/// if let Some(input) = args.get_input() {
///     println!("Converting file: {}", input);
/// }
/// ```
#[derive(Debug, Clone, clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
  /// Path to the input Markdown file.
  input: Option<String>,
  /// Path to the output PDF file.
  #[arg(short, long)]
  pub output: Option<String>,
  /// Template to use for PDF generation.
  #[arg(short, long, default_value = "none")]
  pub template: Option<String>,
  /// Watch the input file for changes and rebuild automatically.
  #[arg(short, long)]
  pub watch: bool,
  /// Check all links in the markdown file and display warnings for unreachable links.
  #[arg(long)]
  pub check_links: bool,
  /// List all available templates.
  #[arg(long)]
  pub list_templates: bool,
  /// Create default configuration file.
  #[arg(long)]
  pub create_config: bool,
  /// Show configuration file locations and settings.
  #[arg(long)]
  pub show_config: bool,
  /// Open the generated PDF file after creation.
  #[arg(long)]
  pub open: bool,
  /// Refresh templates in the config directory with the latest embedded versions.
  #[arg(long)]
  pub refresh_templates: bool,
}

impl Args {
  /// Parse command-line arguments from the environment.
  ///
  /// # Examples
  ///
  /// ```
  /// let args = Args::parse();
  /// ```
  ///
  /// # Panics
  ///
  /// Exits the program if invalid arguments are provided or `--help`/`--version` flags are used.
  pub fn parse() -> Self {
    Parser::parse()
  }

  /// Get the input file path if provided.
  ///
  /// # Examples
  ///
  /// ```
  /// let args = Args::parse();
  /// match args.get_input() {
  ///     Some(input) => println!("Input file: {}", input),
  ///     None => println!("No input file specified"),
  /// }
  /// ```
  pub fn get_input(&self) -> Option<&String> {
    self.input.as_ref()
  }

  /// Generate the output file path.
  ///
  /// Returns the output file path from `--output` argument, or automatically
  /// generates one by replacing `.md` with `.pdf`.
  ///
  /// # Examples
  ///
  /// ```
  /// let args = Args::parse();
  /// let output_path = args.get_output_path();
  /// println!("Output will be saved to: {}", output_path);
  /// ```
  pub fn get_output_path(&self) -> String {
    match &self.output {
      Some(output) => output.clone(),
      None => match &self.input {
        Some(input) => input.clone().replace(".md", ".pdf"),
        None => "output.pdf".to_string(),
      },
    }
  }

  /// Check if template listing was requested.
  pub fn should_list_templates(&self) -> bool {
    self.list_templates
  }

  /// Get the template name if specified.
  pub fn get_template(&self) -> Option<&String> {
    self.template.as_ref()
  }

  /// Check if watch mode was requested.
  pub fn should_watch(&self) -> bool {
    self.watch
  }

  /// Check if configuration creation was requested.
  pub fn should_create_config(&self) -> bool {
    self.create_config
  }

  /// Check if configuration display was requested.
  pub fn should_show_config(&self) -> bool {
    self.show_config
  }

  /// Check if link checking was requested.
  pub fn should_check_links(&self) -> bool {
    self.check_links
  }

  /// Check if opening the PDF file was requested.
  pub fn should_open(&self) -> bool {
    self.open
  }

  /// Check if template refresh was requested.
  pub fn should_refresh_templates(&self) -> bool {
    self.refresh_templates
  }
}
