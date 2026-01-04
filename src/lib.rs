pub mod ai_commit;
pub mod cli;
pub mod config;

use std::process::Command;

use crate::cli::{Cli, ConfigCmd};
use crate::config::{Config, get_config_path};

pub async fn run_generate(args: Cli, config: Config) {
    let api_key = match config.api_key {
        Some(key) => key,
        None => {
            eprintln!("API key not set. Please run `ai_commit config set-api-key <YOUR_KEY>`");
            return;
        }
    };

    let language = args
        .language
        .or(config.language)
        .unwrap_or_else(|| "en".to_string());
    let prompt = args.prompt.or(config.prompt).unwrap_or_default();
    let url = args
        .url
        .or(config.url)
        .unwrap_or_else(|| "https://api.openai.com/v1/chat/completions".to_string());
    let model = args
        .model
        .or(config.model)
        .unwrap_or_else(|| "gpt-3.5-turbo".to_string());

    let diff = get_git_diff();
    if diff.is_empty() {
        println!("No staged changes to commit.");
        return;
    }

    match ai_commit::generate_commit_message(&diff, &api_key, &language, &prompt, &url, &model)
        .await
    {
        Ok(commit_message) => {
            println!("{}", commit_message);
        }
        Err(e) => {
            eprintln!("Error generating commit message:\n{}", e);
        }
    }
}

fn get_git_diff() -> String {
    let output = Command::new("git")
        .arg("diff")
        .arg("--staged")
        .output()
        .expect("failed to execute git diff");

    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn handle_config_command(cmd: ConfigCmd, mut config: Config) {
    match cmd {
        ConfigCmd::SetApiKey { key } => {
            config.api_key = Some(key);
            config.save_config();
            println!("API key set successfully.");
        }
        ConfigCmd::SetUrl { url } => {
            config.url = Some(url);
            config.save_config();
            println!("API URL set to: {}", config.url.as_deref().unwrap());
        }
        ConfigCmd::SetModel { model } => {
            config.model = Some(model);
            config.save_config();
            println!("Default model set to: {}", config.model.as_deref().unwrap());
        }
        ConfigCmd::SetLanguage { lang } => {
            config.language = Some(lang);
            config.save_config();
            println!(
                "Default language set to: {}",
                config.language.as_deref().unwrap()
            );
        }
        ConfigCmd::SetPrompt { prompt } => {
            config.prompt = Some(prompt);
            config.save_config();
            println!("Default prompt set.");
        }
        ConfigCmd::Show => {
            println!(
                "Current configuration file path: {}",
                get_config_path().display()
            );
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
