//! Accordion — stack of collapsible sections.

use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    widget::{column, container},
};

use crate::{
    components::{collapsible::collapsible, divider},
    theme::AppTheme,
};

pub struct Section<'a, Message> {
    pub title: String,
    pub expanded: bool,
    pub on_toggle: Message,
    pub content: Element<'a, Message>,
}

pub fn accordion<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    sections: Vec<Section<'a, Message>>,
) -> Element<'a, Message> {
    let t = *theme;
    let mut col = column![].width(Length::Fill);
    let count = sections.len();
    for (i, section) in sections.into_iter().enumerate() {
        col = col.push(collapsible(
            theme,
            section.title,
            section.expanded,
            section.on_toggle,
            section.content,
        ));
        if i + 1 < count {
            col = col.push(divider::horizontal(theme));
        }
    }

    container(col)
        .padding(Padding::from(0.0))
        .width(Length::Fill)
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
        })
        .into()
}
