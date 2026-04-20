//! Calendar — month grid of day buttons.

use chrono::{Datelike, NaiveDate, Weekday};
use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, row, text},
};

use crate::{
    components::icon::{icon_colored, IconName},
    theme::AppTheme,
};

pub fn calendar<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    view_year: i32,
    view_month: u32,
    selected: Option<NaiveDate>,
    on_select: impl Fn(NaiveDate) -> Message + 'a + Copy,
    on_prev_month: Message,
    on_next_month: Message,
) -> Element<'a, Message> {
    let t = *theme;
    let header = row![
        nav_btn(t, IconName::ChevronLeft, on_prev_month),
        container(text(format!("{} {}", month_name(view_month), view_year)).size(14.0).color(t.foreground))
            .width(Length::Fill)
            .align_x(Horizontal::Center),
        nav_btn(t, IconName::ChevronRight, on_next_month),
    ]
    .spacing(8)
    .align_y(Vertical::Center);

    let weekdays = row![
        dow(t, "Mo"), dow(t, "Tu"), dow(t, "We"), dow(t, "Th"),
        dow(t, "Fr"), dow(t, "Sa"), dow(t, "Su"),
    ]
    .spacing(4);

    let first = NaiveDate::from_ymd_opt(view_year, view_month, 1).unwrap_or(NaiveDate::MIN);
    let leading = weekday_to_idx(first.weekday());
    let days_in_month = last_day_of_month(view_year, view_month);
    let total_cells = (leading + days_in_month as usize).div_ceil(7) * 7;

    let mut grid = column![].spacing(4);
    let mut cells_in_row: Vec<Element<Message>> = Vec::new();
    for cell in 0..total_cells {
        let day_n = cell as i32 - leading as i32 + 1;
        let el: Element<Message> = if day_n >= 1 && day_n <= days_in_month as i32 {
            let date = NaiveDate::from_ymd_opt(view_year, view_month, day_n as u32).unwrap();
            let is_selected = Some(date) == selected;
            let is_today = chrono::Local::now().date_naive() == date;
            day_cell(t, day_n as u32, is_selected, is_today, on_select(date))
        } else {
            container(text("").size(13.0))
                .width(Length::Fixed(36.0))
                .height(Length::Fixed(32.0))
                .into()
        };
        cells_in_row.push(el);
        if cells_in_row.len() == 7 {
            let mut r = row![].spacing(4);
            for el in cells_in_row.drain(..) {
                r = r.push(el);
            }
            grid = grid.push(r);
        }
    }

    column![header, weekdays, grid].spacing(10).into()
}

fn day_cell<'a, Message: Clone + 'a>(
    t: AppTheme,
    day: u32,
    selected: bool,
    today: bool,
    on_press: Message,
) -> Element<'a, Message> {
    let content = container(text(day.to_string()).size(13.0))
        .width(Length::Fixed(36.0))
        .height(Length::Fixed(32.0))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center);
    button(content)
        .padding(Padding::from(0.0))
        .on_press(on_press)
        .style(move |_, status| {
            use button::Status::*;
            let (bg, fg, border_color, border_width) = if selected {
                (t.primary, t.primary_foreground, t.primary, 0.0)
            } else if today {
                (t.background, t.foreground, t.primary, 1.0)
            } else {
                match status {
                    Hovered => (t.accent, t.foreground, iced::Color::TRANSPARENT, 0.0),
                    Pressed => (t.muted, t.foreground, iced::Color::TRANSPARENT, 0.0),
                    _ => (t.background, t.foreground, iced::Color::TRANSPARENT, 0.0),
                }
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: fg,
                border: Border {
                    color: border_color,
                    width: border_width,
                    radius: 6.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            }
        })
        .into()
}

fn nav_btn<'a, Message: Clone + 'a>(
    t: AppTheme,
    glyph: IconName,
    on_press: Message,
) -> Element<'a, Message> {
    button(icon_colored::<Message>(glyph, 14.0, t.foreground))
        .padding(Padding::from([4.0, 8.0]))
        .on_press(on_press)
        .style(move |_, status| {
            use button::Status::*;
            let bg = match status {
                Hovered => t.accent,
                Pressed => t.muted,
                _ => iced::Color::TRANSPARENT,
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
        })
        .into()
}

fn dow<'a, Message: 'a>(t: AppTheme, label: &str) -> Element<'a, Message> {
    container(text(label.to_string()).size(11.0).color(t.muted_foreground))
        .width(Length::Fixed(36.0))
        .align_x(Horizontal::Center)
        .into()
}

fn month_name(m: u32) -> &'static str {
    match m {
        1 => "January", 2 => "February", 3 => "March", 4 => "April",
        5 => "May", 6 => "June", 7 => "July", 8 => "August",
        9 => "September", 10 => "October", 11 => "November", 12 => "December",
        _ => "?",
    }
}

fn weekday_to_idx(w: Weekday) -> usize {
    match w {
        Weekday::Mon => 0,
        Weekday::Tue => 1,
        Weekday::Wed => 2,
        Weekday::Thu => 3,
        Weekday::Fri => 4,
        Weekday::Sat => 5,
        Weekday::Sun => 6,
    }
}

fn last_day_of_month(year: i32, month: u32) -> u32 {
    let (ny, nm) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    let first_next = NaiveDate::from_ymd_opt(ny, nm, 1).unwrap();
    first_next.pred_opt().unwrap().day()
}
