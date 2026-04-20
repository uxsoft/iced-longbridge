use iced::{
    Element,
    widget::{column, row, text, Space},
};

use crate::{
    Message, State,
    components::spinner::Spinner,
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);
    let r = state.tick_rotation;

    column![
        header("Sizes"),
        row![
            Spinner::new(theme, r).size(16.0).view(),
            Spinner::new(theme, r).size(24.0).view(),
            Spinner::new(theme, r).size(32.0).view(),
            Spinner::new(theme, r).size(48.0).view(),
        ]
        .spacing(16)
        .align_y(iced::alignment::Vertical::Center),
        Space::new().height(iced::Length::Fixed(8.0)),
        header("Colors"),
        row![
            Spinner::new(theme, r).size(32.0).color(theme.primary).view(),
            Spinner::new(theme, r).size(32.0).color(theme.danger).view(),
            Spinner::new(theme, r).size(32.0).color(theme.success).view(),
            Spinner::new(theme, r).size(32.0).color(theme.warning).view(),
            Spinner::new(theme, r).size(32.0).color(theme.info).view(),
        ]
        .spacing(16),
    ]
    .spacing(12)
    .into()
}
