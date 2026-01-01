use clap::Parser;
use directories::ProjectDirs;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

// ===================================================================
// Configuration Management
// ===================================================================

#[derive(Serialize, Deserialize, Debug, Default)]
struct Config {
    api_key: Option<String>,
    url: Option<String>,
    model: Option<String>,
    language: Option<String>,
    prompt: Option<String>,
}

fn get_config_path() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", "github", "ai-commit")
        .expect("Failed to get project directories");
    let config_dir = proj_dirs.config_dir();
    if !config_dir.exists() {
        fs::create_dir_all(config_dir).expect("Failed to create config directory");
    }
    config_dir.join("config.toml")
}

fn load_config() -> Config {
    let config_path = get_config_path();
    if !config_path.exists() {
        return Config::default();
    }
    let config_str = fs::read_to_string(config_path).expect("Failed to read config file");
    toml::from_str(&config_str).expect("Failed to parse config file")
}

fn save_config(config: &Config) {
    let config_path = get_config_path();
    let config_str = toml::to_string_pretty(config).expect("Failed to serialize config");
    fs::write(config_path, config_str).expect("Failed to write config file");
}

// ===================================================================
// Command-line Interface
// ===================================================================

#[derive(Parser, Debug)]
#[clap(author, version, about = "AI-powered commit message generator.", long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(short, long, help = "Language for the commit message. Overrides config.")]
    language: Option<String>,

    #[clap(short, long, help = "Custom prompt for the AI model. Overrides config.")]
    prompt: Option<String>,

    #[clap(long, help = "Custom URL for the AI model's API. Overrides config.")]
    url: Option<String>,

    #[clap(long, help = "The specific model to use for generation. Overrides config.")]
    model: Option<String>,

    #[clap(subcommand)]
    command: Option<SubCommand>,
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Manage configuration.
    Config(ConfigArgs),
}

#[derive(Parser, Debug)]
struct ConfigArgs {
    #[clap(subcommand)]
    command: ConfigCmd,
}

#[derive(Parser, Debug)]
enum ConfigCmd {
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


// ===================================================================
// Git & Editor Interaction
// ===================================================================

fn get_git_diff() -> String {
    let output = Command::new("git")
        .arg("diff")
        .arg("--staged")
        .output()
        .expect("failed to execute git diff");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn open_in_vim(commit_message: &str) -> String {
    let mut temp_file = tempfile::Builder::new()
        .prefix("COMMIT_MSG_")
        .suffix(".txt")
        .tempfile()
        .expect("Failed to create temporary file");

    write!(temp_file, "{}", commit_message).expect("Failed to write to temporary file");
    let temp_path = temp_file.path().to_str().unwrap();

    Command::new("vim")
        .arg(temp_path)
        .status()
        .expect("Failed to open Vim");

    fs::read_to_string(temp_path).expect("Failed to read from temporary file")
}

// ===================================================================
// AI Commit Message Generation
// ===================================================================

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct OpenAiResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

async fn generate_commit_message(
    diff: &str,
    api_key: &str,
    language: &str,
    prompt: &str,
    url: &str,
    model: &str,
) -> Result<String, String> {
    let client = Client::new();

    let system_prompt = format!(
        "You are a helpful assistant that generates commit messages in {}. \
        The user will provide a git diff, and you should generate a concise and informative commit message. {}",
        language,
        prompt
    );

    let user_prompt = format!("Here is the git diff:\n```\n{}\n```", diff);

    let request = OpenAiRequest {
        model: model.to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_prompt,
            },
            Message {
                role: "user".to_string(),
                content: user_prompt,
            },
        ],
    };

    let res = client
        .post(url)
        .bearer_auth(api_key)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let status = res.status();
    let body = res.text().await.map_err(|e| format!("Failed to read response body: {}", e))?;

    if status.is_success() {
        match serde_json::from_str::<OpenAiResponse>(&body) {
            Ok(response_json) => {
                if response_json.choices.is_empty() {
                    Err("API response is empty.".to_string())
                } else {
                    Ok(response_json.choices[0].message.content.clone())
                }
            }
            Err(e) => {
                Err(format!(
                    "Failed to parse JSON response: {}. \nRaw response: {}",
                    e,
                    body
                ))
            }
        }
    } else {
        Err(format!(
            "API request failed with status {}. \nResponse: {}",
            status,
            body
        ))
    }
}

// ===================================================================
// Main Execution
// ===================================================================

fn handle_config_command(cmd: ConfigCmd) {
    let mut config = load_config();
    match cmd {
        ConfigCmd::SetApiKey { key } => {
            config.api_key = Some(key);
            save_config(&config);
            println!("API key set successfully.");
        }
        ConfigCmd::SetUrl { url } => {
            config.url = Some(url);
            save_config(&config);
            println!("API URL set to: {}", config.url.as_deref().unwrap());
        }
        ConfigCmd::SetModel { model } => {
            config.model = Some(model);
            save_config(&config);
            println!("Default model set to: {}", config.model.as_deref().unwrap());
        }
        ConfigCmd::SetLanguage { lang } => {
            config.language = Some(lang);
            save_config(&config);
            println!("Default language set to: {}", config.language.as_deref().unwrap());
        }
        ConfigCmd::SetPrompt { prompt } => {
            config.prompt = Some(prompt);
            save_config(&config);
            println!("Default prompt set.");
        }
        ConfigCmd::Show => {
            println!("Current configuration file path: {}", get_config_path().display());
            println!("---");
            if let Some(_api_key) = &config.api_key {
                 println!("api_key = [set]");
            } else {
                 println!("api_key = [not set]");
            }
            if let Some(url) = &config.url {
                println!("url = \"{}\"", url);
            }
            if let Some(model) = &config.model {
                println!("model = \"{}\"", model);
            }
            if let Some(language) = &config.language {
                println!("language = \"{}\"", language);
            }
            if let Some(prompt) = &config.prompt {
                println!("prompt = \"{}\"", prompt);
            }
            println!("---");
        }
    }
}

async fn run_generate(args: Cli, config: Config) {
    let api_key = match config.api_key {
        Some(key) => key,
        None => {
            eprintln!("API key not set. Please run `ai_commit config set-api-key <YOUR_KEY>`");
            return;
        }
    };

    let language = args.language.or(config.language).unwrap_or_else(|| "en".to_string());
    let prompt = args.prompt.or(config.prompt).unwrap_or_default();
    let url = args.url.or(config.url).unwrap_or_else(|| "https://api.openai.com/v1/chat/completions".to_string());
    let model = args.model.or(config.model).unwrap_or_else(|| "gpt-3.5-turbo".to_string());


    let diff = get_git_diff();
    if diff.is_empty() {
        println!("No staged changes to commit.");
        return;
    }

    match generate_commit_message(&diff, &api_key, &language, &prompt, &url, &model).await {
        Ok(commit_message) => {
            let final_message = open_in_vim(&commit_message);
            if !final_message.trim().is_empty() {
                Command::new("git")
                    .arg("commit")
                    .arg("-m")
                    .arg(&final_message)
                    .status()
                    .expect("Failed to execute git commit");
                println!("Commit successful.");
            } else {
                println!("Commit aborted because the message is empty.");
            }
        }
        Err(e) => {
            eprintln!("Error generating commit message:\n{}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config = load_config();

    match cli.command {
        Some(SubCommand::Config(config_args)) => {
            handle_config_command(config_args.command);
        }
        None => {
            run_generate(cli, config).await;
        }
    }
}
