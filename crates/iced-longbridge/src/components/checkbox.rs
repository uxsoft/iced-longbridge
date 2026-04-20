//! Checkbox component.

use iced::{
    Background, Border,
    widget::{checkbox, Checkbox},
};

use crate::theme::AppTheme;

pub fn checkbox_styled<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    label: impl Into<String>,
    is_checked: bool,
) -> Checkbox<'a, Message> {
    let t = *theme;
    checkbox(is_checked).label(label.into()).size(16).spacing(8).text_size(14).style(move |_, status| {
        use checkbox::Status::*;
        let (bg, border, icon) = match status {
            Active { is_checked } => {
                if is_checked {
                    (t.primary, t.primary, t.primary_foreground)
                } else {
                    (t.background, t.border, t.primary_foreground)
                }
            }
            Hovered { is_checked } => {
                if is_checked {
                    (t.primary_hover, t.primary, t.primary_foreground)
                } else {
                    (t.muted, t.muted_foreground, t.primary_foreground)
                }
            }
            Disabled { is_checked } => {
                let bg = if is_checked {
                    crate::theme::with_alpha(t.primary, 0.5)
                } else {
                    t.muted
                };
                (bg, t.border, t.primary_foreground)
            }
        };
        checkbox::Style {
            background: Background::Color(bg),
            icon_color: icon,
            border: Border {
                color: border,
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Some(t.foreground),
        }
    })
}
