use iced::{
    Element, Length,
    widget::{canvas::Canvas, column, row},
};

use crate::{
    Message,
    components::chart::{pie::PieChart, PieSlice},
    demos::common::{section_caption, section_title, vspace},
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let make_slices = || {
        vec![
            PieSlice::new("Equities", 46.0),
            PieSlice::new("Bonds", 22.0),
            PieSlice::new("Cash", 12.0),
            PieSlice::new("Crypto", 10.0),
            PieSlice::new("Commodities", 10.0),
        ]
    };

    let pie: Element<Message> = Canvas::new(PieChart::new(theme, make_slices()))
        .width(Length::Fill)
        .height(Length::Fixed(260.0))
        .into();

    let donut: Element<Message> = Canvas::new(PieChart::new(theme, make_slices()).donut(0.55))
        .width(Length::Fill)
        .height(Length::Fixed(260.0))
        .into();

    column![
        section_title(theme, "Pie / donut chart"),
        section_caption(
            theme,
            "Pie slices rendered from a shared dataset; donut variant sets an inner radius ratio.",
        ),
        row![pie, donut].spacing(16),
        vspace(4.0),
        section_caption(
            theme,
            "Each slice's legend entry shows its percentage of the total.",
        ),
    ]
    .spacing(10)
    .into()
}
