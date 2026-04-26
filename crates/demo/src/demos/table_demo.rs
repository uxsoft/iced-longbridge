use iced::{
    Element, Length,
    alignment::Horizontal,
    widget::{column, text},
};

use crate::{
    Message, SortDir, State,
    components::{
        badge::{badge, BadgeVariant},
        menu::{menu, Item},
        table::{table, Column},
    },
    demos::common::{section_caption, section_title, vspace},
    lucide,
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

    let w = |i: usize| {
        state
            .table_col_widths
            .get(i)
            .copied()
            .map(Length::Fixed)
            .unwrap_or(Length::Fill)
    };

    let name_col = Column::text("Name", |p: &Person| p.name.to_string())
        .width(w(0))
        .sortable("name");

    let role_col = Column::text("Role", |p: &Person| p.role.to_string())
        .width(w(1))
        .sortable("role");

    let status_col = Column::new("Status", move |p: &Person| {
        let variant = match p.status {
            "Active" => BadgeVariant::Success,
            "On leave" => BadgeVariant::Warning,
            _ => BadgeVariant::Secondary,
        };
        badge(&t, p.status.to_string(), variant)
    })
    .width(w(2))
    .align(Horizontal::Left);

    let mut salary_col = Column::text("Salary", |p: &Person| format!("${}", fmt_money(p.salary)))
        .width(w(3))
        .align(Horizontal::Right)
        .sortable("salary")
        .header_button(lucide::settings(), Message::TableHeaderButtonToggle(3));

    if state.table_header_menu_open == Some(3) {
        let panel = menu(
            theme,
            vec![
                Item::new("Hide column", Message::TableHeaderMenuAction("hide"))
                    .icon(lucide::eye_off()),
                Item::new("Filter…", Message::TableHeaderMenuAction("filter"))
                    .icon(lucide::filter()),
                Item::Separator,
                Item::new("Reset sort", Message::TableHeaderMenuAction("reset_sort"))
                    .icon(lucide::refresh_cw()),
            ],
        );
        salary_col = salary_col.header_panel(panel);
    }

    let tbl: Element<Message> = table(
        theme,
        &rows,
        vec![name_col, role_col, status_col, salary_col],
    )
    .row_height(44.0)
    .sort(state.table_sort, Message::TableSort)
    .resize(
        state.table_resize_col,
        Message::TableResizeStart,
        Message::TableResizeMove,
        Message::TableResizeEnd,
    )
    .into();

    column![
        section_title(theme, "Sortable table"),
        section_caption(
            theme,
            "Click a sortable header to toggle ordering. Use the settings button on the Salary header to open a per-column menu. Drag a divider to resize.",
        ),
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
            if matches!(dir, SortDir::Desc) { ord.reverse() } else { ord }
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
