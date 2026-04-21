//! Reusable `iced` style builders.
//!
//! Most components in this library render containers, buttons, and text inputs
//! with near-identical boilerplate: a `Background::Color`, a `Border` with
//! theme-driven color and a radius, `Shadow::default()`, and `snap: true`.
//! Collecting those shapes here keeps component files focused on layout.

use iced::{
    Background, Border, Color, Shadow, Vector,
    widget::{button, container},
};

use crate::theme::AppTheme;

/// Build a [`button::Style`] with the library's defaults (no shadow, snap on).
pub fn button_style(
    bg: Option<Color>,
    fg: Color,
    border_color: Color,
    border_width: f32,
    radius: f32,
) -> button::Style {
    button::Style {
        background: bg.map(Background::Color),
        text_color: fg,
        border: Border {
            color: border_color,
            width: border_width,
            radius: radius.into(),
        },
        shadow: Shadow::default(),
        snap: true,
    }
}

/// Floating panel style used by popovers, toasts, and dialogs. Background is
/// the theme's `popover` surface, with a soft drop shadow.
pub fn popover_container(theme: &AppTheme, radius: f32) -> container::Style {
    container::Style {
        background: Some(Background::Color(theme.popover)),
        text_color: Some(theme.popover_foreground),
        border: Border {
            color: theme.border,
            width: 1.0,
            radius: radius.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
            offset: Vector::new(0.0, 6.0),
            blur_radius: 18.0,
        },
        snap: true,
    }
}

/// Popover style with a custom accent border color — used by toasts to tint
/// the border by notification kind.
pub fn popover_container_accent(
    theme: &AppTheme,
    accent: Color,
    radius: f32,
) -> container::Style {
    container::Style {
        background: Some(Background::Color(theme.popover)),
        text_color: Some(theme.popover_foreground),
        border: Border {
            color: accent,
            width: 1.0,
            radius: radius.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 14.0,
        },
        snap: true,
    }
}
