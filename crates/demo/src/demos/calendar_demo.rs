use iced::{Element, widget::{column, text}};

use crate::{
    Message, State,
    components::calendar::calendar,
    demos::common::{section_caption, section_title, vspace},
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let cal = calendar(
        theme,
        state.calendar_view_year,
        state.calendar_view_month,
        state.calendar_selected,
        Message::CalendarSelected,
        Message::CalendarPrevMonth,
        Message::CalendarNextMonth,
    );

    let selected_label = match state.calendar_selected {
        Some(d) => format!("Selected: {}", d.format("%A, %-d %B %Y")),
        None => String::from("No date selected"),
    };

    column![
        section_title(theme, "Month grid"),
        section_caption(theme, "Navigate months with the chevrons, click a day to select."),
        cal,
        vspace(10.0),
        text(selected_label).size(13.0).color(theme.muted_foreground),
    ]
    .spacing(10)
    .into()
}
