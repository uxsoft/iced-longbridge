//! Data table — thin wrapper over [`table`] for simple `Vec<Vec<String>>` data.

use iced::{
    Element, Length,
    alignment::Horizontal,
    widget::text,
};

use crate::{
    components::table::{table_with, Column, TableOptions},
    theme::AppTheme,
};

pub struct DataTable {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub widths: Vec<Length>,
    pub aligns: Vec<Horizontal>,
}

impl DataTable {
    pub fn new(headers: Vec<impl Into<String>>, rows: Vec<Vec<String>>) -> Self {
        let n = headers.len();
        Self {
            headers: headers.into_iter().map(Into::into).collect(),
            rows,
            widths: vec![Length::Fill; n],
            aligns: vec![Horizontal::Left; n],
        }
    }

    pub fn widths(mut self, widths: Vec<Length>) -> Self {
        self.widths = widths;
        self
    }

    pub fn aligns(mut self, aligns: Vec<Horizontal>) -> Self {
        self.aligns = aligns;
        self
    }
}

pub fn data_table<'a, Message: Clone + 'a>(
    theme: &AppTheme,
    data: &'a DataTable,
) -> Element<'a, Message> {
    let theme_copy = *theme;
    let mut columns: Vec<Column<'a, Vec<String>, Message>> = Vec::with_capacity(data.headers.len());
    for (idx, header) in data.headers.iter().enumerate() {
        let width = data.widths.get(idx).copied().unwrap_or(Length::Fill);
        let align = data.aligns.get(idx).copied().unwrap_or(Horizontal::Left);
        columns.push(
            Column::new(header.clone(), move |row: &Vec<String>| {
                let cell = row.get(idx).cloned().unwrap_or_default();
                text(cell).size(13.0).color(theme_copy.foreground).into()
            })
            .width(width)
            .align(align),
        );
    }
    table_with(theme, &data.rows, columns, TableOptions::default())
}
