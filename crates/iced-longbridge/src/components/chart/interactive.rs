//! Shared infrastructure for interactive, hoverable charts.
//!
//! All chart `canvas::Program` implementations in this module use
//! [`ChartState`] as their state type. When the cursor is over a data item,
//! they populate `state.hover` with a [`ChartHover`] describing the item and
//! request a redraw.
//!
//! [`ChartWidget`] wraps the raw `Canvas` element and, on each frame, reads
//! that state from the canvas's widget tree. If a hover is active, the
//! wrapper returns a [`ChartTooltipOverlay`] as its `overlay()`, which renders
//! a real floating tooltip above the chart using iced's renderer primitives
//! (same pipeline any built-in widget uses). Because the state lives in the
//! canvas program, no demo-side message plumbing is required.

use iced::{
    Background, Border, Color, Element, Event, Length, Pixels, Point,
    Rectangle, Shadow, Size, Vector,
    advanced::{
        Clipboard, Shell,
        layout::{self, Layout},
        overlay,
        renderer,
        text::{self, LineHeight, Text},
        widget::{Operation, Tree, Widget},
    },
    alignment::Vertical,
    mouse,
};

use crate::theme::AppTheme;

#[derive(Debug, Clone, Default)]
pub struct ChartState {
    pub hover: Option<ChartHover>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChartHover {
    pub title: String,
    pub entries: Vec<ChartHoverEntry>,
    /// Anchor point, in the canvas's local coordinate system (0,0 = top-left
    /// of the canvas widget). The tooltip is placed adjacent to this point.
    pub anchor: Point,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChartHoverEntry {
    pub label: String,
    pub value: String,
    pub color: Color,
}

impl ChartHoverEntry {
    pub fn new(
        label: impl Into<String>,
        value: impl Into<String>,
        color: Color,
    ) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            color,
        }
    }
}

pub struct ChartWidget<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: text::Renderer,
{
    inner: Element<'a, Message, Theme, Renderer>,
    theme: AppTheme,
}

impl<'a, Message, Theme, Renderer> ChartWidget<'a, Message, Theme, Renderer>
where
    Renderer: text::Renderer,
{
    pub fn new(
        inner: impl Into<Element<'a, Message, Theme, Renderer>>,
        theme: &AppTheme,
    ) -> Self {
        Self {
            inner: inner.into(),
            theme: *theme,
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for ChartWidget<'_, Message, Theme, Renderer>
where
    Renderer: text::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.inner)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.inner.as_widget()]);
    }

    fn size(&self) -> Size<Length> {
        self.inner.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.inner.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.inner
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.inner.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.inner.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.inner.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        self.inner.as_widget_mut().operate(
            &mut tree.children[0],
            layout,
            renderer,
            operation,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let child_tree = &mut tree.children[0];

        let hover = child_tree
            .state
            .downcast_ref::<ChartState>()
            .hover
            .clone();

        let child_overlay = self.inner.as_widget_mut().overlay(
            child_tree,
            layout,
            renderer,
            viewport,
            translation,
        );

        let tooltip_overlay: Option<overlay::Element<'b, Message, Theme, Renderer>> =
            hover.map(|hover| {
                let bounds = layout.bounds();
                let anchor = Point::new(
                    bounds.x + translation.x + hover.anchor.x,
                    bounds.y + translation.y + hover.anchor.y,
                );
                overlay::Element::new(Box::new(ChartTooltipOverlay {
                    theme: self.theme,
                    hover,
                    anchor,
                }))
            });

        match (child_overlay, tooltip_overlay) {
            (None, None) => None,
            (Some(o), None) | (None, Some(o)) => Some(o),
            (Some(a), Some(b)) => {
                Some(overlay::Group::with_children(vec![a, b]).overlay())
            }
        }
    }
}

impl<'a, Message, Theme, Renderer> From<ChartWidget<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a + text::Renderer,
{
    fn from(w: ChartWidget<'a, Message, Theme, Renderer>) -> Self {
        Element::new(w)
    }
}

// ---- Tooltip overlay ----

const TT_PADDING_X: f32 = 10.0;
const TT_PADDING_Y: f32 = 8.0;
const TT_LINE_HEIGHT: f32 = 16.0;
const TT_TITLE_SIZE: f32 = 12.0;
const TT_BODY_SIZE: f32 = 11.0;
const TT_SWATCH: f32 = 9.0;
const TT_SWATCH_GAP: f32 = 7.0;
const TT_GAP: f32 = 14.0;
const TT_MIN_WIDTH: f32 = 140.0;
const TT_MAX_WIDTH: f32 = 280.0;

struct ChartTooltipOverlay {
    theme: AppTheme,
    hover: ChartHover,
    /// Anchor in absolute window coordinates.
    anchor: Point,
}

impl<Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for ChartTooltipOverlay
where
    Renderer: text::Renderer,
{
    fn layout(&mut self, _renderer: &Renderer, bounds: Size) -> layout::Node {
        // Width estimate: rough average char width per font size.
        // Shaped text will only be smaller than this approximation.
        let title_w = self.hover.title.chars().count() as f32 * TT_TITLE_SIZE * 0.55;
        let entry_w = self
            .hover
            .entries
            .iter()
            .map(|e| {
                // label + two spaces + value
                (e.label.chars().count() + e.value.chars().count() + 2) as f32
                    * TT_BODY_SIZE
                    * 0.58
                    + TT_SWATCH
                    + TT_SWATCH_GAP
            })
            .fold(0.0_f32, f32::max);

        let content_w = title_w.max(entry_w);
        let width = (content_w + TT_PADDING_X * 2.0)
            .clamp(TT_MIN_WIDTH, TT_MAX_WIDTH);

        let lines = 1.0 + self.hover.entries.len() as f32;
        let height = TT_PADDING_Y * 2.0 + lines * TT_LINE_HEIGHT;

        // Prefer right of anchor, centered vertically.
        let mut x = self.anchor.x + TT_GAP;
        let mut y = self.anchor.y - height / 2.0;

        if x + width > bounds.width {
            x = self.anchor.x - TT_GAP - width;
        }
        if x < 0.0 {
            x = 0.0;
        }
        if y < 0.0 {
            y = 0.0;
        }
        if y + height > bounds.height {
            y = (bounds.height - height).max(0.0);
        }

        layout::Node::new(Size::new(width, height))
            .translate(Vector::new(x, y))
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
    ) {
        let bounds = layout.bounds();

        // Soft drop shadow.
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: bounds.y + 3.0,
                    width: bounds.width,
                    height: bounds.height,
                },
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 6.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            },
            Background::Color(Color {
                a: 0.12,
                ..Color::BLACK
            }),
        );

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    color: self.theme.border,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            },
            Background::Color(self.theme.popover),
        );

        let font = renderer.default_font();

        let mut y = bounds.y + TT_PADDING_Y;
        renderer.fill_text(
            Text {
                content: self.hover.title.clone(),
                bounds: Size::new(
                    bounds.width - TT_PADDING_X * 2.0,
                    TT_LINE_HEIGHT,
                ),
                size: Pixels(TT_TITLE_SIZE),
                line_height: LineHeight::default(),
                font,
                align_x: text::Alignment::Left,
                align_y: Vertical::Top,
                shaping: text::Shaping::Advanced,
                wrapping: text::Wrapping::None,
            },
            Point::new(bounds.x + TT_PADDING_X, y),
            self.theme.popover_foreground,
            bounds,
        );
        y += TT_LINE_HEIGHT;

        for entry in &self.hover.entries {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: bounds.x + TT_PADDING_X,
                        y: y + (TT_LINE_HEIGHT - TT_SWATCH) / 2.0,
                        width: TT_SWATCH,
                        height: TT_SWATCH,
                    },
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 2.0.into(),
                    },
                    shadow: Shadow::default(),
                    snap: true,
                },
                Background::Color(entry.color),
            );

            let text_x = bounds.x + TT_PADDING_X + TT_SWATCH + TT_SWATCH_GAP;
            let text_w = (bounds.x + bounds.width - TT_PADDING_X - text_x).max(0.0);
            let line = format!("{}  {}", entry.label, entry.value);
            renderer.fill_text(
                Text {
                    content: line,
                    bounds: Size::new(text_w, TT_LINE_HEIGHT),
                    size: Pixels(TT_BODY_SIZE),
                    line_height: LineHeight::default(),
                    font,
                    align_x: text::Alignment::Left,
                    align_y: Vertical::Top,
                    shaping: text::Shaping::Advanced,
                    wrapping: text::Wrapping::None,
                },
                Point::new(text_x, y),
                self.theme.popover_foreground,
                bounds,
            );
            y += TT_LINE_HEIGHT;
        }
    }

    fn index(&self) -> f32 {
        100.0
    }
}
