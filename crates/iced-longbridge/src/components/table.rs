//! Table — column-and-row layout with typed cells.
//!
//! Build columns with [`Column::new`] (or [`Column::text`] for plain-text
//! cells) and render rows with the [`table`] builder:
//!
//! ```ignore
//! table(theme, &rows, columns)
//!     .row_height(44.0)
//!     .sort(state.sort, Message::Sort)
//!     .resize(&state.resize, Message::Resize)
//!     .into()
//! ```
//!
//! Header is a sticky row; body is scrollable.
//!
//! ## Resizable columns
//!
//! [`Table::resize`] enables drag-to-resize on column boundaries. The caller
//! owns a single [`ResizeState`] value (which carries widths + drag bookkeeping)
//! and forwards a single [`ResizeEvent`] message back into it:
//!
//! ```ignore
//! // State
//! pub resize: ResizeState,
//!
//! // Update
//! Message::Resize(event) => state.resize.apply(event),
//!
//! // View — read column widths from the state
//! Column::text("Name", ...).width(state.resize.width(0))
//! ```
//!
//! While a drag is in progress the table wraps itself in a full-area mouse
//! overlay so the drag keeps working even when the cursor wanders off the
//! handle.

use iced::{
    Background, Border, Element, Length, Padding, Point, Shadow,
    alignment::{Horizontal, Vertical},
    widget::{column, container, mouse_area, row, scrollable, stack, text, Space},
};

use crate::{
    components::{
        button::ghost_icon_button,
        icon::{icon_colored, lucide, Icon},
        popover::popover_aligned,
        tooltip::wrap as tooltip_wrap,
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

/// Header row height. Matches the body row default for visual balance.
const HEADER_HEIGHT: f32 = 36.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDir {
    Asc,
    Desc,
}

pub struct Column<'a, T, Message> {
    header: String,
    width: Length,
    align: Horizontal,
    #[allow(clippy::type_complexity)]
    render: Box<dyn Fn(&T) -> Element<'a, Message> + 'a>,
    sort_key: Option<&'static str>,
    header_button_icon: Option<Icon>,
    header_button_msg: Option<Message>,
    header_button_tooltip: Option<String>,
    header_panel: Option<Element<'a, Message>>,
}

impl<'a, T, Message> Column<'a, T, Message> {
    pub fn new<F>(header: impl Into<String>, render: F) -> Self
    where
        F: Fn(&T) -> Element<'a, Message> + 'a,
    {
        Self {
            header: header.into(),
            width: Length::Fill,
            align: Horizontal::Left,
            render: Box::new(render),
            sort_key: None,
            header_button_icon: None,
            header_button_msg: None,
            header_button_tooltip: None,
            header_panel: None,
        }
    }

    /// Shortcut for the common case: a plain-text cell. The accessor returns
    /// a `String` rendered at the standard cell size; the row container's
    /// `text_color` styles the foreground.
    pub fn text<F>(header: impl Into<String>, accessor: F) -> Self
    where
        F: Fn(&T) -> String + 'a,
        Message: 'a,
    {
        Self::new(header, move |row| {
            iced::widget::text(accessor(row)).size(13.0).into()
        })
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

    /// Optional tooltip shown when hovering over the header button.
    pub fn header_button_tooltip(mut self, label: impl Into<String>) -> Self {
        self.header_button_tooltip = Some(label.into());
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

struct ResizeHandlers<'a, Message> {
    on_grab: Box<dyn Fn(usize) -> Message + 'a>,
    on_drag: Box<dyn Fn(Point) -> Message + 'a>,
    on_release: Message,
    dragging: Option<usize>,
}

/// Per-column-resize state. Owned by the caller, updated via
/// [`ResizeState::apply`] from a single message handler.
#[derive(Debug, Clone)]
pub struct ResizeState {
    widths: Vec<f32>,
    dragging: Option<DragInfo>,
    min_width: f32,
    max_width: f32,
}

#[derive(Debug, Clone, Copy)]
struct DragInfo {
    col: usize,
    last_x: Option<f32>,
}

/// Opaque event emitted by the table during a resize interaction. Forward it
/// back into [`ResizeState::apply`] from the message handler.
#[derive(Debug, Clone)]
pub enum ResizeEvent {
    Grab(usize),
    Move(Point),
    Release,
}

impl ResizeState {
    /// Initialize with starting per-column widths (one entry per column).
    pub fn new(widths: Vec<f32>) -> Self {
        Self {
            widths,
            dragging: None,
            min_width: 40.0,
            max_width: f32::INFINITY,
        }
    }

    /// Constrain the per-column width during drags. Defaults to `40.0`.
    pub fn min_width(mut self, w: f32) -> Self {
        self.min_width = w;
        self
    }

    /// Constrain the per-column width during drags. Defaults to unbounded.
    pub fn max_width(mut self, w: f32) -> Self {
        self.max_width = w;
        self
    }

    /// `Length::Fixed` for the configured column, or `Length::Fill` if the
    /// index is past the end.
    pub fn width(&self, col: usize) -> Length {
        self.widths
            .get(col)
            .copied()
            .map(Length::Fixed)
            .unwrap_or(Length::Fill)
    }

    pub fn widths(&self) -> &[f32] {
        &self.widths
    }

    /// Apply an event from the table. Call from the message handler.
    pub fn apply(&mut self, event: ResizeEvent) {
        match event {
            ResizeEvent::Grab(col) => {
                self.dragging = Some(DragInfo { col, last_x: None });
            }
            ResizeEvent::Move(p) => {
                if let Some(drag) = self.dragging.as_mut() {
                    if let Some(prev) = drag.last_x {
                        let delta = p.x - prev;
                        if let Some(w) = self.widths.get_mut(drag.col) {
                            *w = (*w + delta).clamp(self.min_width, self.max_width);
                        }
                    }
                    drag.last_x = Some(p.x);
                }
            }
            ResizeEvent::Release => {
                self.dragging = None;
            }
        }
    }
}

/// Builder produced by [`table`]. Configure with the chained methods, then
/// convert to an `Element` via `.into()`.
///
/// `'r` is the rows borrow; `'a` is the lifetime of column closures and other
/// captured callbacks. The two are kept separate so the produced `Element`
/// (lifetime `'a`) doesn't appear to borrow `rows`.
pub struct Table<'r, 'a, T, Message> {
    theme: AppTheme,
    rows: &'r [T],
    columns: Vec<Column<'a, T, Message>>,
    sort: Option<(&'static str, SortDir)>,
    on_sort: Option<Box<dyn Fn(&'static str) -> Message + 'a>>,
    striped: bool,
    row_height: f32,
    resize: Option<ResizeHandlers<'a, Message>>,
}

pub fn table<'r, 'a, T, Message>(
    theme: &AppTheme,
    rows: &'r [T],
    columns: Vec<Column<'a, T, Message>>,
) -> Table<'r, 'a, T, Message> {
    Table {
        theme: *theme,
        rows,
        columns,
        sort: None,
        on_sort: None,
        striped: true,
        row_height: 40.0,
        resize: None,
    }
}

impl<'r, 'a, T, Message> Table<'r, 'a, T, Message> {
    pub fn striped(mut self, on: bool) -> Self {
        self.striped = on;
        self
    }

    pub fn row_height(mut self, h: f32) -> Self {
        self.row_height = h;
        self
    }

    /// Wire up sortable columns. `current` is the active sort, or `None` if
    /// the table is unsorted; `on_sort` fires when the user clicks a sortable
    /// header.
    pub fn sort<F>(mut self, current: Option<(&'static str, SortDir)>, on_sort: F) -> Self
    where
        F: Fn(&'static str) -> Message + 'a,
    {
        self.sort = current;
        self.on_sort = Some(Box::new(on_sort));
        self
    }

    /// Enable drag-to-resize on column boundaries. The caller owns a
    /// [`ResizeState`] (typically stored alongside other view state) and
    /// forwards [`ResizeEvent`]s back into it via `on_event`. See the module
    /// docs for the full pattern.
    pub fn resize<F>(mut self, state: &ResizeState, on_event: F) -> Self
    where
        F: Fn(ResizeEvent) -> Message + 'a,
        Message: 'a,
    {
        let on_event = std::rc::Rc::new(on_event);
        let grab_cb = on_event.clone();
        let drag_cb = on_event.clone();
        self.resize = Some(ResizeHandlers {
            on_grab: Box::new(move |i| grab_cb(ResizeEvent::Grab(i))),
            on_drag: Box::new(move |p| drag_cb(ResizeEvent::Move(p))),
            on_release: on_event(ResizeEvent::Release),
            dragging: state.dragging.as_ref().map(|d| d.col),
        });
        self
    }
}

impl<'r, 'a, T, Message> From<Table<'r, 'a, T, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
    T: 'r,
{
    fn from(table: Table<'r, 'a, T, Message>) -> Self {
        render(table)
    }
}

fn render<'r, 'a, T, Message>(table: Table<'r, 'a, T, Message>) -> Element<'a, Message>
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
        let bg = if zebra { t.muted } else { t.background };
        body = body.push(
            container(r)
                .width(Length::Fill)
                .style(move |_| solid_bg(bg, t.foreground, t.border)),
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
