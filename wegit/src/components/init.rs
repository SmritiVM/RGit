use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path};
use std::env;
use dirs::home_dir;
use toml::ser::to_string_pretty;
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

    if let Err(e) = fs::create_dir_all(&directory_name) {
        eprintln!("Failed to create directory {}: {}", directory_name, e);
        return;
    }

    let wegit_dir = Path::new(&directory_name).join(".wegit");
    if let Err(e) = fs::create_dir_all(&wegit_dir) {
        eprintln!("Failed to create .wegit directory: {}", e);
        return;
    }

    let config_path = wegit_dir.join("config.toml");
    if let Err(e) = create_config_file(&config_path, &directory_name) {
        eprintln!("Failed to create config.toml: {}", e);
        return;
    }

    if let Err(e) = create_index_file(&wegit_dir) {
        eprintln!("Failed to create tracking files in .wegit: {}", e);
        return;
    }

    let file_objects_dir = wegit_dir.join("file_objects");
    if let Err(e) = fs::create_dir_all(&file_objects_dir) {
        eprintln!("Failed to create file objects directory: {}", e);
        return;
    }

    let index_objects_dir = wegit_dir.join("index_objects");
    if let Err(e) = fs::create_dir_all(&index_objects_dir) {
        eprintln!("Failed to create index object directory: {}", e);
        return;
    }

    println!("Initialized empty wegit repository in '{}/.wegit'", directory_name);
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

fn create_index_file(wegit_dir: &Path) -> io::Result<()> {
    let index_file_path = wegit_dir.join("index.txt");
    File::create(index_file_path)?;

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
