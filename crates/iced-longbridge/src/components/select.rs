//! Select — styled pick_list and filterable combo_box.

use std::borrow::Borrow;

use iced::{
    Background, Border, Padding, Shadow,
    overlay::menu,
    widget::{combo_box, pick_list, ComboBox, PickList},
};

use crate::theme::{AppTheme, Size};

pub fn select<'a, T, L, Message>(
    theme: &AppTheme,
    options: L,
    selected: Option<T>,
    on_select: impl Fn(T) -> Message + 'a,
) -> PickList<'a, T, L, T, Message>
where
    T: Clone + PartialEq + ToString + 'a,
    L: Borrow<[T]> + 'a,
    Message: Clone + 'a,
{
    select_sized(theme, Size::Md, options, selected, on_select)
}

pub fn select_sized<'a, T, L, Message>(
    theme: &AppTheme,
    size: Size,
    options: L,
    selected: Option<T>,
    on_select: impl Fn(T) -> Message + 'a,
) -> PickList<'a, T, L, T, Message>
where
    T: Clone + PartialEq + ToString + 'a,
    L: Borrow<[T]> + 'a,
    Message: Clone + 'a,
{
    let t = *theme;
    let radius = size.radius();
    let text_size = size.font_size();
    let py = size.padding_x() / 2.0;
    let px = size.padding_x();

    pick_list(options, selected, on_select)
        .text_size(text_size)
        .padding(Padding::from([py, px]))
        .style(move |_, status| {
            use pick_list::Status::*;
            let border_color = match status {
                Opened { .. } | Hovered => t.primary,
                Active => t.input_border,
            };
            pick_list::Style {
                background: Background::Color(t.background),
                text_color: t.foreground,
                placeholder_color: t.muted_foreground,
                handle_color: t.muted_foreground,
                border: Border {
                    color: border_color,
                    width: 1.0,
                    radius: radius.into(),
                },
            }
        })
        .menu_style(move |_| menu::Style {
            background: Background::Color(t.popover),
            border: Border {
                color: t.border,
                width: 1.0,
                radius: 6.0.into(),
            },
            text_color: t.popover_foreground,
            selected_text_color: t.primary_foreground,
            selected_background: Background::Color(t.primary),
            shadow: Shadow::default(),
        })
}

pub fn combobox<'a, T, Message>(
    _theme: &AppTheme,
    state: &'a combo_box::State<T>,
    placeholder: &str,
    selected: Option<&'a T>,
    on_selected: impl Fn(T) -> Message + 'static,
) -> ComboBox<'a, T, Message>
where
    T: std::fmt::Display + Clone + 'static,
    Message: Clone + 'static,
{
    let t = *_theme;
    combo_box(state, placeholder, selected, on_selected)
        .size(14.0)
        .padding(Padding::from([6.0, 10.0]))
        .input_style(move |_, status| {
            use iced::widget::text_input::Status::*;
            let border_color = match status {
                Focused { .. } => t.ring,
                Hovered => t.muted_foreground,
                _ => t.input_border,
            };
            iced::widget::text_input::Style {
                background: Background::Color(t.background),
                border: Border {
                    color: border_color,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                icon: t.muted_foreground,
                placeholder: t.muted_foreground,
                value: t.foreground,
                selection: crate::theme::with_alpha(t.primary, 0.3),
            }
        })
        .menu_style(move |_| menu::Style {
            background: Background::Color(t.popover),
            border: Border {
                color: t.border,
                width: 1.0,
                radius: 6.0.into(),
            },
            text_color: t.popover_foreground,
            selected_text_color: t.primary_foreground,
            selected_background: Background::Color(t.primary),
            shadow: Shadow::default(),
        })
}
