//! Description list — two-column term / definition pairs.

use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{column, row, text, Space},
};

use crate::theme::AppTheme;

pub struct DescItem {
    pub term: String,
    pub definition: String,
}

impl DescItem {
    pub fn new(term: impl Into<String>, definition: impl Into<String>) -> Self {
        Self {
            term: term.into(),
            definition: definition.into(),
        }
    }
}

pub fn description_list<'a, Message: 'a>(
    theme: &AppTheme,
    items: Vec<DescItem>,
) -> Element<'a, Message> {
    description_list_sized(theme, 180.0, items)
}

pub fn description_list_sized<'a, Message: 'a>(
    theme: &AppTheme,
    term_width: f32,
    items: Vec<DescItem>,
) -> Element<'a, Message> {
    let mut c = column![].spacing(10).width(Length::Fill);
    for it in items {
        let row = row![
            text(it.term).size(13.0).color(theme.muted_foreground).width(Length::Fixed(term_width)),
            Space::new().width(Length::Fixed(12.0)),
            text(it.definition).size(13.0).color(theme.foreground).width(Length::Fill),
        ]
        .align_y(Vertical::Top);
        c = c.push(row);
    }
    c.into()
}
