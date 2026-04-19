use iced::{
    Element,
    widget::{column, row, text, Space},
};

use crate::{
    Message,
    components::{button::{button_ex, Variant}, tooltip::wrap},
    theme::{AppTheme, Size},
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);

    let row1 = row![
        wrap(theme, button_ex(theme, "Hover me", Variant::Primary, Size::Md, Some(Message::NoOp), false, false), "A friendly tooltip"),
        wrap(theme, button_ex(theme, "Info", Variant::Secondary, Size::Md, Some(Message::NoOp), false, false), "More information about this action"),
        wrap(theme, button_ex(theme, "Danger", Variant::Danger, Size::Md, Some(Message::NoOp), false, false), "Destructive action — be careful!"),
    ]
    .spacing(8);

    column![
        header("Tooltips"),
        row1,
        Space::new().height(iced::Length::Fixed(8.0)),
        text("Hover any button above to see its tooltip.").size(13.0).color(theme.muted_foreground),
    ]
    .spacing(12)
    .into()
}
