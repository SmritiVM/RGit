use std::fs;
use std::io::{self, Write};
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
    let _= io::stdout().flush();
    let mut input = String::new();
    let _= io::stdin().read_line(&mut input);
    Ok(input.trim().to_string())
}

pub fn setup_global_config(){
    let home = home_dir().expect("Could not find home directory");
    let config_path = home.join(".newconfig");

    if config_path.exists() {
       return
    }

    println!("First-time setup. Please enter your details:");
    let name = get_user_input("Enter your name: ");
    let email = get_user_input("Enter your email: ");

    let config = GitConfig {
        user: UserConfig { name: name.expect("Name not available"), email: email.expect("Email not available") },
    };

    let toml_string = toml::to_string(&config).expect("Failed to serialize config");
    let _= fs::write(config_path, toml_string);
    println!("Global config created successfully!");
}
