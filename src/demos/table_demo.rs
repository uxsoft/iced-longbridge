use iced::{
    Element, Length,
    alignment::Horizontal,
    widget::{column, text},
};

use crate::{
    Message, State,
    components::{
        badge::{badge, BadgeVariant},
        table::{table_with, Column, SortDir, TableOptions},
    },
    demos::common::{section_caption, section_title, vspace},
    theme::AppTheme,
};

#[derive(Debug, Clone)]
pub struct Person {
    pub name: &'static str,
    pub role: &'static str,
    pub status: &'static str,
    pub salary: i64,
}

pub const PEOPLE: &[Person] = &[
    Person { name: "Ada Lovelace", role: "Engineering Lead", status: "Active", salary: 180_000 },
    Person { name: "Alan Turing", role: "Research Fellow", status: "Active", salary: 165_000 },
    Person { name: "Grace Hopper", role: "Staff Engineer", status: "On leave", salary: 172_000 },
    Person { name: "Dennis Ritchie", role: "Principal Engineer", status: "Active", salary: 190_000 },
    Person { name: "Ken Thompson", role: "Principal Engineer", status: "Active", salary: 188_000 },
    Person { name: "Margaret Hamilton", role: "Systems Architect", status: "Inactive", salary: 175_000 },
    Person { name: "Brian Kernighan", role: "Senior Engineer", status: "Active", salary: 160_000 },
    Person { name: "Barbara Liskov", role: "Researcher", status: "Active", salary: 170_000 },
];

pub fn view<'a>(state: &'a State, theme: &AppTheme) -> Element<'a, Message> {
    let t = *theme;
    let rows = sorted_rows(state);

    let name_col = Column::new("Name", move |p: &Person| {
        text(p.name.to_string()).size(13.0).color(t.foreground).into()
    })
    .width(Length::FillPortion(3))
    .sortable("name");

    let role_col = Column::new("Role", move |p: &Person| {
        text(p.role.to_string()).size(13.0).color(t.muted_foreground).into()
    })
    .width(Length::FillPortion(3))
    .sortable("role");

    let status_col = Column::new("Status", move |p: &Person| {
        let variant = match p.status {
            "Active" => BadgeVariant::Success,
            "On leave" => BadgeVariant::Warning,
            _ => BadgeVariant::Secondary,
        };
        badge(&t, p.status.to_string(), variant)
    })
    .width(Length::FillPortion(2))
    .align(Horizontal::Left);

    let salary_col = Column::new("Salary", move |p: &Person| {
        text(format!("${}", fmt_money(p.salary))).size(13.0).color(t.foreground).into()
    })
    .width(Length::FillPortion(2))
    .align(Horizontal::Right)
    .sortable("salary");

    let sort = state.table_sort.map(|(k, d)| (k, match d {
        crate::SortKind::Asc => SortDir::Asc,
        crate::SortKind::Desc => SortDir::Desc,
    }));

    let options = TableOptions {
        sort,
        on_sort: Some(Box::new(Message::TableSort)),
        striped: true,
        row_height: 44.0,
    };

    let tbl = table_with(
        theme,
        &rows,
        vec![name_col, role_col, status_col, salary_col],
        options,
    );

    column![
        section_title(theme, "Sortable table"),
        section_caption(theme, "Click a sortable header to toggle asc/desc ordering."),
        tbl,
        vspace(8.0),
        text(format!("{} people", rows.len())).size(12.0).color(theme.muted_foreground),
    ]
    .spacing(10)
    .into()
}

fn sorted_rows(state: &State) -> Vec<Person> {
    let mut v: Vec<Person> = PEOPLE.to_vec();
    if let Some((key, dir)) = state.table_sort {
        v.sort_by(|a, b| {
            let ord = match key {
                "name" => a.name.cmp(b.name),
                "role" => a.role.cmp(b.role),
                "salary" => a.salary.cmp(&b.salary),
                _ => std::cmp::Ordering::Equal,
            };
            if matches!(dir, crate::SortKind::Desc) { ord.reverse() } else { ord }
        });
    }
    v
}

fn fmt_money(n: i64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.insert(0, ',');
        }
        out.insert(0, ch);
    }
    out
}
