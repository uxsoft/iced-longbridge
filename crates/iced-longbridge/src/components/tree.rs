//! Tree — recursive nested list with expand / collapse.
//!
//! Callers own the expansion state. The tree renders chevrons that emit
//! `on_toggle(id)` messages and rows that emit `on_select(id)`.

use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    alignment::Vertical,
    widget::{button, column, container, row, text, Space},
};

use crate::{
    components::icon::{icon_colored, Icon, IconName},
    theme::AppTheme,
};

pub struct TreeNode {
    pub id: String,
    pub label: String,
    pub icon: Option<Icon>,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            children: Vec::new(),
        }
    }

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn child(mut self, child: TreeNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }
}

pub fn tree<'a, Message, OnToggle, OnSelect>(
    theme: &AppTheme,
    nodes: &'a [TreeNode],
    expanded: impl Fn(&str) -> bool + 'a + Copy,
    selected: Option<&'a str>,
    on_toggle: OnToggle,
    on_select: OnSelect,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
    OnToggle: Fn(String) -> Message + 'a + Copy,
    OnSelect: Fn(String) -> Message + 'a + Copy,
{
    let mut rows: Vec<Element<Message>> = Vec::new();
    for n in nodes {
        flatten(theme, n, 0, expanded, selected, on_toggle, on_select, &mut rows);
    }
    let mut c = column![].spacing(2);
    for r in rows {
        c = c.push(r);
    }
    c.into()
}

#[allow(clippy::too_many_arguments)]
fn flatten<'a, Message, OnToggle, OnSelect>(
    theme: &AppTheme,
    node: &'a TreeNode,
    depth: usize,
    expanded: impl Fn(&str) -> bool + 'a + Copy,
    selected: Option<&'a str>,
    on_toggle: OnToggle,
    on_select: OnSelect,
    sink: &mut Vec<Element<'a, Message>>,
) where
    Message: Clone + 'a,
    OnToggle: Fn(String) -> Message + 'a + Copy,
    OnSelect: Fn(String) -> Message + 'a + Copy,
{
    let t = *theme;
    let id = node.id.clone();
    let has_children = !node.children.is_empty();
    let is_expanded = expanded(&id);
    let is_selected = selected == Some(id.as_str());

    let indent = 16.0 * depth as f32;
    let mut row_content = row![Space::new().width(Length::Fixed(indent))]
        .spacing(0)
        .align_y(Vertical::Center);

    if has_children {
        let glyph = if is_expanded {
            IconName::ChevronDown
        } else {
            IconName::ChevronRight
        };
        let chevron = button(icon_colored::<Message>(glyph, 12.0, t.muted_foreground))
            .padding(Padding::from(2.0))
            .on_press(on_toggle(id.clone()))
            .style(move |_, status| {
                use button::Status::*;
                let bg = match status {
                    Hovered => t.accent,
                    _ => iced::Color::TRANSPARENT,
                };
                button::Style {
                    background: Some(Background::Color(bg)),
                    text_color: t.foreground,
                    border: Border::default(),
                    shadow: Shadow::default(),
                    snap: true,
                }
            });
        row_content = row_content.push(chevron);
    } else {
        row_content = row_content.push(Space::new().width(Length::Fixed(18.0)));
    }

    if let Some(icon_src) = node.icon.clone() {
        row_content = row_content.push(Space::new().width(Length::Fixed(4.0)));
        row_content = row_content.push(icon_colored::<Message>(icon_src, 14.0, t.muted_foreground));
    }

    row_content = row_content.push(Space::new().width(Length::Fixed(6.0)));
    row_content = row_content.push(text(node.label.clone()).size(13.0).color(t.foreground));
    row_content = row_content.push(Space::new().width(Length::Fill));

    let row_btn = button(
        container(row_content)
            .padding(Padding::from([4.0, 6.0]))
            .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding(0)
    .on_press(on_select(id.clone()))
    .style(move |_, status| {
        use button::Status::*;
        let bg = if is_selected {
            t.accent
        } else {
            match status {
                Hovered => t.muted,
                Pressed => t.accent,
                _ => iced::Color::TRANSPARENT,
            }
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: t.foreground,
            border: Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: 4.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        }
    });

    sink.push(row_btn.into());

    if is_expanded {
        for child in &node.children {
            flatten(theme, child, depth + 1, expanded, selected, on_toggle, on_select, sink);
        }
    }
}
