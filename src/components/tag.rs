//! Tag component — like a badge but with squared corners and used for labels.

use iced::{
    Background, Border, Element, Padding, Shadow,
    widget::{container, text},
};

use crate::theme::AppTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagVariant {
    #[default]
    Default,
    Secondary,
    Outline,
    Danger,
    Success,
    Warning,
    Info,
}

pub fn tag<'a, Message: 'a>(
    theme: &AppTheme,
    label: impl Into<String>,
    variant: TagVariant,
) -> Element<'a, Message> {
    let t = *theme;
    let label = label.into();
    let (bg, fg, border) = match variant {
        TagVariant::Default => (Some(t.muted), t.foreground, t.border),
        TagVariant::Secondary => (Some(t.secondary), t.secondary_foreground, t.secondary),
        TagVariant::Outline => (None, t.foreground, t.border),
        TagVariant::Danger => (
            Some(crate::theme::with_alpha(t.danger, 0.15)),
            t.danger,
            crate::theme::with_alpha(t.danger, 0.3),
        ),
        TagVariant::Success => (
            Some(crate::theme::with_alpha(t.success, 0.15)),
            t.success,
            crate::theme::with_alpha(t.success, 0.3),
        ),
        TagVariant::Warning => (
            Some(crate::theme::with_alpha(t.warning, 0.15)),
            t.warning,
            crate::theme::with_alpha(t.warning, 0.3),
        ),
        TagVariant::Info => (
            Some(crate::theme::with_alpha(t.info, 0.15)),
            t.info,
            crate::theme::with_alpha(t.info, 0.3),
        ),
    };
    container(text(label).size(12.0).color(fg))
        .padding(Padding::from([2.0, 8.0]))
        .style(move |_| container::Style {
            background: bg.map(Background::Color),
            text_color: Some(fg),
            border: Border {
                color: border,
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}
