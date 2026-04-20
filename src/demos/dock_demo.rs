use iced::{
    Element, Length,
    widget::{column, pane_grid, text, PaneGrid},
};

use crate::{
    Message, State,
    components::dock::{dock_pane, DockPanel, DockTab},
    demos::common::section_title,
    theme::AppTheme,
};

#[derive(Debug, Clone)]
pub struct DockPaneData {
    pub tabs: Vec<String>,
    pub selected: usize,
    pub id: usize,
}

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let t = *theme;

    let grid = PaneGrid::new(&state.dock_state, move |_pane, data: &'a DockPaneData, _is_max| {
        let id = data.id;
        let selected = data.selected;
        let tabs: Vec<DockTab<Message>> = data
            .tabs
            .iter()
            .enumerate()
            .map(|(i, label)| DockTab {
                label: label.clone(),
                on_select: Message::DockTabSelected(id, i),
                on_close: None,
                content: tab_body(theme, label, selected == i),
            })
            .collect();
        dock_pane(theme, DockPanel { tabs, selected })
    })
    .on_resize(8, Message::DockResized)
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
        picked_split: pane_grid::Line { color: t.primary, width: 2.0 },
        hovered_split: pane_grid::Line {
            color: crate::theme::with_alpha(t.primary, 0.5),
            width: 2.0,
        },
    });

    column![
        section_title(theme, "Dockable panels"),
        iced::widget::container(grid)
            .width(Length::Fill)
            .height(Length::Fixed(360.0)),
    ]
    .spacing(10)
    .into()
}

fn tab_body<'a>(theme: &AppTheme, label: &str, _is_selected: bool) -> Element<'a, Message> {
    text(format!("Contents of '{}' tab.", label))
        .size(13.0)
        .color(theme.muted_foreground)
        .into()
}
