//! Date picker — button that anchors a calendar popover.

use iced::{
    Background, Border, Element, Padding, Shadow,
    alignment::Vertical,
    widget::{button, row, text},
};

use crate::{
    Message, State,
    components::{
        calendar::calendar,
        icon::{icon, IconName},
        popover::popover_dismissable,
    },
    demos::common::{section_caption, section_title},
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let t = *theme;
    let label = match state.calendar_selected {
        Some(d) => d.format("%-d %b %Y").to_string(),
        None => "Pick a date".into(),
    };
    let open = state.date_picker_open;

    let trigger = button(
        row![
            icon(theme, IconName::Calendar, 14.0),
            text(label).size(13.0),
            icon(theme, if open { IconName::ChevronUp } else { IconName::ChevronDown }, 13.0),
        ]
        .spacing(8)
        .align_y(Vertical::Center),
    )
    .padding(Padding::from([6.0, 12.0]))
    .on_press(Message::DatePickerToggle)
    .style(move |_, status| {
        use button::Status::*;
        let bg = match status {
            Hovered => t.accent,
            Pressed => t.muted,
            _ => t.background,
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: t.foreground,
            border: Border {
                color: t.input_border,
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        }
    })
    .into();

    let panel = open.then(|| {
        calendar(
            theme,
            state.calendar_view_year,
            state.calendar_view_month,
            state.calendar_selected,
            chrono::Local::now().date_naive(),
            Message::DatePickerSelect,
            Message::CalendarPrevMonth,
            Message::CalendarNextMonth,
        )
    });

    iced::widget::column![
        section_title(theme, "Date picker"),
        section_caption(theme, "Click the field to open the calendar. Pick a day to close it."),
        popover_dismissable(theme, trigger, panel, Message::DatePickerToggle),
    ]
    .spacing(10)
    .into()
}
