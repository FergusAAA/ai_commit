// ===================================================================
// Command-line Interface
// ===================================================================

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about = "AI-powered commit message generator.", long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(
        short,
        long,
        help = "Language for the commit message. Overrides config."
    )]
    pub language: Option<String>,

    #[clap(
        short,
        long,
        help = "Custom prompt for the AI model. Overrides config."
    )]
    pub prompt: Option<String>,

    #[clap(long, help = "Custom URL for the AI model's API. Overrides config.")]
    pub url: Option<String>,

    #[clap(
        long,
        help = "The specific model to use for generation. Overrides config."
    )]
    pub model: Option<String>,

    #[clap(short = 'm', hide = true)]
    pub msg: bool,

    #[clap(long, hide = true, value_name = "SHELL", min_values = 0, max_values = 1)]
    pub gen_completion: Option<Option<String>>,

    #[clap(subcommand)]
    pub command: Option<SubCommand>,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    /// Manage configuration.
    Config(ConfigArgs),
}

#[derive(Parser, Debug)]
pub struct ConfigArgs {
    #[clap(subcommand)]
    pub command: ConfigCmd,
}

#[derive(Parser, Debug)]
pub enum ConfigCmd {
    #[clap(about = "Set the API key for the AI service.")]
    SetApiKey { key: String },
    #[clap(about = "Set the API URL for a custom AI model endpoint.")]
    SetUrl { url: String },
    #[clap(about = "Set the default model to use for generation.")]
    SetModel { model: String },
    #[clap(about = "Set the default language for commit messages.")]
    SetLanguage { lang: String },
    #[clap(about = "Set a default prompt to guide the AI.")]
    SetPrompt { prompt: String },
    #[clap(about = "Show the current configuration (hides API key for security).")]
    Show,
}
