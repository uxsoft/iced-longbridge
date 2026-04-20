use iced::{
    Element, Length,
    widget::{canvas::Canvas, column},
};

use crate::{
    Message,
    components::chart::{line::LineChart, Series},
    demos::common::{section_caption, section_title},
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let series_a = Series::new(
        "Revenue",
        (0..12)
            .map(|i| {
                let x = i as f32;
                let y = 120.0 + 40.0 * (x * 0.6).sin() + 8.0 * x;
                (x, y)
            })
            .collect(),
    );
    let series_b = Series::new(
        "Cost",
        (0..12)
            .map(|i| {
                let x = i as f32;
                let y = 80.0 + 30.0 * (x * 0.5).cos() + 4.0 * x;
                (x, y)
            })
            .collect(),
    );

    let months = (["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"])
        .iter()
        .map(|s| s.to_string())
        .collect();

    let chart: Element<Message> = Canvas::new(
        LineChart::new(theme, vec![series_a, series_b]).x_labels(months),
    )
    .width(Length::Fill)
    .height(Length::Fixed(280.0))
    .into();

    column![
        section_title(theme, "Line chart"),
        section_caption(
            theme,
            "Two numeric series rendered on a linear scale with dot markers.",
        ),
        chart,
    ]
    .spacing(10)
    .into()
}
