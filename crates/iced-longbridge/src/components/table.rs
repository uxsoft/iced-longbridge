//! Table — column-and-row layout with typed cells.
//!
//! Build columns with [`Column::new`] and render rows from any `Vec<T>` via
//! [`table`]. Header is a sticky row; body is scrollable.
//!
//! ## Resizable columns
//!
//! Pass a [`ResizeHandlers`] in [`TableOptions::resize`] to enable drag-to-
//! resize on the boundary between adjacent columns. The caller owns the
//! per-column width state (as `Length::Fixed` on each [`Column`]) and updates
//! it from the emitted messages:
//!
//! - `on_grab(i)` fires when the user presses on the divider *after* column
//!   `i` — the caller should record `dragging = Some(i)`.
//! - `on_drag(Point)` fires continuously while the user moves the mouse; the
//!   caller converts the cursor-x delta to a width change.
//! - `on_release` fires on mouse-up; the caller clears `dragging`.
//!
//! While `dragging.is_some()` the table wraps itself in a full-area mouse
//! overlay so the drag keeps working even when the cursor wanders off the
//! handle.

use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    alignment::{Horizontal, Vertical},
    widget::{column, container, mouse_area, row, scrollable, stack, text, Space},
};

use crate::{
    components::{
        button::ghost_icon_button,
        icon::{icon_colored, lucide, Icon},
        popover::popover_aligned,
    },
    theme::AppTheme,
};

/// Width of iced's default vertical scrollbar (matches `scrollable::Scrollbar`
/// `width` + `margin`). Kept in sync so header/body columns stay aligned.
const SCROLLBAR_WIDTH: f32 = 10.0;

/// Width of the column-boundary strip in the header and body. When resize is
/// enabled, the header strip is a mouse_area; the body strip is a plain gap
/// so header/body columns stay aligned.
const DIVIDER_WIDTH: f32 = 4.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDir {
    Asc,
    Desc,
}

pub struct Column<'a, T, Message> {
    pub header: String,
    pub width: Length,
    pub align: Horizontal,
    #[allow(clippy::type_complexity)]
    pub render: Box<dyn Fn(&T) -> Element<'a, Message> + 'a>,
    pub sort_key: Option<&'static str>,
    pub header_button_icon: Option<Icon>,
    pub header_button_msg: Option<Message>,
    pub header_panel: Option<Element<'a, Message>>,
}

impl<'a, T, Message> Column<'a, T, Message> {
    pub fn new(
        header: impl Into<String>,
        render: impl Fn(&T) -> Element<'a, Message> + 'a,
    ) -> Self {
        Self {
            header: header.into(),
            width: Length::Fill,
            align: Horizontal::Left,
            render: Box::new(render),
            sort_key: None,
            header_button_icon: None,
            header_button_msg: None,
            header_panel: None,
        }
    }

    pub fn width(mut self, w: Length) -> Self {
        self.width = w;
        self
    }

    pub fn align(mut self, a: Horizontal) -> Self {
        self.align = a;
        self
    }

    pub fn sortable(mut self, key: &'static str) -> Self {
        self.sort_key = Some(key);
        self
    }

    /// Add an icon button to this column's header. Composed next to the
    /// title/sort area so it stays independently clickable when the column
    /// is also `sortable`.
    pub fn header_button(mut self, icon: impl Into<Icon>, on_press: Message) -> Self {
        self.header_button_icon = Some(icon.into());
        self.header_button_msg = Some(on_press);
        self
    }

    /// Attach a floating panel (e.g. a `menu()`) anchored beneath the header
    /// button. The same `on_press` message also fires on outside-click to
    /// dismiss, so a single toggle message can drive open/close.
    pub fn header_panel(mut self, panel: impl Into<Element<'a, Message>>) -> Self {
        self.header_panel = Some(panel.into());
        self
    }
}

/// Drag-to-resize wiring for column boundaries. See module docs for the
/// lifecycle.
pub struct ResizeHandlers<'a, Message> {
    /// Mouse-down on the divider between column `i` and `i + 1`.
    pub on_grab: Box<dyn Fn(usize) -> Message + 'a>,
    /// Mouse moved while the caller is in a dragging state.
    pub on_drag: Box<dyn Fn(iced::Point) -> Message + 'a>,
    /// Mouse released (while dragging).
    pub on_release: Message,
    /// Index of the column currently being dragged, or `None` when idle.
    pub dragging: Option<usize>,
}

pub struct TableOptions<'a, Message> {
    pub sort: Option<(&'static str, SortDir)>,
    pub on_sort: Option<Box<dyn Fn(&'static str) -> Message + 'a>>,
    pub striped: bool,
    pub row_height: f32,
    pub resize: Option<ResizeHandlers<'a, Message>>,
}

impl<'a, Message> Default for TableOptions<'a, Message> {
    fn default() -> Self {
        Self {
            sort: None,
            on_sort: None,
            striped: true,
            row_height: 40.0,
            resize: None,
        }
    }
}

#[allow(dead_code)]
pub fn table<'a, T, Message: Clone + 'a>(
    theme: &AppTheme,
    rows: &[T],
    columns: Vec<Column<'a, T, Message>>,
) -> Element<'a, Message> {
    table_with(theme, rows, columns, TableOptions::default())
}

pub fn table_with<'a, T, Message: Clone + 'a>(
    theme: &AppTheme,
    rows: &[T],
    columns: Vec<Column<'a, T, Message>>,
    options: TableOptions<'a, Message>,
) -> Element<'a, Message> {
    let t = *theme;
    let TableOptions {
        sort,
        on_sort,
        striped,
        row_height,
        resize,
    } = options;
    let mut columns = columns;
    let col_count = columns.len();

    // Header row.
    let mut header = row![].spacing(0);
    for (i, col) in columns.iter_mut().enumerate() {
        let is_sorted = sort.map(|(k, _)| Some(k) == col.sort_key).unwrap_or(false);
        let sort_dir = if is_sorted { sort.map(|(_, d)| d) } else { None };

        let cell_el: Element<Message> = if col.header_button_icon.is_some() {
            build_header_cell_with_button(&t, col, sort_dir, on_sort.as_deref())
        } else {
            build_header_cell(&t, col, sort_dir, on_sort.as_deref())
        };

        header = header.push(cell_el);

        if i + 1 < col_count {
            header = header.push(header_divider(&t, resize.as_ref(), i));
        }
    }
    // Reserve the vertical scrollbar's width on the right so header columns
    // line up with body columns once the scrollable steals 10px for its track.
    let header_bar = container(header)
        .width(Length::Fill)
        .padding(Padding::default().right(SCROLLBAR_WIDTH))
        .style(move |_| container::Style {
            background: Some(Background::Color(t.muted)),
            text_color: Some(t.muted_foreground),
            border: Border {
                color: t.border,
                width: 0.0,
                radius: 0.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        });

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
        body = body.push(
            container(r)
                .width(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(if zebra {
                        t.muted
                    } else {
                        t.background
                    })),
                    text_color: Some(t.foreground),
                    border: Border {
                        color: t.border,
                        width: 0.0,
                        radius: 0.0.into(),
                    },
                    shadow: Shadow::default(),
                    snap: true,
                }),
        );
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
    match resize {
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
        .height(Length::Fixed(36.0))
        .style(move |_| {
            // Subtle vertical rule; only visible when resize is enabled so the
            // user has an affordance to grab.
            let bg = if resizable { t.border } else { iced::Color::TRANSPARENT };
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

/// Build a sort-toggle button (or plain title row) for the column header.
/// Used by `build_header_cell_with_button` so the title click target stays
/// distinct from the icon button.
fn build_sort_area<'a, T, Message: Clone + 'a>(
    t: &AppTheme,
    col: &Column<'a, T, Message>,
    sort_dir: Option<SortDir>,
    on_sort: Option<&(dyn Fn(&'static str) -> Message + 'a)>,
) -> Element<'a, Message> {
    let t = *t;
    let mut inner = row![text(col.header.clone()).size(12.0).color(t.muted_foreground)]
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

    match (col.sort_key, on_sort) {
        (Some(key), Some(cb)) => {
            let msg = cb(key);
            iced::widget::button(inner)
                .padding(Padding::from([0.0, 12.0]))
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
        _ => container(inner)
            .padding(Padding::from([0.0, 12.0]))
            .into(),
    }
}

/// Header cell layout used when a column has no `header_button`. Kept
/// behaviour-equivalent to the pre-extension code so existing call sites
/// render identically.
fn build_header_cell<'a, T, Message: Clone + 'a>(
    t: &AppTheme,
    col: &Column<'a, T, Message>,
    sort_dir: Option<SortDir>,
    on_sort: Option<&(dyn Fn(&'static str) -> Message + 'a)>,
) -> Element<'a, Message> {
    let t = *t;
    let mut inner = row![text(col.header.clone()).size(12.0).color(t.muted_foreground)]
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

    let cell = container(inner)
        .padding(Padding::from([0.0, 12.0]))
        .width(col.width)
        .height(Length::Fixed(36.0))
        .align_x(col.align)
        .align_y(Vertical::Center);

    match (col.sort_key, on_sort) {
        (Some(key), Some(cb)) => {
            let msg = cb(key);
            iced::widget::button(cell)
                .padding(0)
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
        _ => cell.into(),
    }
}

/// Header cell with an icon button (and optional floating panel) composed
/// next to the title/sort area. The icon button is a sibling of the sort
/// wrapper — not a child — so each `iced::widget::button` is hit-tested
/// independently.
fn build_header_cell_with_button<'a, T, Message: Clone + 'a>(
    t: &AppTheme,
    col: &mut Column<'a, T, Message>,
    sort_dir: Option<SortDir>,
    on_sort: Option<&(dyn Fn(&'static str) -> Message + 'a)>,
) -> Element<'a, Message> {
    let t = *t;
    let sort_area = build_sort_area(&t, col, sort_dir, on_sort);

    let icon = col.header_button_icon.clone().expect("checked by caller");
    let msg = col.header_button_msg.clone().expect("checked by caller");
    let trigger = ghost_icon_button(&t, icon, Some(msg.clone()));
    let header_btn: Element<'a, Message> = match col.header_panel.take() {
        Some(panel) => popover_aligned(
            &t,
            trigger,
            Some(panel),
            Horizontal::Right,
            Some(msg),
        ),
        None => trigger,
    };

    let inner_row = match col.align {
        Horizontal::Left => row![
            sort_area,
            Space::new().width(Length::Fill),
            header_btn,
        ],
        Horizontal::Right => row![
            Space::new().width(Length::Fill),
            sort_area,
            header_btn,
        ],
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
        .height(Length::Fixed(36.0))
        .align_y(Vertical::Center)
        .into()
}
