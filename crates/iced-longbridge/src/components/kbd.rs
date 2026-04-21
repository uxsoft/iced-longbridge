//! Kbd — keyboard shortcut display.

use iced::{
    Background, Border, Element, Padding, Shadow,
    widget::{container, row, text},
};

use crate::theme::AppTheme;

pub fn kbd<'a, Message: 'a>(
    theme: &AppTheme,
    key: impl Into<String>,
) -> Element<'a, Message> {
    let t = *theme;
    container(text(key.into()).size(11.0).color(t.muted_foreground))
        .padding(Padding::from([2.0, 6.0]))
        .style(move |_| container::Style {
            background: Some(Background::Color(t.muted)),
            text_color: Some(t.muted_foreground),
            border: Border {
                color: t.border,
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}

pub fn combo<'a, Message: 'a>(
    theme: &AppTheme,
    keys: &[&str],
) -> Element<'a, Message> {
    let mut r = row![].spacing(4);
    for (i, k) in keys.iter().enumerate() {
        if i > 0 {
            r = r.push(text("+").size(11.0).color(theme.muted_foreground));
        }
        r = r.push(kbd(theme, *k));
    }
    r.into()
}
