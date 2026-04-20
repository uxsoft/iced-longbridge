use iced::{
    Element,
    widget::{column, row, text, Space},
};

use crate::{
    Message, State,
    components::rating::rating,
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);

    column![
        header("Rating"),
        rating(theme, state.rating_value, 5, Message::RatingChanged),
        Space::new().height(iced::Length::Fixed(4.0)),
        row![
            text("You rated:").size(13.0).color(theme.muted_foreground),
            text(format!("{} / 5", state.rating_value)).size(13.0).color(theme.foreground),
        ]
        .spacing(6),
        Space::new().height(iced::Length::Fixed(8.0)),
        header("Read only"),
        rating(theme, 4, 5, |_| Message::NoOp),
        rating(theme, 3, 5, |_| Message::NoOp),
        rating(theme, 2, 5, |_| Message::NoOp),
    ]
    .spacing(10)
    .into()
}
