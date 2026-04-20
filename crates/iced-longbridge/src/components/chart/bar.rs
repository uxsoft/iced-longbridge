//! Bar chart — grouped bars on a band x-scale.

use iced::{
    Color, Element, Length, Point, Rectangle, Renderer, Theme,
    mouse,
    widget::canvas::{self, Canvas, Frame, Geometry, Path},
};

use crate::theme::AppTheme;

use super::{
    axis::{draw_x_labels, draw_y_axis, fill_background, PlotArea, PlotPadding},
    line::format_tick,
    scale::{padded_domain, Band, Linear},
};

pub struct BarChart {
    pub theme: AppTheme,
    pub categories: Vec<String>,
    pub groups: Vec<(String, Vec<f32>, Option<Color>)>,
}

impl BarChart {
    pub fn new(
        theme: &AppTheme,
        categories: Vec<String>,
        groups: Vec<(String, Vec<f32>)>,
    ) -> Self {
        Self {
            theme: *theme,
            categories,
            groups: groups
                .into_iter()
                .map(|(n, v)| (n, v, None))
                .collect(),
        }
    }
}

pub fn bar_chart<'a, Message: 'a>(
    theme: &AppTheme,
    categories: Vec<String>,
    groups: Vec<(String, Vec<f32>)>,
) -> Element<'a, Message> {
    Canvas::new(BarChart::new(theme, categories, groups))
        .width(Length::Fill)
        .height(Length::Fixed(260.0))
        .into()
}

impl<Message> canvas::Program<Message> for BarChart {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _iced_theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        fill_background(&mut frame, bounds, &self.theme);

        let area = PlotArea::from_bounds(bounds, PlotPadding::DEFAULT);
        if area.width() <= 0.0 || area.height() <= 0.0 || self.categories.is_empty() {
            return vec![frame.into_geometry()];
        }

        let mut all_values: Vec<f32> =
            self.groups.iter().flat_map(|(_, v, _)| v.iter().copied()).collect();
        all_values.push(0.0);
        let (y_min_raw, y_max) = padded_domain(all_values.into_iter());
        let y_min = y_min_raw.min(0.0);

        let x_band = Band::new(self.categories.len(), area.x_range()).padding(0.25);
        let y_scale = Linear::new((y_min, y_max), area.y_range());

        draw_y_axis(&mut frame, &self.theme, area, y_scale, 5, format_tick);

        let labels: Vec<(f32, String)> = self
            .categories
            .iter()
            .enumerate()
            .map(|(i, l)| (x_band.center(i), l.clone()))
            .collect();
        draw_x_labels(&mut frame, &self.theme, area, &labels);

        let group_count = self.groups.len().max(1);
        let band_w = x_band.bandwidth();
        let bar_w = band_w / group_count as f32;
        let zero_y = y_scale.to_pixel(0.0);

        for (gi, (_, values, override_color)) in self.groups.iter().enumerate() {
            let color = override_color
                .unwrap_or(self.theme.chart[gi % self.theme.chart.len()]);
            for (i, v) in values.iter().enumerate() {
                if i >= self.categories.len() {
                    break;
                }
                let left = x_band.left(i) + bar_w * gi as f32;
                let top = y_scale.to_pixel(v.max(0.0));
                let bottom = if *v < 0.0 {
                    y_scale.to_pixel(*v)
                } else {
                    zero_y
                };
                let y = top.min(bottom);
                let h = (top - bottom).abs().max(1.0);
                let path = Path::rectangle(Point::new(left, y), iced::Size::new(bar_w - 2.0, h));
                frame.fill(&path, canvas::Fill::from(color));
            }
        }

        vec![frame.into_geometry()]
    }
}
