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

use crate::theme::{AppTheme, with_alpha};

/// Scale a base accent colour by button status. Used by every tinted button
/// variant (Danger, Success, Warning, Info) to avoid re-writing the same
/// hover/pressed/disabled alpha ramp per component.
pub fn tinted_status(base: Color, status: button::Status) -> Color {
    use button::Status::*;
    match status {
        Hovered => with_alpha(base, 0.9),
        Pressed => with_alpha(base, 0.8),
        Disabled => with_alpha(base, 0.5),
        Active => base,
    }
}

/// Borderless icon button that tints its background on hover/press. Shared by
/// dialog, sheet, notification, and any other "X" / close control.
pub fn icon_button_style(theme: &AppTheme, status: button::Status, radius: f32) -> button::Style {
    use button::Status::*;
    let bg = match status {
        Hovered => theme.accent,
        Pressed => theme.muted,
        _ => Color::TRANSPARENT,
    };
    button_style(
        Some(bg),
        theme.muted_foreground,
        Color::TRANSPARENT,
        0.0,
        radius,
    )
}

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
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.0), // 0.25),
            offset: Vector::new(0.0, 6.0),
            blur_radius: 18.0,
        },
        snap: true,
    }
}

/// Flat container style (solid background, simple border, no shadow). Used by
/// alert / badge / tag / list row wrappers and anywhere the popover drop
/// shadow would be wrong.
pub fn simple_container(
    bg: Option<Color>,
    fg: Color,
    border_color: Color,
    border_width: f32,
    radius: f32,
) -> container::Style {
    container::Style {
        background: bg.map(Background::Color),
        text_color: Some(fg),
        border: Border {
            color: border_color,
            width: border_width,
            radius: radius.into(),
        },
        shadow: Shadow::default(),
        snap: true,
    }
}

/// Small floating panel used by tooltips (subtler drop shadow than a popover).
pub fn tooltip_container(theme: &AppTheme, radius: f32) -> container::Style {
    container::Style {
        background: Some(Background::Color(theme.popover)),
        text_color: Some(theme.popover_foreground),
        border: Border {
            color: theme.border,
            width: 1.0,
            radius: radius.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.0), //0.15),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 6.0,
        },
        snap: true,
    }
}

/// Popover style with a custom accent border color — used by toasts to tint
/// the border by notification kind.
pub fn popover_container_accent(theme: &AppTheme, accent: Color, radius: f32) -> container::Style {
    container::Style {
        background: Some(Background::Color(theme.popover)),
        text_color: Some(theme.popover_foreground),
        border: Border {
            color: accent,
            width: 1.0,
            radius: radius.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.0), // 0.25),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 14.0,
        },
        snap: true,
    }
}
