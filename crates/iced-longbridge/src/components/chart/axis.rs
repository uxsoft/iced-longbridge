//! Axis rendering helpers — draw ticks, labels, gridlines on a `canvas::Frame`.

use iced::{
    Color, Pixels, Point, Rectangle,
    advanced::text::Alignment as TextAlign,
    alignment::Vertical,
    widget::canvas::{self, Frame, Path, Stroke, Text},
};

use crate::theme::AppTheme;

use super::scale::Linear;

/// Plot area — the inner rectangle where chart geometry is drawn. Everything
/// outside is reserved for axis labels.
#[derive(Debug, Clone, Copy)]
pub struct PlotArea {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl PlotArea {
    pub fn from_bounds(bounds: Rectangle, padding: PlotPadding) -> Self {
        Self {
            left: padding.left,
            top: padding.top,
            right: bounds.width - padding.right,
            bottom: bounds.height - padding.bottom,
        }
    }

    pub fn width(&self) -> f32 {
        (self.right - self.left).max(0.0)
    }

    pub fn height(&self) -> f32 {
        (self.bottom - self.top).max(0.0)
    }

    pub fn x_range(&self) -> (f32, f32) {
        (self.left, self.right)
    }

    /// Y axis grows upwards in data space but pixel y grows downwards —
    /// so the range returned is (bottom, top) for a standard orientation.
    pub fn y_range(&self) -> (f32, f32) {
        (self.bottom, self.top)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlotPadding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl PlotPadding {
    pub const DEFAULT: Self = Self {
        top: 14.0,
        right: 14.0,
        bottom: 28.0,
        left: 44.0,
    };
}

/// Draw a y-axis with horizontal gridlines and numeric labels to the left.
pub fn draw_y_axis(
    frame: &mut Frame,
    theme: &AppTheme,
    area: PlotArea,
    scale: Linear,
    tick_count: usize,
    fmt: impl Fn(f32) -> String,
) {
    let ticks = scale.ticks(tick_count);

    for t in &ticks {
        let y = scale.to_pixel(*t);
        if y < area.top - 0.5 || y > area.bottom + 0.5 {
            continue;
        }

        let grid = Path::line(
            Point::new(area.left, y),
            Point::new(area.right, y),
        );
        frame.stroke(
            &grid,
            Stroke::default()
                .with_color(with_alpha(theme.border, 0.5))
                .with_width(1.0),
        );

        frame.fill_text(Text {
            content: fmt(*t),
            position: Point::new(area.left - 6.0, y),
            color: theme.muted_foreground,
            size: Pixels(10.0),
            align_x: TextAlign::Right,
            align_y: Vertical::Center,
            ..Default::default()
        });
    }

    let baseline = Path::line(
        Point::new(area.left, area.bottom),
        Point::new(area.right, area.bottom),
    );
    frame.stroke(
        &baseline,
        Stroke::default().with_color(theme.border).with_width(1.0),
    );
}

/// Draw x-axis tick labels from `positions` (pixel x) and their labels.
pub fn draw_x_labels(
    frame: &mut Frame,
    theme: &AppTheme,
    area: PlotArea,
    positions: &[(f32, String)],
) {
    for (x, label) in positions {
        frame.fill_text(Text {
            content: label.clone(),
            position: Point::new(*x, area.bottom + 6.0),
            color: theme.muted_foreground,
            size: Pixels(10.0),
            align_x: TextAlign::Center,
            align_y: Vertical::Top,
            ..Default::default()
        });
    }
}

/// Draw the standard chart bounding frame (1-pixel border around the plot
/// area).
#[allow(dead_code)]
pub fn draw_frame(frame: &mut Frame, theme: &AppTheme, area: PlotArea) {
    let rect = Path::rectangle(
        Point::new(area.left, area.top),
        iced::Size::new(area.width(), area.height()),
    );
    frame.stroke(
        &rect,
        Stroke::default().with_color(theme.border).with_width(1.0),
    );
}

pub fn with_alpha(color: Color, a: f32) -> Color {
    Color { a, ..color }
}

/// Helper used by each chart's `Program::draw` to pick its background.
pub fn fill_background(
    frame: &mut Frame,
    bounds: Rectangle,
    theme: &AppTheme,
) {
    frame.fill_rectangle(
        Point::new(0.0, 0.0),
        bounds.size(),
        canvas::Fill::from(theme.card),
    );
}
