use iced::{Element, widget::column};

use crate::{
    Message,
    components::breadcrumb::{breadcrumb, Crumb},
    demos::common::section_title,
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let basic = breadcrumb(
        theme,
        vec![
            Crumb::new("Home").on_press(Message::BreadcrumbPressed(0)),
            Crumb::new("Components").on_press(Message::BreadcrumbPressed(1)),
            Crumb::new("Layout").on_press(Message::BreadcrumbPressed(2)),
            Crumb::new("Breadcrumb"),
        ],
    );

    let long = breadcrumb(
        theme,
        vec![
            Crumb::new("Users").on_press(Message::BreadcrumbPressed(0)),
            Crumb::new("Settings").on_press(Message::BreadcrumbPressed(1)),
            Crumb::new("Profile"),
        ],
    );

    column![
        section_title(theme, "Navigation trail"),
        basic,
        section_title(theme, "Trailing leaf"),
        long,
    ]
    .spacing(12)
    .into()
}
