mod args;
mod config;
mod convert;
mod frontmatter;
mod link_checker;

use args::Args;
use config::Config;
use convert::{convert_to_pdf, list_available_templates, watch_file};
use dirs;
use link_checker::check_links_in_file;
use std::path::Path;
use std::process;

fn main() {
    let args = Args::parse();

    // Handle config-related commands first
    if args.should_show_config() {
        show_config_info();
        return;
    }

    if args.should_create_config() {
        create_default_config();
        return;
    }

    // If user wants to list templates, do that and exit
    if args.should_list_templates() {
        list_available_templates();
        return;
    }

    // Check if input file is provided
    let input_file = match args.get_input() {
        Some(input) => input,
        None => {
            eprintln!("Error: Input markdown file is required.");
            eprintln!(
                "Use --help for usage information or --list-templates to see available templates."
            );
            process::exit(1);
        }
    };

    // Check if input file exists
    let input_path = Path::new(input_file);
    if !input_path.exists() {
        eprintln!(
            "Error: Input file '{}' does not exist.",
            input_path
                .canonicalize()
                .unwrap_or_else(|_| input_path.to_path_buf())
                .display()
        );
        process::exit(1);
    }

    // If user wants to check links, do that first
    if args.should_check_links() {
        match check_links_in_file(input_path) {
            Ok(_) => {
                // Link checking completed successfully
            }
            Err(e) => {
                eprintln!("Link checking failed: {}", e);
                // Don't exit here - we can still proceed with PDF conversion
            }
        }
        println!(); // Add blank line after link checking
    }

    // Show template being used
    match args.get_template() {
        Some(template) => println!("Using template: {}", template),
        None => println!("Using default template"),
    }

    let output_path = args.get_output_path();

    // Check if watch mode is enabled
    if args.should_watch() {
        match watch_file(input_path, &output_path, args.get_template()) {
            Ok(_) => {
                println!("Watch mode stopped.");
            }
            Err(e) => {
                eprintln!("Watch mode failed: {}", e);
                process::exit(1);
            }
        }
    } else {
        // For single conversion, only show starting message if we didn't already check links
        if !args.should_check_links() {
            println!("Starting Markdown to PDF Converter.");
        }

        match convert_to_pdf(input_path, &output_path, args.get_template()) {
            Ok(_) => {
                println!("Done.");
            }
            Err(e) => {
                eprintln!("Conversion failed: {}", e);
                process::exit(1);
            }
        }
    }
}

/// Show configuration information
fn show_config_info() {
    match Config::load() {
        Ok(config) => {
            println!("Configuration Information");
            println!("========================");
            println!(
                "Templates directory: {}",
                config
                    .get_templates_dir()
                    .canonicalize()
                    .unwrap_or_else(|_| config.get_templates_dir().to_path_buf())
                    .display()
            );

            println!(
                "Default template: {}",
                config
                    .default_template
                    .as_ref()
                    .unwrap_or(&"none".to_string())
            );
            println!(
                "Default language: {}",
                config
                    .default_language
                    .as_ref()
                    .unwrap_or(&"en".to_string())
            );
            println!("Default TOC: {}", config.default_toc.unwrap_or(false));
            println!(
                "Default author: {}",
                config
                    .default_author
                    .as_ref()
                    .unwrap_or(&"User".to_string())
            );

            println!("\nConfiguration file search locations (in priority order):");
            println!("  1. ./md-pdf.ron");
            println!("  2. <binary-dir>/md-pdf.ron");
            println!("  3. ~/md-pdf.ron");
            println!("  4. ~/.config/md-pdf.ron");
            println!("  5. ~/.config/md-pdf/config.ron");
            println!("  6. ~/.config/zas/md-pdf.ron");
        }
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
            process::exit(1);
        }
    }
}

/// Create default configuration file
fn create_default_config() {
    // Try to create config in ~/.config/md-pdf/config.ron
    if let Some(home_dir) = dirs::home_dir() {
        let config_path = home_dir.join(".config").join("md-pdf").join("config.ron");
        match Config::create_default_config(&config_path) {
            Ok(_) => {
                println!(
                    "✓ Created default configuration at: {}",
                    config_path
                        .canonicalize()
                        .unwrap_or_else(|_| config_path.to_path_buf())
                        .display()
                );
                println!("You can edit this file to customize templates directory and defaults.");
            }
            Err(e) => {
                eprintln!("Error creating config file: {}", e);
                process::exit(1);
            }
        }
    } else {
        eprintln!("Could not determine home directory");
        process::exit(1);
    }
}
