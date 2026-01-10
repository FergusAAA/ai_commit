use ai_commit::cli::{Cli, SubCommand};
use ai_commit::config::load_config;
use ai_commit::{handle_config_command, run_generate_commit};
use clap::{CommandFactory, Parser};
use clap_complete::{generate, Generator, Shell};
use std::env;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config = load_config();

    if let Some(maybe_shell) = cli.gen_completion {
        let shell_str = match maybe_shell {
            Some(s) => s,
            None => detect_current_shell().unwrap_or_else(|| {
                eprintln!("Error: Could not detect current shell");
                eprintln!("Please specify shell explicitly: --gen-completion <SHELL>");
                eprintln!("Supported shells: bash, zsh, fish, power-shell, elvish");
                std::process::exit(1);
            }),
        };
        generate_completion_script(&shell_str);
        return;
    }

    match cli.command {
        Some(SubCommand::Config(config_args)) => {
            handle_config_command(config_args.command, config);
        }
        None => {
            run_generate_commit(cli, config).await;
        }
    }
}

fn generate_completion_script(shell_str: &str) {
    let mut cmd = Cli::command();

    let shell = match shell_str.to_lowercase().as_str() {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        "fish" => Shell::Fish,
        "power-shell" | "powershell" | "ps" | "pwsh" => Shell::PowerShell,
        "elvish" => Shell::Elvish,
        _ => {
            eprintln!("Error: Unsupported shell '{}'", shell_str);
            eprintln!("Supported shells: bash, zsh, fish, power-shell, elvish");
            std::process::exit(1);
        }
    };

    let file_name = shell.file_name("ai_commit");
    let mut buf: Vec<u8> = Vec::new();

    generate(shell, &mut cmd, "ai_commit", &mut buf);

    std::fs::write(&file_name, buf).unwrap_or_else(|e| {
        eprintln!("Error writing completion script: {}", e);
        std::process::exit(1);
    });

    println!("Completion script generated: {}", file_name);
}

fn detect_current_shell() -> Option<String> {
    let shell_path = env::var("SHELL").ok()?;
    if shell_path.is_empty() {
        return None;
    }
    let shell_name = shell_path
        .rsplit('/')
        .next()
        .unwrap_or_else(|| &shell_path);

    let normalized = normalize_shell_name(shell_name);
    if matches!(normalized.as_str(), "bash" | "zsh" | "fish" | "powershell" | "elvish") {
        Some(normalized)
    } else {
        None
    }
}

fn normalize_shell_name(name: &str) -> String {
    let name = name.to_lowercase();
    match name.as_str() {
        "bash" => "bash".to_string(),
        "zsh" => "zsh".to_string(),
        "fish" => "fish".to_string(),
        "pwsh" | "powershell" => "powershell".to_string(),
        "elvish" => "elvish".to_string(),
        _ => name,
    }
}
