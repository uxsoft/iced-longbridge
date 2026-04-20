use iced::{Element, widget::{column, text}};

use crate::{
    Message,
    components::title_bar::title_bar,
    demos::common::{section_caption, section_title},
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let simple = title_bar::<Message>(
        theme,
        "Iced ✦ Longbridge",
        None,
        Some(Message::NoOp),
        Some(Message::NoOp),
        Some(Message::NoOp),
    );

    let with_trailing = title_bar(
        theme,
        "Project: iced-longbridge",
        Some(text("12:34").size(12.0).color(theme.muted_foreground).into()),
        Some(Message::NoOp),
        Some(Message::NoOp),
        Some(Message::NoOp),
    );

    column![
        section_title(theme, "Standard title bar"),
        simple,
        section_title(theme, "With trailing slot"),
        with_trailing,
        section_caption(theme, "The main app's title bar in main.rs uses a simpler variant."),
    ]
    .spacing(14)
    .into()
}
