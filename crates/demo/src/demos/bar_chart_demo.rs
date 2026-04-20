use iced::{Element, widget::column};

use crate::{
    Message,
    components::chart::bar_chart,
    demos::common::{section_caption, section_title},
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let categories = vec!["Q1", "Q2", "Q3", "Q4"]
        .into_iter()
        .map(String::from)
        .collect();

    let groups = vec![
        (String::from("2023"), vec![42.0, 58.0, 51.0, 66.0]),
        (String::from("2024"), vec![55.0, 62.0, 71.0, 80.0]),
        (String::from("2025"), vec![66.0, 74.0, 83.0, 91.0]),
    ];

    column![
        section_title(theme, "Bar chart"),
        section_caption(
            theme,
            "Grouped bars across four quarters; each year sits in its own band slot.",
        ),
        bar_chart::<Message>(theme, categories, groups),
    ]
    .spacing(10)
    .into()
}
