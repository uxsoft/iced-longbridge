use iced::{
    Element,
    widget::{column, text, Space},
};

use crate::{
    Message,
    components::alert::{alert, AlertVariant},
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);

    column![
        header("Variants"),
        alert(theme, AlertVariant::Default, "Heads up!", "You can add components and dependencies to your app using the CLI."),
        alert(theme, AlertVariant::Info, "Information", "Your session will expire in 15 minutes. Please save your work."),
        alert(theme, AlertVariant::Success, "Success", "Your changes have been saved successfully."),
        alert(theme, AlertVariant::Warning, "Warning", "Your storage is almost full. Consider upgrading your plan."),
        alert(theme, AlertVariant::Danger, "Error", "Something went wrong. Please try again later."),
        Space::new().height(iced::Length::Fixed(0.0)),
    ]
    .spacing(12)
    .max_width(720)
    .into()
}
