use iced::{Element, widget::{column, row, text}};

use crate::{
    Message, State,
    components::color_picker::{color_picker, format_hex},
    demos::common::{section_caption, section_title},
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let picker = color_picker(
        theme,
        state.color_value,
        &state.color_hex,
        state.color_picker_open,
        Message::ColorPickerToggle,
        Message::ColorPickerChanged,
        Message::ColorPickerHexChanged,
    );

    column![
        section_title(theme, "Color picker"),
        section_caption(theme, "Click the swatch to open the panel. Featured palette, HSL / alpha sliders, and a hex input."),
        row![
            picker,
            text(format_hex(state.color_value)).size(13.0).color(theme.foreground),
        ]
        .spacing(16),
    ]
    .spacing(10)
    .into()
}
