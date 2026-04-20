use iced::{Element, widget::column};

use crate::{
    Message, State,
    components::pagination::pagination,
    demos::common::{section_caption, section_title},
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    column![
        section_title(theme, "Pagination"),
        section_caption(theme, format!("Page {} of {}", state.pagination_page, 20)),
        pagination(theme, state.pagination_page, 20, Message::PaginationSelected),
        section_title(theme, "Small set"),
        pagination(theme, state.pagination_small, 5, Message::PaginationSmallSelected),
    ]
    .spacing(12)
    .into()
}
