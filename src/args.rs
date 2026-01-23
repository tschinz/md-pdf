use clap::Parser;

#[derive(Debug, Clone, clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the input Markdown file.
    input: Option<String>,
    /// Path to the output PDF file.
    #[arg(short, long)]
    pub output: Option<String>,
    /// list the templates and select the one you want.
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
}

impl Args {
    pub fn parse() -> Self {
        Parser::parse()
    }

    pub fn get_input(&self) -> Option<&String> {
        self.input.as_ref()
    }

    pub fn get_output_path(&self) -> String {
        match &self.output {
            Some(output) => output.clone(),
            None => match &self.input {
                Some(input) => input.clone().replace(".md", ".pdf"),
                None => "output.pdf".to_string(),
            },
        }
    }

    pub fn should_list_templates(&self) -> bool {
        self.list_templates
    }

    pub fn get_template(&self) -> Option<&String> {
        self.template.as_ref()
    }

    pub fn should_watch(&self) -> bool {
        self.watch
    }

    pub fn should_create_config(&self) -> bool {
        self.create_config
    }

    pub fn should_show_config(&self) -> bool {
        self.show_config
    }

    pub fn should_check_links(&self) -> bool {
        self.check_links
    }
}
