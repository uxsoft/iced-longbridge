use iced::{
    Element, Length,
    widget::{column, row, text, Space},
};

use crate::{
    Message, State,
    components::slider::slider_styled,
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);

    column![
        header("Volume"),
        row![
            slider_styled(theme, 0.0..=100.0, state.slider_value, Message::SliderChanged)
                .width(Length::Fixed(400.0)),
            text(format!("{:.0}", state.slider_value)).size(13.0).color(theme.muted_foreground),
        ]
        .spacing(12)
        .align_y(iced::alignment::Vertical::Center),
        Space::new().height(Length::Fixed(16.0)),
        header("Brightness"),
        row![
            slider_styled(theme, 0.0..=1.0, state.slider_float, Message::SliderFloatChanged)
                .width(Length::Fixed(400.0)),
            text(format!("{:.2}", state.slider_float)).size(13.0).color(theme.muted_foreground),
        ]
        .spacing(12)
        .align_y(iced::alignment::Vertical::Center),
    ]
    .spacing(12)
    .into()
}
