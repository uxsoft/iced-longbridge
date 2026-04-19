//! Badge component — small inline status indicator.

use iced::{
    Background, Border, Element, Padding, Shadow,
    widget::{container, text},
};

use crate::theme::AppTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Outline,
    Danger,
    Success,
    Warning,
    Info,
}

pub fn badge<'a, Message: 'a>(
    theme: &AppTheme,
    label: impl Into<String>,
    variant: BadgeVariant,
) -> Element<'a, Message> {
    let t = *theme;
    let label = label.into();
    let (bg, fg, border) = match variant {
        BadgeVariant::Default => (Some(t.primary), t.primary_foreground, t.primary),
        BadgeVariant::Secondary => (Some(t.secondary), t.secondary_foreground, t.secondary),
        BadgeVariant::Outline => (None, t.foreground, t.border),
        BadgeVariant::Danger => (Some(t.danger), t.danger_foreground, t.danger),
        BadgeVariant::Success => (Some(t.success), t.success_foreground, t.success),
        BadgeVariant::Warning => (Some(t.warning), t.warning_foreground, t.warning),
        BadgeVariant::Info => (Some(t.info), t.info_foreground, t.info),
    };
    container(text(label).size(11.0).color(fg))
        .padding(Padding::from([2.0, 8.0]))
        .style(move |_| container::Style {
            background: bg.map(Background::Color),
            text_color: Some(fg),
            border: Border {
                color: border,
                width: if matches!(variant, BadgeVariant::Outline) { 1.0 } else { 0.0 },
                radius: 999.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}
