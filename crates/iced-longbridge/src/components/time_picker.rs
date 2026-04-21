//! Time picker — hour / minute / optional-second selector and a popover trigger.

use chrono::{NaiveTime, Timelike};
use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, row, text},
};

use crate::{
    components::{
        icon::{icon, IconName},
        popover::popover_dismissable,
    },
    theme::AppTheme,
};

/// Inline three-column picker.
///
/// Each column shows the current value with a `+` button above and a `−`
/// button below. Hours wrap 0–23, minutes and seconds wrap 0–59.
pub fn time_picker<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    time: NaiveTime,
    show_seconds: bool,
    on_hour: impl Fn(u32) -> Message + 'a + Copy,
    on_minute: impl Fn(u32) -> Message + 'a + Copy,
    on_second: impl Fn(u32) -> Message + 'a + Copy,
) -> Element<'a, Message> {
    let t = *theme;
    container(time_picker_inner(theme, time, show_seconds, on_hour, on_minute, on_second))
        .padding(Padding::from([12.0, 16.0]))
        .style(move |_| container::Style {
            background: Some(Background::Color(t.background)),
            text_color: Some(t.foreground),
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

/// The picker row without the outer bordered container — used when the caller
/// (e.g. [`time_picker_popover`]) already wraps the content in a bordered
/// popover chrome.
fn time_picker_inner<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    time: NaiveTime,
    show_seconds: bool,
    on_hour: impl Fn(u32) -> Message + 'a + Copy,
    on_minute: impl Fn(u32) -> Message + 'a + Copy,
    on_second: impl Fn(u32) -> Message + 'a + Copy,
) -> Element<'a, Message> {
    let t = *theme;

    let hour_col = spinner(t, time.hour(), 24, on_hour);
    let minute_col = spinner(t, time.minute(), 60, on_minute);

    let sep = |label: &str| -> Element<'a, Message> {
        container(text(label.to_string()).size(22.0).color(t.foreground))
            .height(Length::Fixed(120.0))
            .align_y(Vertical::Center)
            .into()
    };

    let mut r = row![hour_col, sep(":"), minute_col]
        .spacing(8)
        .align_y(Vertical::Center);
    if show_seconds {
        r = r.push(sep(":"));
        r = r.push(spinner(t, time.second(), 60, on_second));
    }
    r.into()
}

/// Button trigger + anchored picker, mirroring [`date_picker`](super::calendar).
pub fn time_picker_popover<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    time: NaiveTime,
    show_seconds: bool,
    open: bool,
    on_toggle: Message,
    on_hour: impl Fn(u32) -> Message + 'a + Copy,
    on_minute: impl Fn(u32) -> Message + 'a + Copy,
    on_second: impl Fn(u32) -> Message + 'a + Copy,
) -> Element<'a, Message> {
    let t = *theme;
    let label = if show_seconds {
        format!("{:02}:{:02}:{:02}", time.hour(), time.minute(), time.second())
    } else {
        format!("{:02}:{:02}", time.hour(), time.minute())
    };

    let trigger = button(
        row![
            icon(theme, IconName::Clock, 14.0),
            text(label).size(13.0),
            icon(
                theme,
                if open { IconName::ChevronUp } else { IconName::ChevronDown },
                13.0,
            ),
        ]
        .spacing(8)
        .align_y(Vertical::Center),
    )
    .padding(Padding::from([6.0, 12.0]))
    .on_press(on_toggle.clone())
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
        container(time_picker_inner(theme, time, show_seconds, on_hour, on_minute, on_second))
            .padding(Padding::from([6.0, 8.0]))
            .into()
    });
    popover_dismissable(theme, trigger, panel, on_toggle)
}

fn spinner<'a, Message: Clone + 'a>(
    t: AppTheme,
    value: u32,
    modulo: u32,
    on_change: impl Fn(u32) -> Message + 'a + Copy,
) -> Element<'a, Message> {
    let next = (value + 1) % modulo;
    let prev = (value + modulo - 1) % modulo;

    column![
        step_btn(t, "▴", on_change(next)),
        container(text(format!("{value:02}")).size(22.0).color(t.foreground))
            .width(Length::Fixed(56.0))
            .height(Length::Fixed(40.0))
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center),
        step_btn(t, "▾", on_change(prev)),
    ]
    .spacing(4)
    .align_x(Horizontal::Center)
    .into()
}

fn step_btn<'a, Message: Clone + 'a>(
    t: AppTheme,
    glyph: &str,
    on_press: Message,
) -> Element<'a, Message> {
    button(
        container(text(glyph.to_string()).size(14.0).color(t.foreground))
            .width(Length::Fixed(56.0))
            .height(Length::Fixed(28.0))
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center),
    )
    .padding(Padding::from(0.0))
    .on_press(on_press)
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
                color: t.border,
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        }
    })
    .into()
}
