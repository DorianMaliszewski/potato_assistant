use std::{fs, path::PathBuf};

use dotenvy::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum AiProvider {
    OpenAI,
    Gemini,
    Custom,
}

#[derive(Debug, Deserialize)]
pub struct UserConfig {
    pub api_key: Option<String>,
    pub language: Option<String>,
    pub volume: Option<u8>,
    pub debug_mode: Option<bool>,
    pub ai_provider: Option<AiProvider>,
    pub ai_model: Option<String>,
    pub ai_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AppConfig {
    pub api_key: String,
    pub ai_url: String,
    pub language: String,
    pub volume: u8,
    pub debug_mode: bool,
    pub ai_model: String,
    pub ai_provider: AiProvider,
}

impl AppConfig {
    pub fn override_with(self, user_config: UserConfig) -> Self {
        Self {
            api_key: user_config.api_key.unwrap_or(self.api_key),
            language: user_config.language.unwrap_or(self.language),
            volume: user_config.volume.unwrap_or(self.volume),
            debug_mode: user_config.debug_mode.unwrap_or(self.debug_mode),
            ai_url: user_config.ai_url.unwrap_or(self.ai_url),
            ai_model: user_config.ai_model.unwrap_or(self.ai_model),
            ai_provider: user_config.ai_provider.unwrap_or(self.ai_provider),
        }
    }
}

fn get_default_config() -> AppConfig {
    AppConfig {
        ai_url: "".to_string(),
        api_key: "".to_string(),
        ai_model: "".to_string(),
        ai_provider: AiProvider::Custom,
        language: "en".to_string(),
        volume: 50,
        debug_mode: false,
    }
}

pub fn get_config() -> AppConfig {
    let mut config: AppConfig = match load_env() {
        Ok(config) => config,
        Err(e) => {
            println!("{}", e);
            get_default_config()
        }
    };
    match read_from_home_config_file() {
        Ok(user_config) => {
            config = config.override_with(user_config);
        }
        Err(e) => {
            println!("{}", e);
        }
    };

    config
}

pub fn load_env() -> Result<AppConfig, String> {
    dotenv().ok();

    match envy::from_env::<AppConfig>() {
        Ok(config) => Ok(config),
        Err(e) => Err(format!("Erreur de configuration : {}", e)),
    }
}

fn read_from_home_config_file() -> Result<UserConfig, String> {
    let config_path = get_config_path();
    if !config_path.exists() {
        return Err(format!(
            "Fichier de config introuvable ici : {}",
            config_path.display()
        ));
    }

    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Format de config invalide : {}", e))?;

    let config: UserConfig =
        toml::from_str(&content).map_err(|e| format!("Format de config invalide : {}", e))?;

    Ok(config)
}

pub fn get_config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".potato_config")
}
