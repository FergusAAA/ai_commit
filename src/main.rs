use ai_commit::cli::{Cli, SubCommand};
use ai_commit::config::load_config;
use ai_commit::{handle_config_command, run_generate_commit};
use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config = load_config();

    match cli.command {
        Some(SubCommand::Config(config_args)) => {
            handle_config_command(config_args.command, config);
        }
        None => {
            run_generate_commit(cli, config).await;
        }
    }
}
