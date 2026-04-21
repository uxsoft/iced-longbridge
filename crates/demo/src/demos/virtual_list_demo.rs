use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    alignment::Vertical,
    widget::{container, row, text},
};

use crate::{
    Message, State,
    components::virtual_list::virtual_list,
    demos::common::{section_caption, section_title, vspace},
    theme::AppTheme,
};

const ROW_HEIGHT: f32 = 44.0;
const VIEWPORT_HEIGHT: f32 = 360.0;

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let t = *theme;
    let items = &state.virtual_list_items;
    let selected = state.virtual_list_selected;

    let list = virtual_list(
        theme,
        items,
        ROW_HEIGHT,
        VIEWPORT_HEIGHT,
        state.virtual_list_offset,
        move |label, index| row_view(t, label, index, selected == index),
        Message::VirtualListScrolled,
    );

    iced::widget::column![
        section_title(theme, "Virtual list"),
        section_caption(
            theme,
            format!(
                "Rendering {} rows by materialising only the ~{} visible ones.",
                items.len(),
                (VIEWPORT_HEIGHT / ROW_HEIGHT).ceil() as usize,
            ),
        ),
        vspace(4.0),
        list,
    ]
    .spacing(10)
    .into()
}

fn row_view<'a>(t: AppTheme, label: &'a String, index: usize, selected: bool) -> Element<'a, Message> {
    let bg = if selected { t.accent } else { t.background };
    let content = row![
        text(format!("{:05}", index)).size(12.0).color(t.muted_foreground),
        text(label.clone()).size(13.0).color(t.foreground),
    ]
    .spacing(16)
    .align_y(Vertical::Center);

    let inner = container(content)
        .padding(Padding::from([0.0, 14.0]))
        .width(Length::Fill)
        .height(Length::Fixed(ROW_HEIGHT));

    iced::widget::button(inner)
        .padding(Padding::from(0.0))
        .on_press(Message::VirtualListSelected(index))
        .style(move |_, status| {
            use iced::widget::button::Status::*;
            let bg = match status {
                Hovered => t.muted,
                Pressed => t.accent,
                _ => bg,
            };
            iced::widget::button::Style {
                background: Some(Background::Color(bg)),
                text_color: t.foreground,
                border: Border {
                    color: t.border,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            }
        })
        .into()
}
