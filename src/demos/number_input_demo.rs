use iced::{Element, widget::column};

use crate::{
    Message, State,
    components::number_input::{number_input, number_input_sized},
    demos::common::{section_caption, section_title, vspace},
    theme::{AppTheme, Size},
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let basic = number_input(theme, state.number_value, Message::NumberChanged, 1.0, None, None);
    let bounded = number_input(theme, state.number_bounded, Message::NumberBoundedChanged, 1.0, Some(0.0), Some(10.0));
    let decimal = number_input(theme, state.number_decimal, Message::NumberDecimalChanged, 0.25, None, None);

    let sizes = column![
        number_input_sized(theme, Size::Xs, state.number_value, Message::NumberChanged, 1.0, None, None),
        number_input_sized(theme, Size::Sm, state.number_value, Message::NumberChanged, 1.0, None, None),
        number_input_sized(theme, Size::Md, state.number_value, Message::NumberChanged, 1.0, None, None),
        number_input_sized(theme, Size::Lg, state.number_value, Message::NumberChanged, 1.0, None, None),
    ]
    .spacing(8);

    column![
        section_title(theme, "Basic"),
        section_caption(theme, "Type or click the steppers to change the value."),
        basic,
        vspace(10.0),
        section_title(theme, "Bounded [0, 10]"),
        bounded,
        vspace(10.0),
        section_title(theme, "Step 0.25"),
        decimal,
        vspace(10.0),
        section_title(theme, "Sizes"),
        sizes,
    ]
    .spacing(10)
    .into()
}
