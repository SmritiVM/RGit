use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Serialize, Deserialize)]
struct GitConfig {
    user: UserConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserConfig {
    name: String,
    email: String,
}

fn get_user_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn setup_global_config() -> io::Result<GitConfig> {
    let home = home_dir().expect("Could not find home directory");
    let config_path = home.join(".newconfig");

    // If global config exists, read and return it
    if config_path.exists() {
        let content = fs::read_to_string(config_path)?;
        return Ok(toml::from_str(&content).expect("Failed to parse config"));
    }

    // If it doesn't exist, create it
    println!("First-time setup. Please enter your details:");
    let name = get_user_input("Enter your name: ")?;
    let email = get_user_input("Enter your email: ")?;

    let config = GitConfig {
        user: UserConfig { name, email },
    };

    let toml_string = toml::to_string(&config).expect("Failed to serialize config");
    fs::write(config_path, toml_string)?;
    println!("Global config created successfully!");

    Ok(config)
}

fn create_repository(repo_name: &str, global_config: &GitConfig) -> io::Result<()> {
    // Get current directory
    let current_dir = std::env::current_dir()?;
    
    // Create repository directory parallel to current directory
    let repo_path = current_dir.parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find parent directory"))?
        .join(repo_name);

    // Create the repository directory
    fs::create_dir_all(&repo_path)?;

    // Create config.toml inside the new repository
    let config_content = toml::to_string(&global_config).expect("Failed to serialize config");
    fs::write(repo_path.join("config.toml"), config_content)?;

    println!("Repository created at: {:?}", repo_path);
    println!("Config file created with user details from global config");

    Ok(())
}

fn main() -> io::Result<()> {
    // First, ensure global config exists or create it
    let global_config = setup_global_config()?;

    // Get repository name from command line args or use default
    let args: Vec<String> = std::env::args().collect();
    let repo_name = args.get(1).map(|s| s.as_str()).unwrap_or("new-repository");

    // Create the repository with config
    create_repository(repo_name, &global_config)?;

    Ok(())
}