//! Button component — mirrors gpui-component's Button variants and sizes.

use iced::{
    Element, Length, Padding,
    widget::{button, text},
};

use crate::{
    styles,
    theme::{AppTheme, Size},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Variant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
    Danger,
    Success,
    Warning,
    Info,
    Link,
}

pub fn button_ex<'a, Message: 'a + Clone>(
    theme: &AppTheme,
    label: impl Into<String>,
    variant: Variant,
    size: Size,
    on_press: Option<Message>,
    loading: bool,
    disabled: bool,
) -> Element<'a, Message> {
    let theme = *theme;
    let text_size = size.font_size();
    let height = size.height();
    let padding_x = size.padding_x();
    let radius = size.radius();
    let label_text: String = label.into();
    let label_text = if loading {
        format!("⟳ {}", label_text)
    } else {
        label_text
    };

    let content = text(label_text)
        .size(text_size)
        .align_y(iced::alignment::Vertical::Center);

    let mut btn = button(content)
        .padding(Padding::from([0.0, padding_x]))
        .height(Length::Fixed(height))
        .style(move |_, status| variant_style(&theme, variant, status, radius));

    if !disabled
        && !loading
        && let Some(msg) = on_press
    {
        btn = btn.on_press(msg);
    }

    btn.into()
}

fn variant_style(
    t: &AppTheme,
    variant: Variant,
    status: button::Status,
    radius: f32,
) -> button::Style {
    use button::Status::*;
    let (bg, fg, border_color, border_width) = match variant {
        Variant::Primary => {
            let bg = match status {
                Hovered => t.primary_hover,
                Pressed => t.primary_active,
                Disabled => crate::theme::with_alpha(t.primary, 0.5),
                Active => t.primary,
            };
            (Some(bg), t.primary_foreground, t.primary, 0.0)
        }
        Variant::Secondary => {
            let bg = match status {
                Hovered => t.secondary_hover,
                Pressed => t.secondary_active,
                Disabled => crate::theme::with_alpha(t.secondary, 0.5),
                Active => t.secondary,
            };
            (Some(bg), t.secondary_foreground, t.secondary, 0.0)
        }
        Variant::Outline => {
            let bg = match status {
                Hovered => t.accent,
                Pressed => t.muted,
                _ => t.background,
            };
            (Some(bg), t.foreground, t.border, 1.0)
        }
        Variant::Ghost => {
            let bg = match status {
                Hovered => t.accent,
                Pressed => t.muted,
                _ => iced::Color::TRANSPARENT,
            };
            (Some(bg), t.foreground, iced::Color::TRANSPARENT, 0.0)
        }
        Variant::Danger => (
            Some(styles::tinted_status(t.danger, status)),
            t.danger_foreground,
            t.danger,
            0.0,
        ),
        Variant::Success => (
            Some(styles::tinted_status(t.success, status)),
            t.success_foreground,
            t.success,
            0.0,
        ),
        Variant::Warning => (
            Some(styles::tinted_status(t.warning, status)),
            t.warning_foreground,
            t.warning,
            0.0,
        ),
        Variant::Info => (
            Some(styles::tinted_status(t.info, status)),
            t.info_foreground,
            t.info,
            0.0,
        ),
        Variant::Link => {
            let fg = match status {
                Hovered | Pressed => t.link_hover,
                _ => t.link,
            };
            (None, fg, iced::Color::TRANSPARENT, 0.0)
        }
    };

    styles::button_style(bg, fg, border_color, border_width, radius)
}
