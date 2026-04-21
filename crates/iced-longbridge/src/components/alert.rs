//! Alert component — block-level status message.

use iced::{
    Element, Length, Padding,
    widget::{column, container, row, text},
};

use crate::{styles, theme::AppTheme};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlertVariant {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Danger,
}

pub fn alert<'a, Message: 'a>(
    theme: &AppTheme,
    variant: AlertVariant,
    title: impl Into<String>,
    description: impl Into<String>,
) -> Element<'a, Message> {
    let t = *theme;
    let title = title.into();
    let description = description.into();
    let (bg, fg, border_color, icon) = match variant {
        AlertVariant::Default => (t.muted, t.foreground, t.border, "•"),
        AlertVariant::Info => (
            crate::theme::with_alpha(t.info, 0.12),
            t.info,
            crate::theme::with_alpha(t.info, 0.4),
            "ℹ",
        ),
        AlertVariant::Success => (
            crate::theme::with_alpha(t.success, 0.12),
            t.success,
            crate::theme::with_alpha(t.success, 0.4),
            "✓",
        ),
        AlertVariant::Warning => (
            crate::theme::with_alpha(t.warning, 0.12),
            t.warning,
            crate::theme::with_alpha(t.warning, 0.4),
            "⚠",
        ),
        AlertVariant::Danger => (
            crate::theme::with_alpha(t.danger, 0.12),
            t.danger,
            crate::theme::with_alpha(t.danger, 0.4),
            "✕",
        ),
    };

    let body = column![
        text(title).size(14.0).color(fg),
        text(description).size(13.0).color(t.muted_foreground),
    ]
    .spacing(4);

    let content = row![text(icon).size(18.0).color(fg), body].spacing(12).align_y(iced::alignment::Vertical::Top);

    container(content)
        .padding(Padding::from([12.0, 16.0]))
        .width(Length::Fill)
        .style(move |_| styles::simple_container(Some(bg), fg, border_color, 1.0, 8.0))
        .into()
}
