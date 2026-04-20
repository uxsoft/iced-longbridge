use iced::{
    Element, Length,
    widget::{column, container, row, text, Space},
};

use crate::{
    Message, State,
    components::{
        button::{button_ex, Variant},
        sheet::{sheet, Side},
    },
    demos::common::{section_caption, section_title},
    theme::{AppTheme, Size},
};

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let triggers = row![
        button_ex(theme, "Left", Variant::Outline, Size::Md, Some(Message::SheetOpen(Side::Left)), false, false),
        button_ex(theme, "Right", Variant::Outline, Size::Md, Some(Message::SheetOpen(Side::Right)), false, false),
        button_ex(theme, "Top", Variant::Outline, Size::Md, Some(Message::SheetOpen(Side::Top)), false, false),
        button_ex(theme, "Bottom", Variant::Outline, Size::Md, Some(Message::SheetOpen(Side::Bottom)), false, false),
    ]
    .spacing(8);

    let base: Element<'a, Message> = column![
        section_title(theme, "Sheet"),
        section_caption(theme, "Slide-in side panel over a backdrop. Click the backdrop to dismiss."),
        triggers,
        Space::new().height(Length::Fixed(16.0)),
        text("Pick a direction above.").size(13.0).color(theme.muted_foreground),
        // Give the stack something to overlay.
        container(Space::new().width(Length::Fill).height(Length::Fixed(320.0))),
    ]
    .spacing(10)
    .into();

    let Some(side) = state.sheet_open else {
        return base;
    };

    let panel: Element<'a, Message> = column![
        text("Sheet panel").size(18.0).color(theme.foreground),
        text(format!("Side: {:?}", side)).size(13.0).color(theme.muted_foreground),
        Space::new().height(Length::Fixed(12.0)),
        text("Sheets are useful for settings, filters, or richer context that doesn't fit in a popover.").size(13.0).color(theme.foreground),
        Space::new().height(Length::Fill),
        button_ex(theme, "Close", Variant::Primary, Size::Md, Some(Message::SheetClose), false, false),
    ]
    .spacing(8)
    .into();

    sheet(theme, base, true, side, Message::SheetClose, panel)
}
