use iced::{Element, widget::{column, row, text}};

use crate::{
    Message, State,
    components::{
        button::{button_ex, Variant},
        notification::{notification_layer, NotificationKind},
    },
    demos::common::{section_caption, section_title, vspace},
    theme::{AppTheme, Size},
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let controls = row![
        button_ex(theme, "Info", Variant::Info, Size::Sm, Some(Message::NotificationPush(NotificationKind::Info)), false, false),
        button_ex(theme, "Success", Variant::Success, Size::Sm, Some(Message::NotificationPush(NotificationKind::Success)), false, false),
        button_ex(theme, "Warning", Variant::Warning, Size::Sm, Some(Message::NotificationPush(NotificationKind::Warning)), false, false),
        button_ex(theme, "Error", Variant::Danger, Size::Sm, Some(Message::NotificationPush(NotificationKind::Error)), false, false),
    ]
    .spacing(8);

    let body: Element<'a, Message> = column![
        section_title(theme, "Toast notifications"),
        section_caption(theme, "Pushed entries auto-dismiss after 5 s; the close button dismisses immediately."),
        controls,
        vspace(8.0),
        text(format!("Active: {}", state.notifications.items.len()))
            .size(12.0)
            .color(theme.muted_foreground),
    ]
    .spacing(10)
    .into();

    notification_layer(theme, body, &state.notifications, Message::NotificationDismiss)
}
