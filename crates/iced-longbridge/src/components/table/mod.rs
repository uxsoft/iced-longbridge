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

mod render;

use iced::{Element, Length, Point, alignment::Horizontal};

use crate::{
    components::{icon::Icon, keyboard_focus},
    theme::AppTheme,
};

/// Re-export of the generic [`crate::components::keyboard_focus::NavEvent`]
/// so callers can still write `iced_longbridge::components::table::NavEvent`.
pub use keyboard_focus::NavEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDir {
    Asc,
    Desc,
}

/// Per-row visual override returned from [`Table::row_style`]. `None` fields
/// fall back to the table's defaults (zebra striping + theme foreground).
#[derive(Debug, Clone, Copy, Default)]
pub struct RowStyle {
    pub background: Option<iced::Color>,
    pub text_color: Option<iced::Color>,
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
    #[allow(clippy::type_complexity)]
    row_style: Option<Box<dyn Fn(usize, &T) -> RowStyle + 'a>>,
    on_row_press: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    on_nav: Option<Box<dyn Fn(NavEvent) -> Message + 'a>>,
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
        row_style: None,
        on_row_press: None,
        on_nav: None,
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

    /// Override the default zebra striping per row. Returning a [`RowStyle`]
    /// with `None` fields falls back to the table's defaults (zebra background
    /// and theme foreground), so callers can tint only the rows that need it.
    pub fn row_style<F>(mut self, f: F) -> Self
    where
        F: Fn(usize, &T) -> RowStyle + 'a,
    {
        self.row_style = Some(Box::new(f));
        self
    }

    /// Fire `f(i)` when any cell in row `i` is clicked. Wraps each row in a
    /// `mouse_area` with `Interaction::Pointer` so the cursor signals
    /// clickability.
    pub fn on_row_press<F>(mut self, f: F) -> Self
    where
        F: Fn(usize) -> Message + 'a,
    {
        self.on_row_press = Some(Box::new(f));
        self
    }

    /// Wire keyboard navigation. When set, the rendered table claims focus on
    /// left-click anywhere inside its bounds (and releases focus on a click
    /// outside), and translates Up/Down/Home/End/PageUp/PageDown/Enter into
    /// [`NavEvent`]s while focused. Other keys fall through.
    ///
    /// The caller owns the focused-row index — keep it in your view state and
    /// reflect it visually via [`Table::row_style`]. `PageUp` / `PageDown`
    /// don't carry a row count; the caller decides how many rows to advance.
    pub fn navigation<F>(mut self, on_nav: F) -> Self
    where
        F: Fn(NavEvent) -> Message + 'a,
    {
        self.on_nav = Some(Box::new(on_nav));
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
        render::render(table)
    }
}
