use iced::widget::text_editor;

use crate::ui::views::AppView;

#[derive(Debug, Clone)]
pub enum UIMessage {
    None,
    UserInputHandle(String),
    SendMessage,
    ChangeView(AppView),
    HandleSettingsInput(text_editor::Action),
    SaveSettings,
    NewConversation,
    SubmitNewConversation,
    CancelNewConversation,
    StartAudio,
    EndAudio,
}
