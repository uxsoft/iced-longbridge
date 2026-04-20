use iced::{Element, widget::{column, text}};

use crate::{
    Message,
    components::group_box::group_box,
    demos::common::{section_title, vspace},
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let profile = group_box(
        theme,
        "Profile",
        column![
            text("Username").size(13.0).color(theme.muted_foreground),
            text("Jan Dryk").size(14.0).color(theme.foreground),
            vspace(8.0),
            text("Email").size(13.0).color(theme.muted_foreground),
            text("me@uxsoft.cz").size(14.0).color(theme.foreground),
        ]
        .spacing(0)
        .into(),
    );

    let notifications = group_box(
        theme,
        "Notifications",
        column![
            text("Email updates on activity.").size(13.0).color(theme.foreground),
            vspace(6.0),
            text("Push notifications disabled.").size(13.0).color(theme.muted_foreground),
        ]
        .spacing(0)
        .into(),
    );

    column![
        section_title(theme, "Titled groups"),
        profile,
        notifications,
    ]
    .spacing(14)
    .into()
}
