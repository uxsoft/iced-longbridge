use iced::{Element, widget::{column, text_editor}};

use crate::{
    Message, State,
    components::text_view::{is_editing, text_view},
    demos::common::{section_caption, section_title},
    theme::AppTheme,
};

pub const SAMPLE: &str = "Iced ✦ Longbridge\n\nThis text view is built on top of iced::widget::text_editor. It supports click-drag selection, keyboard navigation, and copy to clipboard (Ctrl+C), but edits are dropped by the host so the buffer never changes.\n\nDouble-click any word to select it. Triple-click for the line. Ctrl+A selects everything.";

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    column![
        section_title(theme, "Selectable text view"),
        section_caption(theme, "Read-only content with mouse and keyboard selection."),
        text_view(theme, &state.text_view_content, |action| {
            if is_editing(&action) {
                Message::NoOp
            } else {
                Message::TextViewAction(action)
            }
        }),
    ]
    .spacing(10)
    .into()
}

pub fn filter_action(action: text_editor::Action) -> Option<text_editor::Action> {
    if is_editing(&action) {
        None
    } else {
        Some(action)
    }
}
