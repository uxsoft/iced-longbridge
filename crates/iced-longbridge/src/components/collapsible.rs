//! Collapsible — a single expand/collapse panel with a header.

use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    alignment::Vertical,
    widget::{button, column, container, row, text, Space},
};

use crate::{
    components::icon::{icon, lucide},
    theme::AppTheme,
};

pub fn collapsible<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    label: impl Into<String>,
    expanded: bool,
    on_toggle: Message,
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    let t = *theme;
    let glyph = if expanded {
        lucide::chevron_down()
    } else {
        lucide::chevron_right()
    };

    let header = button(
        row![
            icon(theme, glyph, 14.0),
            text(label.into()).size(14.0).color(t.foreground),
            Space::new().width(Length::Fill),
        ]
        .spacing(8)
        .align_y(Vertical::Center),
    )
    .padding(Padding::from([8.0, 12.0]))
    .width(Length::Fill)
    .on_press(on_toggle)
    .style(move |_, status| {
        use button::Status::*;
        let bg = match status {
            Hovered => t.accent,
            Pressed => t.muted,
            _ => iced::Color::TRANSPARENT,
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: t.foreground,
            border: Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        }
    });

    let body: Element<'a, Message> = if expanded {
        container(content)
            .padding(Padding::from([4.0, 12.0]))
            .width(Length::Fill)
            .into()
    } else {
        Space::new().height(Length::Fixed(0.0)).into()
    };

    column![header, body].spacing(0).into()
}
