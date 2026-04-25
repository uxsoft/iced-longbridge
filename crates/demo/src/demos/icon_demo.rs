use iced::{
    Element,
    widget::{column, row, text, Space},
};

use crate::{
    Message,
    components::icon::{icon, icon_colored, Icon},
    lucide,
    theme::AppTheme,
};

const LEMUR_SVG: &[u8] = include_bytes!("../../assets/lemur.svg");

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    let header = |s: &str| text(s.to_string()).size(16.0).color(theme.foreground);
    let label = |s: &str| text(s.to_string()).size(10.0).color(theme.muted_foreground);

    let all: Vec<(&str, Icon)> = vec![
        ("arrow-down", lucide::arrow_down()),
        ("arrow-down-wide-narrow", lucide::arrow_down_wide_narrow()),
        ("arrow-left", lucide::arrow_left()),
        ("arrow-right", lucide::arrow_right()),
        ("arrow-up", lucide::arrow_up()),
        ("arrow-up-narrow-wide", lucide::arrow_up_narrow_wide()),
        ("calendar", lucide::calendar()),
        ("check", lucide::check()),
        ("chevron-down", lucide::chevron_down()),
        ("chevron-left", lucide::chevron_left()),
        ("chevron-right", lucide::chevron_right()),
        ("chevron-up", lucide::chevron_up()),
        ("circle", lucide::circle()),
        ("circle-check", lucide::circle_check()),
        ("circle-x", lucide::circle_x()),
        ("clock", lucide::clock()),
        ("copy", lucide::copy()),
        ("download", lucide::download()),
        ("ellipsis", lucide::ellipsis()),
        ("ellipsis-vertical", lucide::ellipsis_vertical()),
        ("eye", lucide::eye()),
        ("eye-off", lucide::eye_off()),
        ("file", lucide::file()),
        ("filter", lucide::filter()),
        ("folder", lucide::folder()),
        ("grip-horizontal", lucide::grip_horizontal()),
        ("grip-vertical", lucide::grip_vertical()),
        ("heart", lucide::heart()),
        ("house", lucide::house()),
        ("info", lucide::info()),
        ("lock", lucide::lock()),
        ("lock-open", lucide::lock_open()),
        ("mail", lucide::mail()),
        ("menu", lucide::menu()),
        ("minus", lucide::minus()),
        ("moon", lucide::moon()),
        ("palette", lucide::palette()),
        ("pencil", lucide::pencil()),
        ("phone", lucide::phone()),
        ("plus", lucide::plus()),
        ("refresh-cw", lucide::refresh_cw()),
        ("save", lucide::save()),
        ("search", lucide::search()),
        ("settings", lucide::settings()),
        ("square", lucide::square()),
        ("square-minus", lucide::square_minus()),
        ("star", lucide::star()),
        ("sun", lucide::sun()),
        ("trash-2", lucide::trash_2()),
        ("triangle-alert", lucide::triangle_alert()),
        ("upload", lucide::upload()),
        ("user", lucide::user()),
        ("x", lucide::x()),
    ];

    let mut grid = column![].spacing(12);
    for chunk in all.chunks(8) {
        let mut r = row![].spacing(16).align_y(iced::alignment::Vertical::Top);
        for (name, ic) in chunk {
            let cell = column![
                icon(theme, ic.clone(), 20.0),
                label(name),
            ]
            .spacing(4)
            .align_x(iced::alignment::Horizontal::Center)
            .width(iced::Length::Fixed(110.0));
            r = r.push(cell);
        }
        grid = grid.push(r);
    }

    let sizes = row![
        icon(theme, lucide::star(), 12.0),
        icon(theme, lucide::star(), 16.0),
        icon(theme, lucide::star(), 20.0),
        icon(theme, lucide::star(), 24.0),
        icon(theme, lucide::star(), 32.0),
    ]
    .spacing(12)
    .align_y(iced::alignment::Vertical::Center);

    let colors = row![
        icon_colored(lucide::heart(), 24.0, theme.danger),
        icon_colored(lucide::check(), 24.0, theme.success),
        icon_colored(lucide::triangle_alert(), 24.0, theme.warning),
        icon_colored(lucide::info(), 24.0, theme.info),
        icon_colored(lucide::star(), 24.0, theme.primary),
    ]
    .spacing(12);

    let custom = row![
        icon(theme, lucide::star(), 24.0),
        icon(theme, Icon::from_svg_bytes(LEMUR_SVG), 24.0),
        icon_colored(Icon::from_svg_bytes(LEMUR_SVG), 24.0, theme.primary),
        icon_colored(Icon::from_svg_bytes(LEMUR_SVG), 32.0, theme.danger),
    ]
    .spacing(12)
    .align_y(iced::alignment::Vertical::Center);

    column![
        header("All icons (Lucide)"),
        grid,
        Space::new().height(iced::Length::Fixed(12.0)),
        header("Sizes"),
        sizes,
        Space::new().height(iced::Length::Fixed(8.0)),
        header("Colors"),
        colors,
        Space::new().height(iced::Length::Fixed(8.0)),
        header("Custom SVG (caller-supplied via include_bytes!)"),
        custom,
    ]
    .spacing(12)
    .into()
}
