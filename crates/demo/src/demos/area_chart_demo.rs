use iced::{
    Element, Length,
    widget::{canvas::Canvas, column},
};

use crate::{
    Message,
    components::chart::area::AreaChart,
    demos::common::{section_caption, section_title},
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let requests: Vec<(f32, f32)> = (0..24)
        .map(|i| {
            let x = i as f32;
            let y = 40.0 + 18.0 * (x * 0.45).sin() + 6.0 * (x * 0.2).cos() + x * 0.6;
            (x, y)
        })
        .collect();
    let errors: Vec<(f32, f32)> = (0..24)
        .map(|i| {
            let x = i as f32;
            let y = 4.0 + 3.0 * (x * 0.6).sin().abs();
            (x, y)
        })
        .collect();

    let labels = (0..24)
        .filter(|i| i % 4 == 0)
        .map(|i| format!("{i}h"))
        .collect();

    let chart: Element<Message> = Canvas::new(
        AreaChart::new(
            theme,
            vec![
                ("Requests".into(), requests),
                ("Errors".into(), errors),
            ],
        )
        .x_labels(labels),
    )
    .width(Length::Fill)
    .height(Length::Fixed(280.0))
    .into();

    column![
        section_title(theme, "Area chart"),
        section_caption(
            theme,
            "Each series fills to the zero baseline with a translucent band.",
        ),
        chart,
    ]
    .spacing(10)
    .into()
}
