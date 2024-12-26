use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path};
use std::env;
use dirs::home_dir;
use toml::ser::to_string_pretty;
use crate::utils::message_handler::handle_message;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize)]
struct GitConfig {
    core: CoreConfig,
    user: Option<UserConfig>,
}

#[derive(Serialize)]
struct CoreConfig {
    repositoryname: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserConfig {
    name: String,
    email: String,
}

pub fn initialize_repository(directory_name: String) {
    let directory_name = if directory_name.is_empty() {
        env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf()).to_str().unwrap().to_string()
    } else {
        directory_name
    };

    if let Err(_) = fs::create_dir_all(&directory_name) {
        handle_message("Failed to create directory");
        return;
    }

    let wegit_dir = Path::new(&directory_name).join(".wegit");
    if let Err(_) = fs::create_dir_all(&wegit_dir) {
        handle_message("Failed to create .wegit directory");
        return;
    }

    let head_path = wegit_dir.join("HEAD");
    if let Err(_) = create_head_file(&head_path) {
        handle_message("Failed to create HEAD file");
        return;
    }

    let config_path = wegit_dir.join("config.toml");
    if let Err(_) = create_config_file(&config_path, &directory_name) {
        handle_message("Failed to create config.toml");
        return;
    }

    let file_objects_dir = wegit_dir.join("file_objects");
    if let Err(_) = fs::create_dir_all(&file_objects_dir) {
        handle_message("Failed to create file objects directory");
        return;
    }

    let index_objects_dir = wegit_dir.join("index_objects");
    if let Err(_) = fs::create_dir_all(&index_objects_dir) {
        handle_message("Failed to create index object directory");
        return;
    }

    handle_message("Initialized empty wegit repository");
}

fn create_config_file(config_path: &Path, directory_name: &str) -> io::Result<()> {
    let fetched_user = read_user_from_config_toml();
    let config = GitConfig {
        core: CoreConfig {
            repositoryname: directory_name.to_string(),
        },
        user: fetched_user,
    };

    let toml_string = to_string_pretty(&config).expect("Failed to serialize config to TOML");

    let mut config_file = File::create(config_path)?;
    config_file.write_all(toml_string.as_bytes())?;
    Ok(())
}

fn read_user_from_config_toml() -> Option<UserConfig> {
    let home = home_dir().expect("Could not find home directory");
    let config_path = home.join(".wegitconfig");

    let content = fs::read_to_string(config_path).expect("Failed to read .wegitconfig");

    let parsed: toml::Value = toml::de::from_str(&content).expect("Failed to parse .wegitconfig");

    parsed.get("user").and_then(|user_section| {
        let user_str = toml::ser::to_string(&user_section).ok()?; 
        toml::de::from_str(&user_str).ok()
    })
}

fn create_head_file(head_path: &Path) -> io::Result<()> {
    let mut head_file = File::create(head_path)?;
    head_file.write_all(b"0")?;
    Ok(())
}