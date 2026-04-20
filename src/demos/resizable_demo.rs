use iced::{
    Element, Length,
    widget::{column, text},
};

use crate::{
    Message, State,
    components::resizable::{pane_content, resizable},
    demos::common::section_title,
    theme::AppTheme,
};

pub fn view<'a>(state: &'a State, theme: &'a AppTheme) -> Element<'a, Message> {
    let grid = resizable(
        theme,
        &state.resizable_state,
        Message::ResizableResized,
        move |_pane, data: &'a String, _is_max| {
            pane_content(
                theme,
                data.clone(),
                text(format!("Drag the splitter to resize — this is the {} panel.", data))
                    .size(13.0)
                    .color(theme.muted_foreground)
                    .into(),
            )
        },
    );

    column![
        section_title(theme, "Pane grid"),
        iced::widget::container(grid)
            .width(Length::Fill)
            .height(Length::Fixed(320.0)),
    ]
    .spacing(10)
    .into()
}
