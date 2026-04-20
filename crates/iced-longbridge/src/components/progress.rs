//! Progress bar.

use iced::{
    widget::{progress_bar, ProgressBar},
};

use crate::theme::AppTheme;

pub fn progress<'a>(
    theme: &AppTheme,
    value: f32,
) -> ProgressBar<'a> {
    let t = *theme;
    let value = value.clamp(0.0, 100.0);
    progress_bar(0.0..=100.0, value)
        .length(iced::Length::Fill)
        .girth(iced::Length::Fixed(8.0))
        .style(move |_| iced::widget::progress_bar::Style {
            background: t.muted.into(),
            bar: t.primary.into(),
            border: iced::Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: 999.0.into(),
            },
        })
}
