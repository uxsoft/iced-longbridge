//! Input (text_input) component styled to match gpui-component.

use iced::{
    Background, Border,
    widget::{text_input, TextInput},
};

use crate::theme::{AppTheme, Size};

pub fn input<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    placeholder: &str,
    value: &str,
) -> TextInput<'a, Message> {
    styled(theme, Size::Md, placeholder, value)
}

pub fn input_sized<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    size: Size,
    placeholder: &str,
    value: &str,
) -> TextInput<'a, Message> {
    styled(theme, size, placeholder, value)
}

fn styled<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    size: Size,
    placeholder: &str,
    value: &str,
) -> TextInput<'a, Message> {
    let t = *theme;
    let radius = size.radius();
    let font_size = size.font_size();
    let padding = size.padding_x() / 2.0;

    text_input(placeholder, value)
        .padding(padding)
        .size(font_size)
        .style(move |_, status| input_style(&t, status, radius))
}

fn input_style(t: &AppTheme, status: text_input::Status, radius: f32) -> text_input::Style {
    use text_input::Status::*;
    let (border_color, border_width) = match status {
        Focused { .. } => (t.ring, 2.0),
        Hovered => (t.muted_foreground, 1.0),
        Disabled => (t.input_border, 1.0),
        Active => (t.input_border, 1.0),
    };
    let bg = match status {
        Disabled => t.muted,
        _ => t.background,
    };
    text_input::Style {
        background: Background::Color(bg),
        border: Border {
            color: border_color,
            width: border_width,
            radius: radius.into(),
        },
        icon: t.muted_foreground,
        placeholder: t.muted_foreground,
        value: t.foreground,
        selection: crate::theme::with_alpha(t.primary, 0.3),
    }
}
