use iced::{
    Element, Length,
    widget::{column, row, Space},
};

use crate::{
    Message, State,
    components::{
        button::{button_ex, Variant},
        stepper::{stepper, Orientation, Step},
    },
    demos::common::section_title,
    theme::{AppTheme, Size},
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let steps_h = || vec![
        Step::new("Account"),
        Step::new("Profile"),
        Step::new("Review"),
        Step::new("Done"),
    ];
    let steps_v = || vec![
        Step::new("Sign up").description("Create an account"),
        Step::new("Verify").description("Confirm email"),
        Step::new("Onboard").description("Tell us about yourself"),
        Step::new("Launch").description("Start using the app"),
    ];

    let controls = row![
        button_ex(
            theme,
            "Back",
            Variant::Outline,
            Size::Sm,
            Some(Message::StepperBack),
            false,
            state.stepper_current == 0,
        ),
        button_ex(
            theme,
            "Next",
            Variant::Primary,
            Size::Sm,
            Some(Message::StepperNext),
            false,
            state.stepper_current >= 3,
        ),
    ]
    .spacing(8);

    column![
        section_title(theme, "Horizontal stepper"),
        stepper(theme, steps_h(), state.stepper_current, Orientation::Horizontal),
        Space::new().height(Length::Fixed(12.0)),
        controls,
        Space::new().height(Length::Fixed(20.0)),
        section_title(theme, "Vertical stepper"),
        stepper(theme, steps_v(), state.stepper_current, Orientation::Vertical),
    ]
    .spacing(10)
    .into()
}
