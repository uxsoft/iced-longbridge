use iced::{Element, widget::column};

use crate::{
    Message, State,
    components::data_table::data_table,
    demos::common::{section_caption, section_title},
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    column![
        section_title(theme, "Data table"),
        section_caption(theme, "Rows built from Vec<Vec<String>>, columns from headers + widths."),
        data_table::<Message>(theme, &state.data_table),
    ]
    .spacing(10)
    .into()
}
