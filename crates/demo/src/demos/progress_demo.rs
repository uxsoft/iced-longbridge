use iced::{
    Element, Length,
    widget::{column, row, text, Space},
};

use crate::{
    Message, State,
    components::{progress::progress, button::{button_ex, Variant}},
    theme::{AppTheme, Size},
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);

    let bar = column![
        row![
            progress(theme, state.progress_value).length(Length::Fixed(400.0)),
            text(format!("{:.0}%", state.progress_value)).size(13.0).color(theme.muted_foreground),
        ]
        .spacing(12)
        .align_y(iced::alignment::Vertical::Center),
    ];

    let controls = row![
        button_ex(theme, "-10%", Variant::Secondary, Size::Sm, Some(Message::ProgressChanged(-10.0)), false, false),
        button_ex(theme, "+10%", Variant::Secondary, Size::Sm, Some(Message::ProgressChanged(10.0)), false, false),
        button_ex(theme, "Reset", Variant::Outline, Size::Sm, Some(Message::ProgressReset), false, false),
    ]
    .spacing(8);

    column![
        header("Progress"),
        bar,
        Space::new().height(Length::Fixed(8.0)),
        controls,
        Space::new().height(Length::Fixed(16.0)),
        header("Examples"),
        progress(theme, 25.0).length(Length::Fixed(400.0)),
        progress(theme, 50.0).length(Length::Fixed(400.0)),
        progress(theme, 75.0).length(Length::Fixed(400.0)),
        progress(theme, 100.0).length(Length::Fixed(400.0)),
    ]
    .spacing(12)
    .into()
}
