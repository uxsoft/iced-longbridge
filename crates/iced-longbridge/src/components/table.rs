//! Table — column-and-row layout with typed cells.
//!
//! Build columns with [`Column::new`] and render rows from any `Vec<T>` via
//! [`table`]. Header is a sticky row; body is scrollable.

use iced::{
    Background, Border, Element, Length, Padding, Shadow,
    alignment::{Horizontal, Vertical},
    widget::{column, container, row, scrollable, text, Space},
};

use crate::{
    components::icon::{icon_colored, IconName},
    theme::AppTheme,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDir {
    Asc,
    Desc,
}

pub struct Column<'a, T, Message> {
    pub header: String,
    pub width: Length,
    pub align: Horizontal,
    #[allow(clippy::type_complexity)]
    pub render: Box<dyn Fn(&T) -> Element<'a, Message> + 'a>,
    pub sort_key: Option<&'static str>,
}

impl<'a, T, Message> Column<'a, T, Message> {
    pub fn new(
        header: impl Into<String>,
        render: impl Fn(&T) -> Element<'a, Message> + 'a,
    ) -> Self {
        Self {
            header: header.into(),
            width: Length::Fill,
            align: Horizontal::Left,
            render: Box::new(render),
            sort_key: None,
        }
    }

    pub fn width(mut self, w: Length) -> Self {
        self.width = w;
        self
    }

    pub fn align(mut self, a: Horizontal) -> Self {
        self.align = a;
        self
    }

    pub fn sortable(mut self, key: &'static str) -> Self {
        self.sort_key = Some(key);
        self
    }
}

pub struct TableOptions<'a, Message> {
    pub sort: Option<(&'static str, SortDir)>,
    pub on_sort: Option<Box<dyn Fn(&'static str) -> Message + 'a>>,
    pub striped: bool,
    pub row_height: f32,
}

impl<'a, Message> Default for TableOptions<'a, Message> {
    fn default() -> Self {
        Self {
            sort: None,
            on_sort: None,
            striped: true,
            row_height: 40.0,
        }
    }
}

#[allow(dead_code)]
pub fn table<'a, T, Message: Clone + 'a>(
    theme: &AppTheme,
    rows: &[T],
    columns: Vec<Column<'a, T, Message>>,
) -> Element<'a, Message> {
    table_with(theme, rows, columns, TableOptions::default())
}

pub fn table_with<'a, T, Message: Clone + 'a>(
    theme: &AppTheme,
    rows: &[T],
    columns: Vec<Column<'a, T, Message>>,
    options: TableOptions<'a, Message>,
) -> Element<'a, Message> {
    let t = *theme;
    let TableOptions {
        sort,
        on_sort,
        striped,
        row_height,
    } = options;

    // Header row.
    let mut header = row![].spacing(0);
    for col in &columns {
        let is_sorted = sort.map(|(k, _)| Some(k) == col.sort_key).unwrap_or(false);
        let sort_dir = if is_sorted { sort.map(|(_, d)| d) } else { None };

        let mut inner = row![text(col.header.clone()).size(12.0).color(t.muted_foreground)]
            .spacing(6)
            .align_y(Vertical::Center);
        if let Some(dir) = sort_dir {
            inner = inner.push(icon_colored::<Message>(
                match dir {
                    SortDir::Asc => IconName::SortAsc,
                    SortDir::Desc => IconName::SortDesc,
                },
                11.0,
                t.muted_foreground,
            ));
        }

        let cell = container(inner)
            .padding(Padding::from([0.0, 12.0]))
            .width(col.width)
            .height(Length::Fixed(36.0))
            .align_x(col.align)
            .align_y(Vertical::Center);

        // Sortable cells are wrapped in a button that emits on_sort.
        let cell_el: Element<Message> = match (col.sort_key, on_sort.as_ref()) {
            (Some(key), Some(cb)) => {
                let msg = cb(key);
                iced::widget::button(cell)
                    .padding(0)
                    .on_press(msg)
                    .style(move |_, status| {
                        use iced::widget::button::Status::*;
                        let bg = match status {
                            Hovered => t.accent,
                            Pressed => t.muted,
                            _ => iced::Color::TRANSPARENT,
                        };
                        iced::widget::button::Style {
                            background: Some(Background::Color(bg)),
                            text_color: t.foreground,
                            border: Border::default(),
                            shadow: Shadow::default(),
                            snap: true,
                        }
                    })
                    .into()
            }
            _ => cell.into(),
        };

        header = header.push(cell_el);
    }
    let header_bar = container(header)
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

    // Body rows.
    let mut body = column![].spacing(0);
    for (i, data) in rows.iter().enumerate() {
        let mut r = row![].spacing(0);
        for col in &columns {
            let cell = container((col.render)(data))
                .padding(Padding::from([0.0, 12.0]))
                .width(col.width)
                .height(Length::Fixed(row_height))
                .align_x(col.align)
                .align_y(Vertical::Center);
            r = r.push(cell);
        }
        let zebra = striped && i % 2 == 1;
        body = body.push(
            container(r)
                .width(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(if zebra {
                        t.muted
                    } else {
                        t.background
                    })),
                    text_color: Some(t.foreground),
                    border: Border {
                        color: t.border,
                        width: 0.0,
                        radius: 0.0.into(),
                    },
                    shadow: Shadow::default(),
                    snap: true,
                }),
        );
    }

    if rows.is_empty() {
        body = body.push(
            container(text("No rows").size(13.0).color(t.muted_foreground))
                .padding(Padding::from(24.0))
                .width(Length::Fill)
                .align_x(Horizontal::Center),
        );
    }

    let content = column![
        header_bar,
        container(Space::new().width(Length::Fill).height(Length::Fixed(1.0)))
            .style(move |_| container::Style {
                background: Some(Background::Color(t.border)),
                text_color: None,
                border: Border::default(),
                shadow: Shadow::default(),
                snap: true,
            }),
        scrollable(body).height(Length::Shrink),
    ]
    .spacing(0);

    container(content)
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(t.background)),
            text_color: Some(t.foreground),
            border: Border {
                color: t.border,
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}
