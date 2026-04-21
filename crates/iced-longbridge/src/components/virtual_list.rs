//! Virtual list — viewport-aware column that only renders visible rows.
//!
//! The host owns the current scroll offset (wired through [`on_scroll`]) and
//! re-passes it on every render. The component then renders only the rows that
//! fall inside the viewport plus a small buffer, with two [`Space`] spacers
//! above and below to preserve the total scroll height. This keeps memory and
//! layout costs constant regardless of item count.
//!
//! # Contract
//!
//! - **Rows must be a uniform fixed height** equal to `row_height`. Variable
//!   heights will break the offset math and desync the scrollbar.
//! - `row_height` must be positive; values ≤ 0 are clamped to 1.0.
//! - `render_row` is called once per visible index; it should not depend on
//!   hover state to change layout (the item may be re-indexed on scroll).
//! - The scroll offset passed back through `on_scroll` is the absolute pixel
//!   offset from the top — store it verbatim and pass it back on the next
//!   render.

use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    widget::{column, container, scrollable, Space},
};

use crate::theme::AppTheme;

/// Overscan applied on both sides of the viewport so quick scrolls don't reveal
/// whitespace.
const OVERSCAN: usize = 4;

pub fn virtual_list<'a, T, Message>(
    theme: &AppTheme,
    items: &'a [T],
    row_height: f32,
    visible_height: f32,
    scroll_offset: f32,
    render_row: impl Fn(&'a T, usize) -> Element<'a, Message>,
    on_scroll: impl Fn(scrollable::Viewport) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let t = *theme;
    let total = items.len();
    let row_h = row_height.max(1.0);

    let first = ((scroll_offset / row_h).floor() as usize).saturating_sub(OVERSCAN);
    let visible = ((visible_height / row_h).ceil() as usize) + OVERSCAN * 2;
    let last = (first + visible).min(total);

    let leading_px = first as f32 * row_h;
    let trailing_px = (total.saturating_sub(last)) as f32 * row_h;

    let mut col = column![].spacing(0);
    if leading_px > 0.0 {
        col = col.push(Space::new().height(Length::Fixed(leading_px)));
    }
    for (i, item) in items.iter().enumerate().take(last).skip(first) {
        col = col.push(render_row(item, i));
    }
    if trailing_px > 0.0 {
        col = col.push(Space::new().height(Length::Fixed(trailing_px)));
    }

    container(
        scrollable(col)
            .on_scroll(on_scroll)
            .height(Length::Fixed(visible_height)),
    )
    .padding(Padding::from(0.0))
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
    .into()
}
