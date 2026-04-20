//! Toggle button — stateful button that persists a pressed look when active.

use iced::{
    Background, Border, Element, Padding, Shadow,
    alignment::Vertical,
    widget::{button, row, text},
};

use crate::{
    components::icon::{icon, IconName},
    theme::{AppTheme, Size},
};

pub fn toggle_button<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    label: impl Into<String>,
    active: bool,
    on_toggle: Message,
) -> Element<'a, Message> {
    toggle_button_ex(theme, label, None, active, Size::Md, on_toggle)
}

pub fn toggle_button_ex<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    label: impl Into<String>,
    leading_icon: Option<IconName>,
    active: bool,
    size: Size,
    on_toggle: Message,
) -> Element<'a, Message> {
    let t = *theme;
    let radius = size.radius();
    let text_size = size.font_size();
    let px = size.padding_x();
    let h = size.height();
    let label = label.into();

    let mut inner = row![].spacing(8).align_y(Vertical::Center);
    if let Some(name) = leading_icon {
        inner = inner.push(icon(theme, name, text_size));
    }
    inner = inner.push(text(label).size(text_size));

    button(inner)
        .padding(Padding::from([0.0, px]))
        .height(iced::Length::Fixed(h))
        .on_press(on_toggle)
        .style(move |_, status| {
            use button::Status::*;
            let (bg, fg, border) = if active {
                (t.muted, t.foreground, t.primary)
            } else {
                match status {
                    Hovered => (t.accent, t.foreground, t.border),
                    Pressed => (t.muted, t.foreground, t.border),
                    _ => (t.background, t.foreground, t.border),
                }
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: fg,
                border: Border {
                    color: border,
                    width: 1.0,
                    radius: radius.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            }
        })
        .into()
}
