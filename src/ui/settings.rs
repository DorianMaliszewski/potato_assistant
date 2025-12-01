use iced::{
    Alignment, Element, Length,
    widget::{button, column, row, text, text_editor},
};

use crate::{
    PotatoApp,
    ui::{messages::UIMessage, views::AppView},
};

pub fn get_settings_view(state: &PotatoApp) -> Element<'_, UIMessage> {
    column![
        row![
            text("Settings").size(30).width(Length::Fill),
            button("Back to chat").on_press(UIMessage::ChangeView(AppView::Chat))
        ]
        .spacing(10)
        .align_y(Alignment::Center),
        text_editor(&state.user_settings)
            .height(Length::Fill)
            .on_action(UIMessage::HandleSettingsInput),
        text(&state.input_error),
        button("Save settings").on_press(UIMessage::SaveSettings)
    ]
    .spacing(10)
    .padding(10)
    .into()
}
