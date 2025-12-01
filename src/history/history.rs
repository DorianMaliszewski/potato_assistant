use std::{collections::HashMap, fs::create_dir, path::PathBuf};

use crate::AiMessage;

#[derive(Debug)]
pub struct History {
    pub conversations: HashMap<String, Vec<AiMessage>>,
}

pub fn get_history() -> History {
    let config_path = get_history_folder_path();
    if !config_path.exists() {
        match create_dir(config_path) {
            Ok(_) => {
                println!("History folder created");
            }
            Err(e) => {
                println!("Cannot create history folder: {}", e);
            }
        }
    }

    History {
        conversations: HashMap::new(),
    }
}

fn get_history_folder_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".potato_history")
}
