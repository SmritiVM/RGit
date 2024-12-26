use clap::{Parser, Subcommand};

mod components;
mod structures;
mod utils;

#[derive(Parser, Debug)]
#[command(name = "wegit", version = "0.1.0", about = "Git in Rust")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    SetupConfig,
    Init{
        #[clap(default_value_t = String::new())]
        directory_name: String,
    },
    Add{
        file_name: String,
    },
    Commit{
        commit_message: String,
    },
    Log,
    Checkout{
        commit_id: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::SetupConfig => components::configsetup::setup_global_config(),
        Commands::Init{ directory_name } => components::init::initialize_repository(directory_name),
        Commands::Add{ file_name } => components::add::add(&file_name),
        Commands::Commit { commit_message } => components::commit::commit_changes(&commit_message),
        Commands::Log => components::log::log_commits(),
        Commands::Checkout { commit_id } => components::checkout::checkout(&commit_id),
    }

}