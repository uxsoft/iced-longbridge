//! Rendering pipeline for [`super::Table`]. Pure functions; no widget state.

use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    alignment::{Horizontal, Vertical},
    widget::{Space, column, container, mouse_area, row, scrollable, stack, text},
};

use crate::{
    components::{
        button::ghost_icon_button,
        icon::{icon_colored, lucide},
        keyboard_focus,
        popover::popover_aligned,
        tooltip::wrap as tooltip_wrap,
    },
    theme::AppTheme,
};

use super::{Column, ResizeHandlers, SortDir, Table};

/// Width of iced's default vertical scrollbar (matches `scrollable::Scrollbar`
/// `width` + `margin`). Kept in sync so header/body columns stay aligned.
const SCROLLBAR_WIDTH: f32 = 10.0;

/// Width of the column-boundary strip in the header and body. When resize is
/// enabled, the header strip is a mouse_area; the body strip is a plain gap
/// so header/body columns stay aligned.
const DIVIDER_WIDTH: f32 = 4.0;

/// Header row height. Matches the body row default for visual balance.
const HEADER_HEIGHT: f32 = 36.0;

pub(super) fn render<'r, 'a, T, Message>(
    table: Table<'r, 'a, T, Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
    T: 'r,
{
    let Table {
        theme: t,
        rows,
        mut columns,
        sort,
        on_sort,
        striped,
        row_height,
        resize,
        row_style,
        on_row_press,
        on_nav,
    } = table;
    let col_count = columns.len();

    // Header row.
    let mut header = row![].spacing(0);
    for (i, col) in columns.iter_mut().enumerate() {
        let is_sorted = sort.map(|(k, _)| Some(k) == col.sort_key).unwrap_or(false);
        let sort_dir = if is_sorted { sort.map(|(_, d)| d) } else { None };

        header = header.push(build_header_cell(&t, col, sort_dir, on_sort.as_deref()));

        if i + 1 < col_count {
            header = header.push(header_divider(&t, resize.as_ref(), i));
        }
    }
    // Reserve the vertical scrollbar's width on the right so header columns
    // line up with body columns once the scrollable steals 10px for its track.
    let header_bar = container(header)
        .width(Length::Fill)
        .padding(Padding::default().right(SCROLLBAR_WIDTH))
        .style(move |_| solid_bg(t.muted, t.muted_foreground, t.border));

    // Body rows.
    let mut body = column![].spacing(0);
    for (i, data) in rows.iter().enumerate() {
        let mut r = row![].spacing(0);
        for (ci, col) in columns.iter().enumerate() {
            let cell = container((col.render)(data))
                .padding(Padding::from([0.0, 12.0]))
                .width(col.width)
                .height(Length::Fixed(row_height))
                .align_x(col.align)
                .align_y(Vertical::Center);
            r = r.push(cell);
            if ci + 1 < col_count {
                r = r.push(body_divider(row_height));
            }
        }

        let zebra = striped && i % 2 == 1;
        let mut bg = if zebra { t.muted } else { t.background };
        let mut fg = t.foreground;
        if let Some(rs) = row_style.as_deref() {
            let s = rs(i, data);
            if let Some(c) = s.background {
                bg = c;
            }
            if let Some(c) = s.text_color {
                fg = c;
            }
        }

        let row_container = container(r)
            .width(Length::Fill)
            .style(move |_| solid_bg(bg, fg, t.border));

        let row_el: Element<Message> = match on_row_press.as_deref() {
            Some(cb) => mouse_area(row_container)
                .on_press(cb(i))
                .interaction(iced::mouse::Interaction::Pointer)
                .into(),
            None => row_container.into(),
        };
        body = body.push(row_el);
    }

    if rows.is_empty() {
        body = body.push(
            container(text("No rows").size(13.0).color(t.muted_foreground))
                .padding(Padding::from(24.0))
                .width(Length::Fill)
                .align_x(Horizontal::Center),
        );
    }

    let content = column![
        header_bar,
        container(Space::new().width(Length::Fill).height(Length::Fixed(1.0)))
            .style(move |_| container::Style {
                background: Some(Background::Color(t.border)),
                text_color: None,
                border: Border::default(),
                shadow: Shadow::default(),
                snap: true,
            }),
        scrollable(body).height(Length::Shrink),
    ]
    .spacing(0);

    let table_el: Element<Message> = container(content)
        .width(Length::Fill)
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
        .into();

    // When a drag is in progress, overlay a transparent mouse catcher so the
    // move/release events keep flowing even if the cursor leaves the narrow
    // handle.
    let table_el = match resize {
        Some(rh) if rh.dragging.is_some() => {
            let ResizeHandlers {
                on_drag,
                on_release,
                ..
            } = rh;
            let catcher_area: Element<Message> =
                Space::new().width(Length::Fill).height(Length::Fill).into();
            let catcher: Element<Message> = mouse_area(catcher_area)
                .on_move(on_drag)
                .on_release(on_release)
                .interaction(iced::mouse::Interaction::ResizingHorizontally)
                .into();
            stack![table_el, catcher].into()
        }
        _ => table_el,
    };

    // Wrap in a keyboard-capture widget if navigation is wired.
    match on_nav {
        Some(on_nav) => keyboard_focus::wrap(table_el, on_nav),
        None => table_el,
    }
}

fn solid_bg(bg: iced::Color, fg: iced::Color, border: iced::Color) -> container::Style {
    container::Style {
        background: Some(Background::Color(bg)),
        text_color: Some(fg),
        border: Border {
            color: border,
            width: 0.0,
            radius: 0.0.into(),
        },
        shadow: Shadow::default(),
        snap: true,
    }
}

/// Divider strip between two header cells. Clickable when resize is enabled.
fn header_divider<'a, Message: Clone + 'a>(
    t: &AppTheme,
    resize: Option<&ResizeHandlers<'a, Message>>,
    col_idx: usize,
) -> Element<'a, Message> {
    let t = *t;
    let resizable = resize.is_some();
    let base = container(Space::new().width(Length::Fixed(DIVIDER_WIDTH)).height(Length::Fill))
        .width(Length::Fixed(DIVIDER_WIDTH))
        .height(Length::Fixed(HEADER_HEIGHT))
        .style(move |_| {
            // Subtle vertical rule; only visible when resize is enabled so the
            // user has an affordance to grab. `t.border` collides with the
            // header's `t.muted` background in dark mode (both 0x26), so use
            // `muted_foreground` at low alpha for theme-agnostic contrast.
            let bg = if resizable {
                iced::Color { a: 0.35, ..t.muted_foreground }
            } else {
                iced::Color::TRANSPARENT
            };
            container::Style {
                background: Some(Background::Color(bg)),
                text_color: None,
                border: Border::default(),
                shadow: Shadow::default(),
                snap: true,
            }
        });

    match resize {
        Some(rh) => {
            let msg = (rh.on_grab)(col_idx);
            mouse_area(base)
                .on_press(msg)
                .interaction(iced::mouse::Interaction::ResizingHorizontally)
                .into()
        }
        None => base.into(),
    }
}

fn body_divider<'a, Message: 'a>(row_height: f32) -> Element<'a, Message> {
    container(Space::new().width(Length::Fixed(DIVIDER_WIDTH)).height(Length::Fill))
        .width(Length::Fixed(DIVIDER_WIDTH))
        .height(Length::Fixed(row_height))
        .into()
}

/// Title row (header text + optional sort arrow).
fn title_row<'a, Message: 'a>(
    t: AppTheme,
    header: &str,
    sort_dir: Option<SortDir>,
) -> iced::widget::Row<'a, Message> {
    let mut inner = row![text(header.to_string()).size(12.0).color(t.muted_foreground)]
        .spacing(6)
        .align_y(Vertical::Center);
    if let Some(dir) = sort_dir {
        inner = inner.push(icon_colored::<Message>(
            match dir {
                SortDir::Asc => lucide::arrow_up_narrow_wide(),
                SortDir::Desc => lucide::arrow_down_wide_narrow(),
            },
            11.0,
            t.muted_foreground,
        ));
    }
    inner
}

/// Wrap `content` in a sort-toggle button if the column is sortable and
/// `on_sort` is wired; otherwise return `content` (in a padded container if
/// `pad` is set).
fn maybe_sort_button<'a, Message: Clone + 'a>(
    t: AppTheme,
    content: Element<'a, Message>,
    sort_key: Option<&'static str>,
    on_sort: Option<&(dyn Fn(&'static str) -> Message + 'a)>,
    button_padding: Padding,
    fallback_padding: Option<Padding>,
) -> Element<'a, Message> {
    match (sort_key, on_sort) {
        (Some(key), Some(cb)) => {
            let msg = cb(key);
            iced::widget::button(content)
                .padding(button_padding)
                .on_press(msg)
                .style(move |_, status| {
                    use iced::widget::button::Status::*;
                    let bg = match status {
                        Hovered => t.accent,
                        Pressed => t.muted,
                        _ => iced::Color::TRANSPARENT,
                    };
                    iced::widget::button::Style {
                        background: Some(Background::Color(bg)),
                        text_color: t.foreground,
                        border: Border::default(),
                        shadow: Shadow::default(),
                        snap: true,
                    }
                })
                .into()
        }
        _ => match fallback_padding {
            Some(p) => container(content).padding(p).into(),
            None => content,
        },
    }
}

/// Build a single header cell. When the column has a `header_button`, the
/// title/sort area and the icon button are siblings so each is hit-tested
/// independently; otherwise the whole cell is the sort click target.
fn build_header_cell<'a, T, Message: Clone + 'a>(
    t: &AppTheme,
    col: &mut Column<'a, T, Message>,
    sort_dir: Option<SortDir>,
    on_sort: Option<&(dyn Fn(&'static str) -> Message + 'a)>,
) -> Element<'a, Message> {
    let t = *t;
    let title: Element<Message> = title_row(t, &col.header, sort_dir).into();

    if col.header_button_icon.is_some() {
        // Title-only sort button (small click target) + sibling icon button.
        let sort_area = maybe_sort_button(
            t,
            title,
            col.sort_key,
            on_sort,
            Padding::from([0.0, 12.0]),
            Some(Padding::from([0.0, 12.0])),
        );

        let icon = col.header_button_icon.clone().expect("checked above");
        let msg = col.header_button_msg.clone().expect("paired with icon");
        let mut trigger = ghost_icon_button(&t, icon, Some(msg.clone()));
        if let Some(label) = col.header_button_tooltip.clone() {
            trigger = tooltip_wrap(&t, trigger, label).into();
        }
        let header_btn: Element<'a, Message> = match col.header_panel.take() {
            Some(panel) => popover_aligned(&t, trigger, Some(panel), Horizontal::Right, Some(msg)),
            None => trigger,
        };

        let inner_row = match col.align {
            Horizontal::Left => row![sort_area, Space::new().width(Length::Fill), header_btn],
            Horizontal::Right => row![Space::new().width(Length::Fill), sort_area, header_btn],
            Horizontal::Center => row![
                Space::new().width(Length::Fill),
                sort_area,
                Space::new().width(Length::Fill),
                header_btn,
            ],
        }
        .spacing(0)
        .align_y(Vertical::Center);

        container(inner_row)
            .width(col.width)
            .height(Length::Fixed(HEADER_HEIGHT))
            .align_y(Vertical::Center)
            .into()
    } else {
        // Whole cell is the sort click target.
        let cell = container(title)
            .padding(Padding::from([0.0, 12.0]))
            .width(col.width)
            .height(Length::Fixed(HEADER_HEIGHT))
            .align_x(col.align)
            .align_y(Vertical::Center);
        maybe_sort_button(
            t,
            cell.into(),
            col.sort_key,
            on_sort,
            Padding::from(0),
            None,
        )
    }
}
