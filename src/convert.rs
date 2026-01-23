use crate::config::Config;
use crate::frontmatter::{frontmatter_to_typst_vars, parse_frontmatter};
use duct::cmd;
use notify::{Config as NotifyConfig, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::path::Path;
use std::sync::mpsc::channel;

/// Converts a Markdown file to PDF using typst-cli with the specified template.
pub fn convert_to_pdf(
    md_file: &Path,
    output: &str,
    template_option: Option<&String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::load()?;

    // Read and parse the markdown file for front matter first
    let content = fs::read_to_string(md_file)?;
    let (front_matter, markdown_content) = parse_frontmatter(&content)?;

    // Template selection: CLI arg > front matter > config default
    let effective_template =
        if template_option.is_some() && template_option != Some(&"none".to_string()) {
            template_option
        } else if let Some(fm) = &front_matter {
            fm.template.as_ref()
        } else {
            config.default_template.as_ref()
        };

    // Language selection: front matter > config default
    let effective_language = if let Some(fm) = &front_matter {
        fm.language.as_ref().map(|s| s.as_str()).unwrap_or_else(|| {
            config
                .default_language
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or("en")
        })
    } else {
        config
            .default_language
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("en")
    };

    // TOC selection: front matter > config default
    let effective_toc = if let Some(fm) = &front_matter {
        fm.toc
            .unwrap_or_else(|| config.default_toc.unwrap_or(false))
    } else {
        config.default_toc.unwrap_or(false)
    };

    // Get template content from config system
    let template_name = effective_template.map(|s| s.as_str()).unwrap_or("none");
    let template_string = config.get_template_content(template_name)?;

    // Get default author from config
    let default_author = config
        .default_author
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("User");

    // Get absolute paths for proper execution context
    let md_file_absolute = md_file.canonicalize()?;
    let md_dir = md_file_absolute.parent().unwrap_or_else(|| Path::new("."));
    let output_absolute = Path::new(output).canonicalize().unwrap_or_else(|_| {
        // If output doesn't exist, make it absolute relative to current dir
        std::env::current_dir().unwrap().join(output)
    });

    // Create temporary file with processed markdown content in the same directory as the source
    let temp_md_filename = format!("{}.temp.md", md_file_absolute.file_stem().unwrap().to_str().unwrap());
    let temp_md_path = md_dir.join(&temp_md_filename);
    fs::write(&temp_md_path, &markdown_content)?;

    // Build typst command arguments
    let mut args = vec![
        "compile".to_string(),
        "--root".to_string(),
        md_dir.to_str().unwrap().to_string(),
        "--input".to_string(),
        format!("default_author={}", default_author),
        "--input".to_string(),
        format!("filepath={}", temp_md_filename),
        "--input".to_string(),
        format!("language={}", effective_language),
        "--input".to_string(),
        format!("toc={}", effective_toc),
    ];

    // Add front matter variables if present
    if let Some(fm) = &front_matter {
        let fm_vars = frontmatter_to_typst_vars(fm);
        for (key, value) in fm_vars {
            args.push("--input".to_string());
            args.push(format!("{}={}", key, value));
        }
    } else {
        // Add default flag to indicate no front matter
        args.push("--input".to_string());
        args.push("has_frontmatter=false".to_string());
    }

    args.push("-".to_string());
    args.push(output_absolute.to_str().unwrap().to_string());

    let result = cmd("typst", &args)
        .dir(md_dir) // Execute typst from markdown file's directory
        .stdin_bytes(template_string.as_bytes())
        .run();

    // Clean up temporary file
    if temp_md_path.exists() {
        let _ = fs::remove_file(&temp_md_path);
    }

    match result {
        Ok(_) => {
            if let Some(fm) = &front_matter {
                if let Some(title) = &fm.title {
                    println!("✓ Document: {}", title);
                }
                if let Some(author) = &fm.author {
                    println!("✓ Author: {}", author);
                }
                if let Some(date) = &fm.date {
                    println!("✓ Date: {}", date.to_string());
                }
                if let Some(version) = &fm.version {
                    println!("✓ Version: {}", version);
                }
                println!("✓ Language: {}", effective_language);
                if let Some(template) = &fm.template {
                    if template_option.is_some() && template_option != Some(&"none".to_string()) {
                        println!(
                            "✓ Template (CLI override): {} (front matter had: {})",
                            effective_template.unwrap_or(&"none".to_string()),
                            template
                        );
                    } else {
                        println!("✓ Template (from front matter): {}", template);
                    }
                } else if template_option.is_some() {
                    println!(
                        "✓ Template (CLI): {}",
                        template_option.unwrap_or(&"none".to_string())
                    );
                }
                println!("✓ TOC: {}", effective_toc);
                if !fm.tags.is_empty() {
                    println!("✓ Tags: {}", fm.tags.join(", "));
                }
            }
            println!(
                "✓ Successfully converted '{}' to '{}'",
                md_file_absolute.display(),
                output_absolute.display()
            );
            Ok(())
        }
        Err(e) => {
            eprintln!("✗ Error during conversion: {}", e);
            eprintln!("Make sure typst-cli is installed and accessible in your PATH");
            Err(e.into())
        }
    }
}

/// Watch a file for changes and rebuild automatically
pub fn watch_file(
    md_file: &Path,
    output: &str,
    template_option: Option<&String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let md_file_absolute = md_file.canonicalize()?;
    println!(
        "👀 Watching '{}' for changes...",
        md_file_absolute.display()
    );
    println!("Press Ctrl+C to stop watching");

    // Initial conversion
    convert_to_pdf(md_file, output, template_option)?;

    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, NotifyConfig::default())?;

    // Watch the file
    watcher.watch(&md_file_absolute, RecursiveMode::NonRecursive)?;

    loop {
        match rx.recv() {
            Ok(event) => {
                match event {
                    Ok(event) => {
                        // Check if this is a modify event
                        if event.kind.is_modify() {
                            println!("\n🔄 File changed, rebuilding...");
                            match convert_to_pdf(md_file, output, template_option) {
                                Ok(_) => println!("✅ Rebuild complete"),
                                Err(e) => eprintln!("❌ Rebuild failed: {}", e),
                            }
                            println!("👀 Watching for changes...");
                        }
                    }
                    Err(e) => eprintln!("Watch error: {:?}", e),
                }
            }
            Err(e) => {
                eprintln!("Watch error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}

/// List available templates
pub fn list_available_templates() {
    match Config::load() {
        Ok(config) => {
            println!("Available templates:");
            match config.get_template_info() {
                Ok(template_info) => {
                    for (name, description, is_default) in template_info {
                        let default_indicator = if is_default { " (default)" } else { "" };
                        println!("  {:<15} - {}{}", name, description, default_indicator);
                    }
                }
                Err(e) => {
                    eprintln!("Error listing templates: {}", e);
                    // Fallback to basic list
                    match config.list_available_templates() {
                        Ok(templates) => {
                            let default_template =
                                config.default_template.as_ref().map(|s| s.as_str());
                            for template in templates {
                                let is_default = Some(template.as_str()) == default_template;
                                let default_indicator = if is_default { " (default)" } else { "" };
                                let description = format!("Template file{}", default_indicator);
                                println!("  {:<15} - {}", template, description);
                            }
                        }
                        Err(_) => {
                            println!("  Unable to list templates");
                        }
                    }
                }
            }
            println!(
                "\nTemplates directory: {}",
                config
                    .get_templates_dir()
                    .canonicalize()
                    .unwrap_or_else(|_| config.get_templates_dir().to_path_buf())
                    .display()
            );
            println!("You can also specify a path to a custom .typ template file.");
        }
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
            println!("No templates available - configuration could not be loaded");
        }
    }
}
