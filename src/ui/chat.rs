use iced::{
    Alignment, Border, Element, Length,
    widget::{button, column, container, row, scrollable, text, text_input, vertical_rule},
};

use crate::{
    AiMessageFrom,
    ui::{
        messages::{self, UIMessage},
        views::AppView,
    },
};

pub fn get_chat_view(_state: &crate::PotatoApp) -> Element<'_, messages::UIMessage> {
    row![get_conversations_view(_state), get_right_view(_state)].into()
}

fn get_conversations_view(_state: &crate::PotatoApp) -> Element<'_, messages::UIMessage> {
    let history = column(_state.history.conversations.keys().map(|k| text(k).into()));
    container(row![
        column![
            text("Conversations").size(30),
            scrollable(history).height(Length::Fill),
            button("New conversation").on_press(UIMessage::NewConversation)
        ]
        .spacing(10)
        .padding(10)
        .height(Length::Fill),
        vertical_rule(1)
    ])
    .into()
}

fn get_right_view(state: &crate::PotatoApp) -> Element<'_, messages::UIMessage> {
    let messages_column = column(state.messages.iter().map(|m| {
        let mut msg = text(m.content.clone());
        if m.from == AiMessageFrom::User {
            msg = msg.align_x(Alignment::Start).width(Length::Fill);
        } else {
            msg = msg.align_x(Alignment::End).width(Length::Fill);
        }

        container(msg)
            .style(if m.from == AiMessageFrom::User {
                get_user_message_container_style
            } else {
                get_assistant_message_container_style
            })
            .padding(20)
            .into()
    }))
    .spacing(20)
    .width(Length::Fill);

    let mut mic_button = button("M Start").on_press(UIMessage::StartAudio);
    let mut chat_text_input = text_input("Enter your message", state.user_input.as_str());
    let mut send_button = button("Send");

    if state.audio_rec.recording {
        mic_button = button("M Stop").on_press(UIMessage::EndAudio);
    } else {
        chat_text_input = chat_text_input
            .on_input(UIMessage::UserInputHandle)
            .on_submit(UIMessage::SendMessage);
        send_button = send_button.on_press(UIMessage::SendMessage)
    }

    container(
        column![
            row![
                text("Chat").size(30).width(Length::Fill),
                button("Settings").on_press(UIMessage::ChangeView(AppView::Settings))
            ]
            .align_y(Alignment::Center),
            scrollable(messages_column).height(Length::Fill),
            row![chat_text_input, send_button, mic_button].spacing(10)
        ]
        .spacing(10)
        .padding(10)
        .height(Length::Fill),
    )
    .into()
}

fn get_user_message_container_style(theme: &iced::Theme) -> iced::widget::container::Style {
    let palette = theme.extended_palette();
    iced::widget::container::Style {
        background: Some(palette.primary.strong.color.into()),
        text_color: Some(palette.primary.strong.text),
        border: Border {
            radius: 5.into(),
            color: palette.primary.strong.color,
            width: 1.0,
        },
        ..Default::default()
    }
}

fn get_assistant_message_container_style(theme: &iced::Theme) -> iced::widget::container::Style {
    let palette = theme.extended_palette();
    iced::widget::container::Style {
        background: Some(palette.secondary.strong.color.into()),
        text_color: Some(palette.secondary.strong.text),
        border: Border {
            radius: 5.into(),
            color: palette.secondary.strong.color,
            width: 1.0,
        },
        ..Default::default()
    }
}
