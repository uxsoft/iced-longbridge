//! Tabs — tab bar over switched content panels.

use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    alignment::Vertical,
    widget::{button, column, container, row, text, Space},
};

use crate::theme::AppTheme;

pub struct Tab<'a, Message> {
    pub label: String,
    pub content: Element<'a, Message>,
}

impl<'a, Message> Tab<'a, Message> {
    pub fn new(label: impl Into<String>, content: Element<'a, Message>) -> Self {
        Self {
            label: label.into(),
            content,
        }
    }
}

pub fn tab_bar<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    labels: &[String],
    selected: usize,
    on_change: impl Fn(usize) -> Message + 'a + Copy,
) -> Element<'a, Message> {
    let t = *theme;
    let mut r = row![].spacing(2).align_y(Vertical::Bottom);
    for (i, label) in labels.iter().enumerate() {
        let is_active = i == selected;
        let label = label.clone();
        r = r.push(
            button(text(label).size(13.0))
                .padding(Padding::from([8.0, 16.0]))
                .on_press(on_change(i))
                .style(move |_, status| {
                    use button::Status::*;
                    let (bg, fg, border_color, border_width) = if is_active {
                        (iced::Color::TRANSPARENT, t.foreground, t.primary, 2.0)
                    } else {
                        match status {
                            Hovered => (t.accent, t.foreground, iced::Color::TRANSPARENT, 0.0),
                            Pressed => (t.muted, t.foreground, iced::Color::TRANSPARENT, 0.0),
                            _ => (iced::Color::TRANSPARENT, t.muted_foreground, iced::Color::TRANSPARENT, 0.0),
                        }
                    };
                    button::Style {
                        background: Some(Background::Color(bg)),
                        text_color: fg,
                        border: Border {
                            color: border_color,
                            width: 0.0,
                            radius: 0.0.into(),
                        },
                        shadow: Shadow {
                            color: border_color,
                            offset: iced::Vector::new(0.0, border_width),
                            blur_radius: 0.0,
                        },
                        snap: true,
                    }
                }),
        );
    }
    r = r.push(Space::new().width(Length::Fill));

    container(r)
        .padding(Padding::from(0.0))
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

pub fn tabs<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    tabs: Vec<Tab<'a, Message>>,
    selected: usize,
    on_change: impl Fn(usize) -> Message + 'a + Copy,
) -> Element<'a, Message> {
    let t = *theme;
    let labels: Vec<String> = tabs.iter().map(|t| t.label.clone()).collect();
    let mut iter = tabs.into_iter();
    let mut content_el: Option<Element<'a, Message>> = None;
    let mut i = 0usize;
    while let Some(tab) = iter.next() {
        if i == selected {
            content_el = Some(tab.content);
        }
        i += 1;
    }
    let body: Element<'a, Message> = content_el.unwrap_or_else(|| {
        Space::new().width(Length::Fill).height(Length::Fixed(0.0)).into()
    });

    column![
        tab_bar(theme, &labels, selected, on_change),
        horizontal_separator(t),
        container(body)
            .padding(Padding::from([16.0, 0.0]))
            .width(Length::Fill),
    ]
    .spacing(0)
    .into()
}

fn horizontal_separator<'a, Message: 'a>(t: AppTheme) -> Element<'a, Message> {
    container(Space::new().width(Length::Fill).height(Length::Fixed(1.0)))
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .style(move |_| container::Style {
            background: Some(Background::Color(t.border)),
            text_color: None,
            border: Border::default(),
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}
