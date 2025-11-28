use std::{fs, path::PathBuf};

use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
enum AiProvider {
    OpenAI,
    Gemini,
    Custom,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    pub ai_url: String,
    pub ai_endpoint: String,
    pub ai_model: String,
    pub ai_provider: AiProvider,
}

fn get_default_config() -> AppConfig { AppConfig {
            ai_url: "".to_string(),
            ai_endpoint: "".to_string(),
            ai_model: "".to_string(),
            ai_provider: AiProvider::Custom,
        }
}

pub fn get_config() -> AppConfig {
    let mut config: AppConfig = match load_env() {
        Ok(config) => config,
        Err(e) => AppConfig {
            ai_url: "".to_string(),
            ai_endpoint: "".to_string(),
            ai_model: "".to_string(),
            ai_provider: AiProvider::Custom,
        },
    };
    let main_path = get_config_path();
    match File

    config
}

pub fn load_env() -> Result<AppConfig, String> {
    dotenv().ok();

    match envy::from_env::<AppConfig>() {
        Ok(config) => Ok(config),
        Err(e) => Err(format!("Erreur de configuration : {}", e)),
    }
}

fn read_from_home_config_file() -> Result<AppConfig, String> {
    let config_path = get_config_path();
    if !config_path.exists() {
        return Err(format!("Fichier de config introuvable ici : {}", config_path.display()));
    }

    
    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Format de config invalide : {}", e))?;

    
    let config: AppConfig = toml::from_str(&content)
        .map_err(|e| format!("Format de config invalide : {}", e))?;

    Ok(config)
}

pub fn get_config_path() -> PathBuf {
    
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".potato_config")
}
