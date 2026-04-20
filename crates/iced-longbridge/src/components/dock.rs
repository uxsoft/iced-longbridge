//! Dock — tabbed panels arranged in a resizable pane grid.
//!
//! Each pane holds a stack of tabs with one active tab. Caller owns the
//! underlying `pane_grid::State` and a map of `pane → DockPanel`.

use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    alignment::Vertical,
    widget::{button, column, container, pane_grid, row, text, Space},
};

use crate::theme::AppTheme;

pub struct DockPanel<'a, Message> {
    pub tabs: Vec<DockTab<'a, Message>>,
    pub selected: usize,
}

pub struct DockTab<'a, Message> {
    pub label: String,
    pub on_select: Message,
    pub on_close: Option<Message>,
    pub content: Element<'a, Message>,
}

pub fn dock_pane<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    panel: DockPanel<'a, Message>,
) -> pane_grid::Content<'a, Message> {
    let t = *theme;
    let selected = panel.selected;

    let mut tab_row = row![].spacing(0).align_y(Vertical::Center);
    let mut selected_content: Option<Element<'a, Message>> = None;
    for (i, tab) in panel.tabs.into_iter().enumerate() {
        let is_active = i == selected;
        if is_active {
            selected_content = Some(tab.content);
        }
        tab_row = tab_row.push(dock_tab_button(
            t,
            &tab.label,
            is_active,
            tab.on_select,
            tab.on_close,
        ));
    }
    tab_row = tab_row.push(Space::new().width(Length::Fill));

    let header = container(tab_row)
        .padding(Padding::from([4.0, 6.0]))
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(t.muted)),
            text_color: Some(t.foreground),
            border: Border {
                color: t.border,
                width: 0.0,
                radius: 0.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        });

    let content = selected_content.unwrap_or_else(|| {
        Space::new().width(Length::Fill).height(Length::Fill).into()
    });
    let body = container(content)
        .padding(Padding::from(12.0))
        .width(Length::Fill)
        .height(Length::Fill);

    let stack = column![header, body].spacing(0);

    pane_grid::Content::new(
        container(stack)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(t.card)),
                text_color: Some(t.card_foreground),
                border: Border {
                    color: t.border,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            }),
    )
}

fn dock_tab_button<'a, Message: Clone + 'a>(
    t: AppTheme,
    label: &str,
    active: bool,
    on_select: Message,
    on_close: Option<Message>,
) -> Element<'a, Message> {
    let mut inner = row![text(label.to_string()).size(12.0).color(t.foreground)]
        .spacing(8)
        .align_y(Vertical::Center);
    if let Some(close_msg) = on_close {
        inner = inner.push(
            button(text("×").size(14.0))
                .padding(Padding::from([0.0, 4.0]))
                .on_press(close_msg)
                .style(move |_, status| {
                    use button::Status::*;
                    let bg = match status {
                        Hovered => crate::theme::with_alpha(t.danger, 0.15),
                        Pressed => crate::theme::with_alpha(t.danger, 0.25),
                        _ => iced::Color::TRANSPARENT,
                    };
                    button::Style {
                        background: Some(Background::Color(bg)),
                        text_color: t.muted_foreground,
                        border: Border {
                            color: iced::Color::TRANSPARENT,
                            width: 0.0,
                            radius: 3.0.into(),
                        },
                        shadow: Shadow::default(),
                        snap: true,
                    }
                }),
        );
    }
    button(container(inner).padding(Padding::from([0.0, 6.0])))
        .padding(Padding::from([4.0, 8.0]))
        .on_press(on_select)
        .style(move |_, status| {
            use button::Status::*;
            let (bg, fg, border) = if active {
                (t.background, t.foreground, t.border)
            } else {
                match status {
                    Hovered => (t.accent, t.foreground, iced::Color::TRANSPARENT),
                    Pressed => (t.muted, t.foreground, iced::Color::TRANSPARENT),
                    _ => (iced::Color::TRANSPARENT, t.muted_foreground, iced::Color::TRANSPARENT),
                }
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: fg,
                border: Border {
                    color: border,
                    width: if active { 1.0 } else { 0.0 },
                    radius: 4.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            }
        })
        .into()
}
