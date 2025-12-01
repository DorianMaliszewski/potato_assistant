use iced::{
    Element,
    widget::{button, column, container, row, text, text_input},
};

use crate::{PotatoApp, ui::messages::UIMessage};

pub fn get_new_conversation_view(state: &PotatoApp) -> Element<'_, UIMessage> {
    let content = column![
        text("New conversation"),
        text_input("Enter conversation name", &state.new_conv_input),
        row![
            button("Cancel").on_press(UIMessage::CancelNewConversation),
            button("Create").on_press(UIMessage::SubmitNewConversation)
        ]
    ];

    container(content).into()
}
