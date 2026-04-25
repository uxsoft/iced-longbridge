//! Command palette — cmdk-style fuzzy launcher overlay.
//!
//! Composes existing primitives (input, list, kbd, icon) inside a top-anchored
//! overlay panel. The caller owns search state and filtering; this component
//! only renders. Use [`focus_input`] from `update` to focus the search box
//! when the palette opens.

use iced::{
    Background, Border, Color, Element, Length, Padding, Shadow, Task,
    advanced::widget::Id,
    alignment::{Horizontal, Vertical},
    widget::{column, container, mouse_area, operation, row, scrollable, stack, text, text_input, Space},
};

use crate::{
    components::{
        icon::{icon, IconName},
        input::input,
        kbd::kbd,
        list::{list_item, ListItem},
    },
    styles,
    theme::AppTheme,
};

pub const INPUT_ID_STR: &str = "command-palette-input";

/// Stable widget [`Id`] of the palette's search input. Use with [`focus_input`].
pub fn input_id() -> Id {
    Id::from(INPUT_ID_STR)
}

/// Focus the palette's search input. Return this from `update` when opening.
pub fn focus_input<Message: Send + 'static>() -> Task<Message> {
    operation::focus(input_id())
}

pub struct PaletteItem<'a, Message> {
    pub leading: Option<Element<'a, Message>>,
    pub primary: String,
    pub secondary: Option<String>,
    pub on_press: Message,
}

impl<'a, Message> PaletteItem<'a, Message> {
    pub fn new(primary: impl Into<String>, on_press: Message) -> Self {
        Self {
            leading: None,
            primary: primary.into(),
            secondary: None,
            on_press,
        }
    }

    pub fn secondary(mut self, s: impl Into<String>) -> Self {
        self.secondary = Some(s.into());
        self
    }

    pub fn leading(mut self, el: Element<'a, Message>) -> Self {
        self.leading = Some(el);
        self
    }
}

#[allow(clippy::too_many_arguments)]
pub fn command_palette<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    base: Element<'a, Message>,
    open: bool,
    query: &str,
    placeholder: &str,
    items: Vec<PaletteItem<'a, Message>>,
    selected: usize,
    on_query_change: impl Fn(String) -> Message + 'a,
    on_dismiss: Message,
    on_submit: Message,
    on_select_index: impl Fn(usize) -> Message + 'a + Copy,
) -> Element<'a, Message> {
    if !open {
        return base;
    }
    let t = *theme;

    let header = container(
        row![
            icon::<Message>(theme, IconName::Search, 16.0),
            input(theme, placeholder, query)
                .id(input_id())
                .on_input(on_query_change)
                .on_submit(on_submit)
                .size(15.0)
                .padding(Padding::from([8.0, 6.0]))
                .style(move |_, _| header_input_style(&t)),
        ]
        .spacing(10)
        .align_y(Vertical::Center),
    )
    .padding(Padding::from([6.0, 12.0]))
    .style(move |_| container::Style {
        background: None,
        text_color: Some(t.foreground),
        border: Border {
            color: t.border,
            width: 0.0,
            radius: 0.0.into(),
        },
        shadow: Shadow::default(),
        snap: true,
    });

    let body: Element<'a, Message> = if items.is_empty() {
        container(
            text("No results")
                .size(13.0)
                .color(t.muted_foreground),
        )
        .padding(Padding::from([24.0, 16.0]))
        .width(Length::Fill)
        .align_x(Horizontal::Center)
        .into()
    } else {
        let mut col = column![].spacing(2).padding(Padding::from([6.0, 6.0]));
        for (i, item) in items.into_iter().enumerate() {
            let mut li = ListItem::new(item.primary).selected(i == selected);
            if let Some(s) = item.secondary {
                li = li.secondary(s);
            }
            if let Some(l) = item.leading {
                li = li.leading(l);
            }
            li = li.on_press((on_select_index)(i));
            col = col.push(list_item(theme, li));
        }
        container(scrollable(col).height(Length::Shrink))
            .max_height(360.0)
            .width(Length::Fill)
            .into()
    };

    let footer = container(
        row![
            footer_hint(&t, "↑", "↓", "Navigate"),
            Space::new().width(Length::Fixed(16.0)),
            footer_hint_single(&t, "↵", "Select"),
            Space::new().width(Length::Fixed(16.0)),
            footer_hint_single(&t, "Esc", "Close"),
            Space::new().width(Length::Fill),
        ]
        .spacing(0)
        .align_y(Vertical::Center),
    )
    .padding(Padding::from([8.0, 12.0]))
    .width(Length::Fill)
    .style(move |_| container::Style {
        background: Some(Background::Color(t.muted)),
        text_color: Some(t.muted_foreground),
        border: Border {
            color: t.border,
            width: 0.0,
            radius: 0.0.into(),
        },
        shadow: Shadow::default(),
        snap: true,
    });

    let panel_inner = column![
        header,
        thin_divider(&t),
        body,
        thin_divider(&t),
        footer,
    ]
    .spacing(0);

    let panel = container(panel_inner)
        .width(Length::Fixed(560.0))
        .style(move |_| container::Style {
            background: Some(Background::Color(t.popover)),
            text_color: Some(t.popover_foreground),
            border: Border {
                color: t.border,
                width: 1.0,
                radius: 12.0.into(),
            },
            shadow: Shadow {
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.18,
                },
                offset: iced::Vector::new(0.0, 8.0),
                blur_radius: 24.0,
            },
            snap: true,
        });

    let backdrop = mouse_area(
        container(Space::new().width(Length::Fill).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| {
                styles::simple_container(Some(t.overlay), t.foreground, Color::TRANSPARENT, 0.0, 0.0)
            }),
    )
    .on_press(on_dismiss);

    let panel_layer = container(panel)
        .padding(Padding {
            top: 120.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Top);

    stack![base, backdrop, panel_layer].into()
}

fn header_input_style(t: &AppTheme) -> text_input::Style {
    text_input::Style {
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        icon: t.muted_foreground,
        placeholder: t.muted_foreground,
        value: t.foreground,
        selection: crate::theme::with_alpha(t.primary, 0.3),
    }
}

fn thin_divider<'a, Message: 'a>(t: &AppTheme) -> Element<'a, Message> {
    let tt = *t;
    container(Space::new().width(Length::Fill).height(Length::Fixed(1.0)))
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(tt.border)),
            text_color: Some(tt.foreground),
            border: Border::default(),
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}

fn footer_hint<'a, Message: 'a>(
    t: &AppTheme,
    a: &str,
    b: &str,
    label: &str,
) -> Element<'a, Message> {
    row![
        kbd(t, a.to_string()),
        kbd(t, b.to_string()),
        text(label.to_string()).size(11.0).color(t.muted_foreground),
    ]
    .spacing(6)
    .align_y(Vertical::Center)
    .into()
}

fn footer_hint_single<'a, Message: 'a>(
    t: &AppTheme,
    a: &str,
    label: &str,
) -> Element<'a, Message> {
    row![
        kbd(t, a.to_string()),
        text(label.to_string()).size(11.0).color(t.muted_foreground),
    ]
    .spacing(6)
    .align_y(Vertical::Center)
    .into()
}

