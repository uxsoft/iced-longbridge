use iced::{
    Element,
    widget::{column, row, text, Space},
};

use crate::{
    Message, State,
    components::checkbox::checkbox_styled,
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);

    let basic = column![
        checkbox_styled(theme, "Accept terms and conditions", state.checkboxes[0])
            .on_toggle(|v| Message::CheckboxToggled(0, v)),
        checkbox_styled(theme, "Subscribe to newsletter", state.checkboxes[1])
            .on_toggle(|v| Message::CheckboxToggled(1, v)),
        checkbox_styled(theme, "Remember me", state.checkboxes[2])
            .on_toggle(|v| Message::CheckboxToggled(2, v)),
    ]
    .spacing(10);

    let disabled = row![
        checkbox_styled(theme, "Disabled unchecked", false),
        Space::new().width(iced::Length::Fixed(16.0)),
        checkbox_styled(theme, "Disabled checked", true),
    ]
    .spacing(8);

    column![
        header("Basic"),
        basic,
        Space::new().height(iced::Length::Fixed(8.0)),
        header("Disabled"),
        disabled,
    ]
    .spacing(12)
    .into()
}
