use std::collections::HashMap;

use iced::{
    Element, Task, Theme,
    widget::{container, text_editor},
};

use crate::{
    config::{AppConfig, get_config},
    history::history::{History, get_history},
    ui::{
        chat::get_chat_view, messages::UIMessage, new_conversation::get_new_conversation_view,
        settings::get_settings_view, views::AppView,
    },
};

mod config;
mod history;
mod ui;

#[derive(Debug, PartialEq)]
pub enum AiMessageFrom {
    User,
    System,
    Assistant,
}

#[derive(Debug)]
pub struct AiMessage {
    pub from: AiMessageFrom,
    pub content: String,
}

#[derive(Debug)]
struct PotatoApp {
    pub config: AppConfig,
    pub view: AppView,
    pub user_input: String,
    pub new_conv_input: String,
    pub messages: Vec<AiMessage>,
    pub user_settings: text_editor::Content,
    pub input_error: String,
    pub history: History,
}

impl Default for PotatoApp {
    fn default() -> Self {
        Self {
            config: get_config(),
            view: AppView::Chat,
            user_input: "".to_string(),
            new_conv_input: "".to_string(),
            messages: vec![],
            user_settings: text_editor::Content::new(),
            input_error: "".to_string(),
            history: get_history(),
        }
    }
}

impl PotatoApp {
    fn update(&mut self, message: UIMessage) -> Task<UIMessage> {
        match message {
            UIMessage::None => Task::none(),
            UIMessage::UserInputHandle(new_value) => {
                self.user_input = new_value;
                Task::none()
            }
            UIMessage::SendMessage => {
                self.messages.push(AiMessage {
                    from: AiMessageFrom::User,
                    content: self.user_input.clone(),
                });
                self.messages.push(AiMessage {
                    from: AiMessageFrom::Assistant,
                    content: "Roger".to_string(),
                });
                self.user_input = "".to_string();
                Task::none()
            }
            UIMessage::ChangeView(new_view) => {
                if new_view == AppView::Settings {
                    let config_str = match toml::to_string_pretty(&self.config.clone()) {
                        Ok(str) => str,
                        Err(_) => "".to_string(),
                    };
                    self.user_settings = text_editor::Content::with_text(config_str.as_str());
                };
                self.view = new_view;
                Task::none()
            }
            UIMessage::SaveSettings => {
                self.input_error = "".to_string();
                match toml::from_str(&self.user_settings.text()) {
                    Ok(new_config) => {
                        self.config = new_config;
                    }
                    Err(e) => {
                        self.input_error = format!("Error when parsing new config {}", e);
                    }
                };
                Task::none()
            }
            UIMessage::HandleSettingsInput(action) => {
                self.user_settings.perform(action);
                Task::none()
            }
            UIMessage::NewConversation => {
                self.view = AppView::NewConversation;
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, UIMessage> {
        let content = match self.view {
            AppView::Chat => get_chat_view(self),
            AppView::Settings => get_settings_view(self),
            AppView::NewConversation => get_new_conversation_view(self),
        };

        container(content).into()
    }
}

fn main() -> iced::Result {
    iced::application("Potato Assistant", PotatoApp::update, PotatoApp::view)
        .theme(|_| Theme::Dark)
        .run()
}
