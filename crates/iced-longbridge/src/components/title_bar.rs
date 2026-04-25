//! Title bar — app chrome with title, actions, and window controls.

use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    alignment::Vertical,
    widget::{button, container, row, text, Space},
};

use crate::{
    components::icon::{icon_colored, lucide, Icon},
    theme::AppTheme,
};

pub fn title_bar<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    title: impl Into<String>,
    trailing: Option<Element<'a, Message>>,
    on_minimize: Option<Message>,
    on_maximize: Option<Message>,
    on_close: Option<Message>,
) -> Element<'a, Message> {
    let t = *theme;
    let mut content = row![
        text(title.into()).size(14.0).color(t.foreground),
        Space::new().width(Length::Fill),
    ]
    .spacing(12)
    .align_y(Vertical::Center);

    if let Some(el) = trailing {
        content = content.push(el);
    }
    if let Some(msg) = on_minimize {
        content = content.push(window_button(t, lucide::minus(), msg, t.muted_foreground));
    }
    if let Some(msg) = on_maximize {
        content = content.push(window_button(t, lucide::square(), msg, t.muted_foreground));
    }
    if let Some(msg) = on_close {
        content = content.push(window_button(t, lucide::x(), msg, t.danger));
    }

    container(content)
        .padding(Padding::from([10.0, 14.0]))
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(t.background)),
            text_color: Some(t.foreground),
            border: Border {
                color: t.border,
                width: 0.0,
                radius: 0.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}

fn window_button<'a, Message: Clone + 'a>(
    t: AppTheme,
    glyph: Icon,
    on_press: Message,
    hover_color: iced::Color,
) -> Element<'a, Message> {
    button(icon_colored::<Message>(glyph, 12.0, t.foreground))
        .padding(Padding::from([4.0, 8.0]))
        .on_press(on_press)
        .style(move |_, status| {
            use button::Status::*;
            let (bg, fg) = match status {
                Hovered => (crate::theme::with_alpha(hover_color, 0.15), hover_color),
                Pressed => (crate::theme::with_alpha(hover_color, 0.25), hover_color),
                _ => (iced::Color::TRANSPARENT, t.muted_foreground),
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: fg,
                border: Border {
                    color: iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: 4.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            }
        })
        .into()
}
