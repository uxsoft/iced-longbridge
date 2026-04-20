//! Resizable — themed wrapper around iced's `pane_grid`.
//!
//! The caller owns the `pane_grid::State` and decides what to render per pane.

use iced::{
    Element,
    widget::{pane_grid, PaneGrid},
};

use crate::theme::AppTheme;

pub use iced::widget::pane_grid::{Pane, ResizeEvent, State};

pub fn resizable<'a, T, Message: Clone + 'a>(
    theme: &AppTheme,
    state: &'a State<T>,
    on_resize: impl Fn(ResizeEvent) -> Message + 'a,
    render_pane: impl Fn(Pane, &'a T, bool) -> pane_grid::Content<'a, Message> + 'a,
) -> Element<'a, Message> {
    let t = *theme;
    let grid = PaneGrid::new(state, |pane, data, is_maximized| {
        render_pane(pane, data, is_maximized)
    })
    .on_resize(8, on_resize)
    .spacing(4)
    .style(move |_| pane_grid::Style {
        hovered_region: pane_grid::Highlight {
            background: iced::Background::Color(crate::theme::with_alpha(t.primary, 0.08)),
            border: iced::Border {
                color: t.primary,
                width: 1.0,
                radius: 6.0.into(),
            },
        },
        picked_split: pane_grid::Line {
            color: t.primary,
            width: 2.0,
        },
        hovered_split: pane_grid::Line {
            color: crate::theme::with_alpha(t.primary, 0.5),
            width: 2.0,
        },
    });
    Element::new(grid)
}

pub fn pane_content<'a, Message: 'a>(
    theme: &AppTheme,
    title: impl Into<String>,
    body: Element<'a, Message>,
) -> pane_grid::Content<'a, Message> {
    use iced::{
        Background, Border, Length, Padding, Shadow,
        widget::{column, container, text},
    };
    let t = *theme;
    let title = text(title.into()).size(13.0).color(t.muted_foreground);
    let content = column![title, body].spacing(8).padding(Padding::from(12.0));
    pane_grid::Content::new(
        container(content)
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
