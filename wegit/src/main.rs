use clap::{Parser, Subcommand};

mod components;

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
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::SetupConfig => components::configsetup::setup_global_config(),
        Commands::Init{ directory_name } => components::init::initialize_repository(directory_name),
    }

}