use iced::{Element, widget::{column, text}};

use crate::{
    Message, State,
    components::otp_input::otp_input,
    demos::common::{section_caption, section_title, vspace},
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let six = otp_input(theme, &state.otp_value, 6, Message::OtpChanged);
    let four = otp_input(theme, &state.otp_short, 4, Message::OtpShortChanged);

    column![
        section_title(theme, "6-digit code"),
        section_caption(theme, "Typed value is stored in state; last character per box wins."),
        six,
        text(format!("Value: \"{}\"", state.otp_value)).size(12.0).color(theme.muted_foreground),
        vspace(14.0),
        section_title(theme, "4-digit"),
        four,
        text(format!("Value: \"{}\"", state.otp_short)).size(12.0).color(theme.muted_foreground),
    ]
    .spacing(10)
    .into()
}
