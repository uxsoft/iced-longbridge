use iced::{
    Element,
    widget::{column, text, Space},
};

use crate::{
    Message, State,
    components::switch::switch,
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);

    column![
        header("Basic"),
        switch(theme, state.switches[0], Some("Enable notifications".into()))
            .on_toggle(|v| Message::SwitchToggled(0, v)),
        switch(theme, state.switches[1], Some("Dark mode".into()))
            .on_toggle(|v| Message::SwitchToggled(1, v)),
        switch(theme, state.switches[2], Some("Auto-save changes".into()))
            .on_toggle(|v| Message::SwitchToggled(2, v)),
        Space::new().height(iced::Length::Fixed(8.0)),
        header("No label"),
        switch(theme, state.switches[3], None).on_toggle(|v| Message::SwitchToggled(3, v)),
        Space::new().height(iced::Length::Fixed(8.0)),
        header("Disabled"),
        switch(theme, false, Some("Disabled off".into())),
        switch(theme, true, Some("Disabled on".into())),
        Space::new().height(iced::Length::Fixed(8.0)),
        text(format!("States: {:?}", state.switches)).size(13.0).color(theme.muted_foreground),
    ]
    .spacing(12)
    .into()
}
