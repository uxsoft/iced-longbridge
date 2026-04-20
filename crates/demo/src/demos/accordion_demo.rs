use iced::{
    Element, Length,
    widget::{column, text, Space},
};

use crate::{
    Message, State,
    components::{
        accordion::{accordion, Section},
        divider,
    },
    demos::common::{section_title, vspace},
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let sections = vec![
        Section {
            title: "What is iced-longbridge?".into(),
            expanded: state.accordion_open[0],
            on_toggle: Message::AccordionToggled(0),
            content: text(
                "A faithful iced 0.14 port of the longbridge/gpui-component library — same look, same feel, iced primitives.",
            )
            .size(13.0)
            .color(theme.muted_foreground)
            .into(),
        },
        Section {
            title: "How do variants work?".into(),
            expanded: state.accordion_open[1],
            on_toggle: Message::AccordionToggled(1),
            content: text(
                "Every styled component reads from a single AppTheme and selects colors per variant: Primary, Secondary, Danger, Success, Warning, Info, and more.",
            )
            .size(13.0)
            .color(theme.muted_foreground)
            .into(),
        },
        Section {
            title: "Can I use custom themes?".into(),
            expanded: state.accordion_open[2],
            on_toggle: Message::AccordionToggled(2),
            content: column![
                text("Yes. Start with AppTheme::light() or AppTheme::dark() and override individual fields.").size(13.0).color(theme.muted_foreground),
                vspace(6.0),
                text("The palette mirrors shadcn/ui's neutral scale.").size(13.0).color(theme.muted_foreground),
            ].spacing(0).into(),
        },
    ];

    column![
        section_title(theme, "Basic accordion"),
        accordion(theme, sections),
        Space::new().height(Length::Fixed(16.0)),
        section_title(theme, "Divider between sections"),
        divider::horizontal(theme),
    ]
    .spacing(10)
    .into()
}
