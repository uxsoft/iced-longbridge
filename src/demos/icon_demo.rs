use iced::{
    Element,
    widget::{column, row, text, Space},
};

use crate::{
    Message,
    components::icon::{icon, icon_colored, IconName},
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);

    let all: &[IconName] = &[
        IconName::Check, IconName::Close, IconName::Info, IconName::Warning,
        IconName::Success, IconName::Error, IconName::Search, IconName::Plus,
        IconName::Minus, IconName::ChevronDown, IconName::ChevronUp, IconName::ChevronLeft,
        IconName::ChevronRight, IconName::Star, IconName::Heart, IconName::Home,
        IconName::Settings, IconName::User, IconName::Mail, IconName::Phone,
        IconName::Lock, IconName::Unlock, IconName::Menu, IconName::Dots,
        IconName::Calendar, IconName::Clock, IconName::File, IconName::Folder,
        IconName::Trash, IconName::Edit, IconName::Copy, IconName::Save,
        IconName::Refresh, IconName::Download, IconName::Upload, IconName::Eye,
        IconName::EyeOff, IconName::Sun, IconName::Moon,
    ];

    let mut grid = column![].spacing(8);
    for chunk in all.chunks(8) {
        let mut r = row![].spacing(16).align_y(iced::alignment::Vertical::Center);
        for name in chunk {
            r = r.push(icon(theme, *name, 20.0));
        }
        grid = grid.push(r);
    }

    let sizes = row![
        icon(theme, IconName::Star, 12.0),
        icon(theme, IconName::Star, 16.0),
        icon(theme, IconName::Star, 20.0),
        icon(theme, IconName::Star, 24.0),
        icon(theme, IconName::Star, 32.0),
    ]
    .spacing(12)
    .align_y(iced::alignment::Vertical::Center);

    let colors = row![
        icon_colored(IconName::Heart, 24.0, theme.danger),
        icon_colored(IconName::Check, 24.0, theme.success),
        icon_colored(IconName::Warning, 24.0, theme.warning),
        icon_colored(IconName::Info, 24.0, theme.info),
        icon_colored(IconName::Star, 24.0, theme.primary),
    ]
    .spacing(12);

    column![
        header("All icons"),
        grid,
        Space::new().height(iced::Length::Fixed(12.0)),
        header("Sizes"),
        sizes,
        Space::new().height(iced::Length::Fixed(8.0)),
        header("Colors"),
        colors,
    ]
    .spacing(12)
    .into()
}
