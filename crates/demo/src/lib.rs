//! Demo application — app state, messages, navigation and all page views.
//!
//! Both demo-app (native) and demo-web (WASM) depend on this crate and call
//! [`app_builder`] to obtain a configured iced application builder.

pub mod demos;

// Re-export the component library modules so demo files can use `crate::components`
// and `crate::theme` without change.
pub use iced_longbridge::components;
pub use iced_longbridge::theme;

use std::time::Duration;

use chrono::{Datelike, NaiveDate, Timelike};
use iced::{
    Background, Border, Element, Length, Padding, Shadow, Subscription, Task, Theme,
    alignment::{Horizontal, Vertical},
    widget::{button, column, combo_box, container, pane_grid, row, scrollable, text, Space},
};

use crate::{
    components::{button::{button_ex, Variant}, divider, icon::{icon, IconName}, sheet::Side as SheetSide},
    theme::{AppTheme, Appearance, Size},
};

use crate::demos::dock_demo::DockPaneData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Page {
    Home,
    // Basic
    Button,
    Input,
    Checkbox,
    Radio,
    Switch,
    Slider,
    Rating,
    // Display
    Badge,
    Tag,
    Alert,
    Progress,
    Spinner,
    Skeleton,
    Tooltip,
    Avatar,
    Link,
    Kbd,
    Label,
    Icon,
    Divider,
    Markdown,
    Html,
    CodeEditor,
    TextView,
    Notification,
    // Layout (Phase 1)
    Tabs,
    Accordion,
    Collapsible,
    Breadcrumb,
    Pagination,
    Stepper,
    GroupBox,
    TitleBar,
    MenuBar,
    ContextMenu,
    Dialog,
    Resizable,
    Dock,
    Sidebar,
    // Form / menu (Phase 2)
    Select,
    NumberInput,
    OtpInput,
    Calendar,
    DatePicker,
    TimePicker,
    ColorPicker,
    HoverCard,
    Sheet,
    DropdownMenu,
    ButtonGroup,
    ToggleButton,
    Form,
    // Data (Phase 3)
    Table,
    DataTablePage,
    List,
    VirtualList,
    DescriptionList,
    Tree,
    // Charts (Phase 4)
    LineChart,
    BarChart,
    AreaChart,
    PieChart,
    CandlestickChart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Basic,
    Display,
    Layout,
    Form,
    Data,
    Charts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortKind {
    Asc,
    Desc,
}

impl Page {
    pub const ALL: &'static [(Page, &'static str, Category)] = &[
        (Page::Home, "Introduction", Category::Basic),
        // Basic
        (Page::Button, "Button", Category::Basic),
        (Page::Input, "Input", Category::Basic),
        (Page::Checkbox, "Checkbox", Category::Basic),
        (Page::Radio, "Radio", Category::Basic),
        (Page::Switch, "Switch", Category::Basic),
        (Page::Slider, "Slider", Category::Basic),
        (Page::Rating, "Rating", Category::Basic),
        // Display
        (Page::Badge, "Badge", Category::Display),
        (Page::Tag, "Tag", Category::Display),
        (Page::Alert, "Alert", Category::Display),
        (Page::Progress, "Progress", Category::Display),
        (Page::Spinner, "Spinner", Category::Display),
        (Page::Skeleton, "Skeleton", Category::Display),
        (Page::Tooltip, "Tooltip", Category::Display),
        (Page::Avatar, "Avatar", Category::Display),
        (Page::Link, "Link", Category::Display),
        (Page::Kbd, "Kbd", Category::Display),
        (Page::Label, "Label", Category::Display),
        (Page::Icon, "Icon", Category::Display),
        (Page::Divider, "Divider", Category::Display),
        (Page::Markdown, "Markdown", Category::Display),
        (Page::Html, "HTML", Category::Display),
        (Page::CodeEditor, "Code editor", Category::Display),
        (Page::TextView, "Text view", Category::Display),
        (Page::Notification, "Notification", Category::Display),
        // Layout
        (Page::Tabs, "Tabs", Category::Layout),
        (Page::Accordion, "Accordion", Category::Layout),
        (Page::Collapsible, "Collapsible", Category::Layout),
        (Page::Breadcrumb, "Breadcrumb", Category::Layout),
        (Page::Pagination, "Pagination", Category::Layout),
        (Page::Stepper, "Stepper", Category::Layout),
        (Page::GroupBox, "Group box", Category::Layout),
        (Page::TitleBar, "Title bar", Category::Layout),
        (Page::MenuBar, "Menu bar", Category::Layout),
        (Page::ContextMenu, "Context menu", Category::Layout),
        (Page::Dialog, "Dialog", Category::Layout),
        (Page::Resizable, "Resizable", Category::Layout),
        (Page::Dock, "Dock", Category::Layout),
        (Page::Sidebar, "Sidebar", Category::Layout),
        // Form / menu
        (Page::Select, "Select", Category::Form),
        (Page::NumberInput, "Number input", Category::Form),
        (Page::OtpInput, "OTP input", Category::Form),
        (Page::Calendar, "Calendar", Category::Form),
        (Page::DatePicker, "Date picker", Category::Form),
        (Page::TimePicker, "Time picker", Category::Form),
        (Page::ColorPicker, "Color picker", Category::Form),
        (Page::HoverCard, "Hover card", Category::Form),
        (Page::Sheet, "Sheet", Category::Form),
        (Page::DropdownMenu, "Dropdown menu", Category::Form),
        (Page::ButtonGroup, "Button group", Category::Form),
        (Page::ToggleButton, "Toggle button", Category::Form),
        (Page::Form, "Form", Category::Form),
        // Data
        (Page::Table, "Table", Category::Data),
        (Page::DataTablePage, "Data table", Category::Data),
        (Page::List, "List", Category::Data),
        (Page::VirtualList, "Virtual list", Category::Data),
        (Page::DescriptionList, "Description list", Category::Data),
        (Page::Tree, "Tree", Category::Data),
        // Charts
        (Page::LineChart, "Line chart", Category::Charts),
        (Page::BarChart, "Bar chart", Category::Charts),
        (Page::AreaChart, "Area chart", Category::Charts),
        (Page::PieChart, "Pie chart", Category::Charts),
        (Page::CandlestickChart, "Candlestick", Category::Charts),
    ];
}

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation / chrome
    PageSelected(Page),
    ThemeToggle,
    Tick,

    // Component demos (basic)
    ButtonPressed(String),
    InputChanged(String),
    PasswordChanged(String),
    CheckboxToggled(usize, bool),
    RadioSelected(u8),
    SwitchToggled(usize, bool),
    SliderChanged(f32),
    SliderFloatChanged(f32),
    RatingChanged(u8),
    ProgressChanged(f32),
    ProgressReset,
    LinkClicked,
    MarkdownLinkClicked(String),

    // Layout (Phase 1)
    AccordionToggled(usize),
    CollapsibleToggled,
    TabsSelected(usize),
    BreadcrumbPressed(usize),
    PaginationSelected(usize),
    PaginationSmallSelected(usize),
    StepperNext,
    StepperBack,
    DialogOpen(crate::demos::dialog_demo::DialogKind),
    DialogClose,
    ResizableResized(pane_grid::ResizeEvent),
    DockTabSelected(usize, usize),
    DockResized(pane_grid::ResizeEvent),
    SidebarDemoToggle,

    // Form / menu (Phase 2)
    SelectFruit(&'static str),
    ComboboxSelected(&'static str),
    NumberChanged(f64),
    NumberBoundedChanged(f64),
    NumberDecimalChanged(f64),
    OtpChanged(String, Option<usize>),
    OtpShortChanged(String, Option<usize>),
    CalendarSelected(NaiveDate),
    CalendarPrevMonth,
    CalendarNextMonth,
    DatePickerToggle,
    DatePickerSelect(NaiveDate),
    SheetOpen(SheetSide),
    SheetClose,
    DropdownMenuToggle(u8),
    MenuAction(String),
    ButtonGroupSelected(u8),
    ToggleButtonPressed(usize),

    // Data (Phase 3)
    TableSort(&'static str),
    TableResizeStart(usize),
    TableResizeMove(iced::Point),
    TableResizeEnd,
    ListSelected(usize),
    TreeToggle(String),
    TreeSelect(String),

    // New widgets
    ColorPickerToggle,
    ColorPickerChanged(iced::Color),
    ColorPickerHexChanged(String),
    NotificationPush(crate::components::notification::NotificationKind),
    NotificationDismiss(u64),
    TimePickerToggle,
    TimePickerHourChanged(u32),
    TimePickerMinuteChanged(u32),
    TimePickerSecondChanged(u32),
    VirtualListScrolled(iced::widget::scrollable::Viewport),
    VirtualListSelected(usize),
    TextViewAction(iced::widget::text_editor::Action),
    HtmlLinkClicked(String),
    CodeEditorAction(iced::widget::text_editor::Action),
    CodeEditorLanguage(crate::components::code_editor::Language),

    NoOp,
}

pub struct State {
    theme: AppTheme,
    page: Page,

    // Demo state — basic
    pub input_value: String,
    pub password_value: String,
    pub checkboxes: [bool; 3],
    pub radio_value: u8,
    pub switches: [bool; 4],
    pub slider_value: f32,
    pub slider_float: f32,
    pub rating_value: u8,
    pub progress_value: f32,
    pub last_action: String,
    pub tick_rotation: f32,

    // Demo state — layout
    pub accordion_open: [bool; 3],
    pub collapsible_open: bool,
    pub tabs_selected: usize,
    pub pagination_page: usize,
    pub pagination_small: usize,
    pub stepper_current: usize,
    pub dialog_open: Option<crate::demos::dialog_demo::DialogKind>,
    pub resizable_state: pane_grid::State<String>,
    pub dock_state: pane_grid::State<DockPaneData>,
    pub sidebar_demo_collapsed: bool,

    // Demo state — form / menu
    pub select_fruit: Option<&'static str>,
    pub combobox_state: combo_box::State<&'static str>,
    pub combobox_value: Option<&'static str>,
    pub number_value: f64,
    pub number_bounded: f64,
    pub number_decimal: f64,
    pub otp_value: String,
    pub otp_short: String,
    pub calendar_view_year: i32,
    pub calendar_view_month: u32,
    pub calendar_selected: Option<NaiveDate>,
    pub date_picker_open: bool,
    pub sheet_open: Option<SheetSide>,
    pub dropdown_menu_open: Option<u8>,
    pub button_group: u8,
    pub toggles: [bool; 6],

    // Demo state — data
    pub table_sort: Option<(&'static str, SortKind)>,
    pub table_col_widths: Vec<f32>,
    pub table_resize_col: Option<usize>,
    pub table_resize_last_x: Option<f32>,
    pub list_selected: usize,
    pub data_table: crate::components::data_table::DataTable,
    pub tree_nodes: Vec<crate::components::tree::TreeNode>,
    pub tree_expanded: std::collections::HashSet<String>,
    pub tree_selected: Option<String>,

    // Demo state — markdown
    pub markdown_items: Vec<crate::components::markdown::Item>,
    pub markdown_last_link: String,

    // Demo state — new widgets
    pub color_picker_open: bool,
    pub color_value: iced::Color,
    pub color_hex: String,
    pub notifications: crate::components::notification::NotificationList,
    pub next_notification_id: u64,
    pub time_picker_open: bool,
    pub time_value: chrono::NaiveTime,
    pub virtual_list_items: Vec<String>,
    pub virtual_list_offset: f32,
    pub virtual_list_selected: usize,
    pub text_view_content: iced::widget::text_editor::Content,
    pub html_items: Vec<crate::components::markdown::Item>,
    pub code_editor_content: iced::widget::text_editor::Content,
    pub code_editor_language: crate::components::code_editor::Language,
}

impl Default for State {
    fn default() -> Self {
        let resizable_state = pane_grid::State::with_configuration(pane_grid::Configuration::Split {
            axis: pane_grid::Axis::Vertical,
            ratio: 0.35,
            a: Box::new(pane_grid::Configuration::Pane("Navigator".to_string())),
            b: Box::new(pane_grid::Configuration::Split {
                axis: pane_grid::Axis::Horizontal,
                ratio: 0.65,
                a: Box::new(pane_grid::Configuration::Pane("Editor".to_string())),
                b: Box::new(pane_grid::Configuration::Pane("Console".to_string())),
            }),
        });

        let dock_state = pane_grid::State::with_configuration(pane_grid::Configuration::Split {
            axis: pane_grid::Axis::Vertical,
            ratio: 0.5,
            a: Box::new(pane_grid::Configuration::Pane(DockPaneData {
                id: 0,
                tabs: vec!["Explorer".into(), "Search".into(), "Git".into()],
                selected: 0,
            })),
            b: Box::new(pane_grid::Configuration::Pane(DockPaneData {
                id: 1,
                tabs: vec!["README.md".into(), "main.rs".into()],
                selected: 1,
            })),
        });

        let today = chrono::Local::now().date_naive();

        Self {
            theme: AppTheme::light(),
            page: Page::Home,
            input_value: String::new(),
            password_value: String::new(),
            checkboxes: [true, false, true],
            radio_value: 1,
            switches: [true, false, true, false],
            slider_value: 50.0,
            slider_float: 0.5,
            rating_value: 4,
            progress_value: 60.0,
            last_action: String::from("—"),
            tick_rotation: 0.0,
            accordion_open: [true, false, false],
            collapsible_open: false,
            tabs_selected: 0,
            pagination_page: 1,
            pagination_small: 1,
            stepper_current: 1,
            dialog_open: None,
            resizable_state,
            dock_state,
            sidebar_demo_collapsed: false,
            select_fruit: None,
            combobox_state: combo_box::State::new(crate::demos::select_demo::FRUITS.to_vec()),
            combobox_value: None,
            number_value: 3.0,
            number_bounded: 5.0,
            number_decimal: 1.5,
            otp_value: String::new(),
            otp_short: String::new(),
            calendar_view_year: today.year(),
            calendar_view_month: today.month(),
            calendar_selected: None,
            date_picker_open: false,
            sheet_open: None,
            dropdown_menu_open: None,
            button_group: 0,
            toggles: [true, false, false, false, false, false],
            table_sort: None,
            table_col_widths: vec![220.0, 220.0, 140.0, 140.0],
            table_resize_col: None,
            table_resize_last_x: None,
            list_selected: 0,
            data_table: build_sample_data_table(),
            tree_nodes: crate::demos::tree_demo::sample_nodes(),
            tree_expanded: {
                let mut s = std::collections::HashSet::new();
                s.insert("src".to_string());
                s.insert("src/components".to_string());
                s
            },
            tree_selected: None,
            markdown_items: crate::components::markdown::parse(
                crate::demos::markdown_demo::SOURCE,
            )
            .collect(),
            markdown_last_link: String::from("—"),
            color_picker_open: false,
            color_value: iced::Color { r: 0.23, g: 0.51, b: 0.96, a: 1.0 },
            color_hex: String::from("#3B82F6"),
            notifications: crate::components::notification::NotificationList::new(),
            next_notification_id: 1,
            time_picker_open: false,
            time_value: chrono::NaiveTime::from_hms_opt(9, 30, 0).unwrap(),
            virtual_list_items: (0..10_000)
                .map(|i| format!("Row {i:05} — virtualized so only visible rows render"))
                .collect(),
            virtual_list_offset: 0.0,
            virtual_list_selected: 0,
            text_view_content: iced::widget::text_editor::Content::with_text(
                crate::demos::text_view_demo::SAMPLE,
            ),
            html_items: crate::components::html::parse(crate::demos::html_demo::SAMPLE),
            code_editor_content: iced::widget::text_editor::Content::with_text(
                crate::demos::code_editor_demo::SAMPLE_RUST,
            ),
            code_editor_language: crate::components::code_editor::Language::Rust,
        }
    }
}

fn build_sample_data_table() -> crate::components::data_table::DataTable {
    use crate::components::data_table::DataTable;
    DataTable::new(
        vec!["Symbol", "Last", "Change", "Volume"],
        vec![
            vec!["AAPL".into(), "192.45".into(), "+1.23%".into(), "53.1M".into()],
            vec!["MSFT".into(), "421.03".into(), "+0.41%".into(), "22.8M".into()],
            vec!["GOOG".into(), "174.12".into(), "−0.58%".into(), "18.4M".into()],
            vec!["AMZN".into(), "187.89".into(), "+2.04%".into(), "41.9M".into()],
            vec!["NVDA".into(), "126.51".into(), "+3.77%".into(), "201.6M".into()],
        ],
    )
    .widths(vec![
        Length::FillPortion(2),
        Length::FillPortion(2),
        Length::FillPortion(2),
        Length::FillPortion(3),
    ])
    .aligns(vec![
        Horizontal::Left,
        Horizontal::Right,
        Horizontal::Right,
        Horizontal::Right,
    ])
}

pub fn theme(state: &State) -> Theme {
    state.theme.iced_theme()
}

pub fn subscription(_state: &State) -> Subscription<Message> {
    iced::time::every(Duration::from_millis(32)).map(|_| Message::Tick)
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::PageSelected(p) => state.page = p,
        Message::ThemeToggle => {
            state.theme = match state.theme.appearance {
                Appearance::Light => AppTheme::dark(),
                Appearance::Dark => AppTheme::light(),
            };
        }
        Message::Tick => {
            state.tick_rotation =
                (state.tick_rotation + 0.08) % (std::f32::consts::TAU);
            state.notifications.tick(Duration::from_millis(32));
        }
        Message::ButtonPressed(s) => state.last_action = format!("Pressed: {s}"),
        Message::InputChanged(s) => state.input_value = s,
        Message::PasswordChanged(s) => state.password_value = s,
        Message::CheckboxToggled(i, v) => {
            if let Some(b) = state.checkboxes.get_mut(i) {
                *b = v;
            }
        }
        Message::RadioSelected(v) => state.radio_value = v,
        Message::SwitchToggled(i, v) => {
            if let Some(b) = state.switches.get_mut(i) {
                *b = v;
            }
        }
        Message::SliderChanged(v) => state.slider_value = v,
        Message::SliderFloatChanged(v) => state.slider_float = v,
        Message::RatingChanged(v) => state.rating_value = v,
        Message::ProgressChanged(delta) => {
            state.progress_value = (state.progress_value + delta).clamp(0.0, 100.0);
        }
        Message::ProgressReset => state.progress_value = 0.0,
        Message::LinkClicked => state.last_action = "Link clicked".into(),
        Message::MarkdownLinkClicked(url) => state.markdown_last_link = url,
        Message::AccordionToggled(i) => {
            if let Some(b) = state.accordion_open.get_mut(i) {
                *b = !*b;
            }
        }
        Message::CollapsibleToggled => state.collapsible_open = !state.collapsible_open,
        Message::TabsSelected(i) => state.tabs_selected = i,
        Message::BreadcrumbPressed(i) => state.last_action = format!("Crumb: {i}"),
        Message::PaginationSelected(n) => state.pagination_page = n,
        Message::PaginationSmallSelected(n) => state.pagination_small = n,
        Message::StepperNext => state.stepper_current = (state.stepper_current + 1).min(3),
        Message::StepperBack => state.stepper_current = state.stepper_current.saturating_sub(1),
        Message::DialogOpen(k) => state.dialog_open = Some(k),
        Message::DialogClose => state.dialog_open = None,
        Message::ResizableResized(event) => state.resizable_state.resize(event.split, event.ratio),
        Message::DockTabSelected(panel_id, tab_idx) => {
            for (_, data) in state.dock_state.iter_mut() {
                if data.id == panel_id {
                    data.selected = tab_idx;
                }
            }
        }
        Message::DockResized(event) => state.dock_state.resize(event.split, event.ratio),
        Message::SidebarDemoToggle => {
            state.sidebar_demo_collapsed = !state.sidebar_demo_collapsed;
        }
        Message::SelectFruit(v) => {
            state.select_fruit = Some(v);
            state.last_action = format!("Select: {v}");
        }
        Message::ComboboxSelected(v) => {
            state.combobox_value = Some(v);
            state.last_action = format!("Combobox: {v}");
        }
        Message::NumberChanged(v) => state.number_value = v,
        Message::NumberBoundedChanged(v) => state.number_bounded = v,
        Message::NumberDecimalChanged(v) => state.number_decimal = v,
        Message::OtpChanged(v, advance) => {
            state.otp_value = v;
            if let Some(i) = advance {
                return crate::components::otp_input::focus("main", i);
            }
        }
        Message::OtpShortChanged(v, advance) => {
            state.otp_short = v;
            if let Some(i) = advance {
                return crate::components::otp_input::focus("short", i);
            }
        }
        Message::CalendarSelected(d) => {
            state.calendar_selected = Some(d);
            state.calendar_view_year = d.year();
            state.calendar_view_month = d.month();
        }
        Message::CalendarPrevMonth => {
            if state.calendar_view_month == 1 {
                state.calendar_view_month = 12;
                state.calendar_view_year -= 1;
            } else {
                state.calendar_view_month -= 1;
            }
        }
        Message::CalendarNextMonth => {
            if state.calendar_view_month == 12 {
                state.calendar_view_month = 1;
                state.calendar_view_year += 1;
            } else {
                state.calendar_view_month += 1;
            }
        }
        Message::DatePickerToggle => state.date_picker_open = !state.date_picker_open,
        Message::DatePickerSelect(d) => {
            state.calendar_selected = Some(d);
            state.calendar_view_year = d.year();
            state.calendar_view_month = d.month();
            state.date_picker_open = false;
        }
        Message::SheetOpen(side) => state.sheet_open = Some(side),
        Message::SheetClose => state.sheet_open = None,
        Message::DropdownMenuToggle(id) => {
            state.dropdown_menu_open = if state.dropdown_menu_open == Some(id) {
                None
            } else {
                Some(id)
            };
        }
        Message::MenuAction(label) => {
            state.last_action = format!("Menu: {label}");
            state.dropdown_menu_open = None;
        }
        Message::ButtonGroupSelected(i) => state.button_group = i,
        Message::ToggleButtonPressed(i) => {
            if let Some(b) = state.toggles.get_mut(i) {
                *b = !*b;
            }
        }
        Message::TableSort(key) => {
            state.table_sort = match state.table_sort {
                Some((k, SortKind::Asc)) if k == key => Some((key, SortKind::Desc)),
                Some((k, SortKind::Desc)) if k == key => None,
                _ => Some((key, SortKind::Asc)),
            };
        }
        Message::TableResizeStart(i) => {
            state.table_resize_col = Some(i);
            state.table_resize_last_x = None;
        }
        Message::TableResizeMove(pt) => {
            if let Some(i) = state.table_resize_col {
                if let Some(last) = state.table_resize_last_x {
                    let delta = pt.x - last;
                    if let Some(w) = state.table_col_widths.get_mut(i) {
                        *w = (*w + delta).clamp(60.0, 600.0);
                    }
                }
                state.table_resize_last_x = Some(pt.x);
            }
        }
        Message::TableResizeEnd => {
            state.table_resize_col = None;
            state.table_resize_last_x = None;
        }
        Message::ListSelected(i) => {
            state.list_selected = i;
            state.last_action = format!("List: {i}");
        }
        Message::TreeToggle(id) => {
            if !state.tree_expanded.remove(&id) {
                state.tree_expanded.insert(id);
            }
        }
        Message::TreeSelect(id) => state.tree_selected = Some(id),
        Message::ColorPickerToggle => state.color_picker_open = !state.color_picker_open,
        Message::ColorPickerChanged(c) => {
            state.color_value = c;
            state.color_hex = crate::components::color_picker::format_hex(c);
        }
        Message::ColorPickerHexChanged(s) => {
            if let Some(c) = crate::components::color_picker::parse_hex(&s) {
                state.color_value = c;
            }
            state.color_hex = s;
        }
        Message::NotificationPush(kind) => {
            use crate::components::notification::{Notification, NotificationKind};
            let id = state.next_notification_id;
            state.next_notification_id += 1;
            let (title, message) = match kind {
                NotificationKind::Info => ("Heads up", "An informational update."),
                NotificationKind::Success => ("Saved", "Your changes have been saved."),
                NotificationKind::Warning => ("Low disk", "Running low on free space."),
                NotificationKind::Error => ("Failed", "Couldn't reach the server."),
            };
            state
                .notifications
                .push(Notification::new(id, kind, title).message(message));
        }
        Message::NotificationDismiss(id) => state.notifications.dismiss(id),
        Message::TimePickerToggle => state.time_picker_open = !state.time_picker_open,
        Message::TimePickerHourChanged(h) => {
            if let Some(t) = state.time_value.with_hour(h) {
                state.time_value = t;
            }
        }
        Message::TimePickerMinuteChanged(m) => {
            if let Some(t) = state.time_value.with_minute(m) {
                state.time_value = t;
            }
        }
        Message::TimePickerSecondChanged(s) => {
            if let Some(t) = state.time_value.with_second(s) {
                state.time_value = t;
            }
        }
        Message::VirtualListScrolled(viewport) => {
            state.virtual_list_offset = viewport.absolute_offset().y;
        }
        Message::VirtualListSelected(i) => {
            state.virtual_list_selected = i;
            state.last_action = format!("Virtual row: {i}");
        }
        Message::TextViewAction(action) => state.text_view_content.perform(action),
        Message::HtmlLinkClicked(url) => state.last_action = format!("HTML link: {url}"),
        Message::CodeEditorAction(action) => state.code_editor_content.perform(action),
        Message::CodeEditorLanguage(lang) => state.code_editor_language = lang,
        Message::NoOp => {}
    }
    Task::none()
}

pub fn view(state: &State) -> Element<'_, Message> {
    let t = &state.theme;

    let title_bar = container(
        row![
            text("Iced ✦ Longbridge").size(16.0).color(t.foreground),
            Space::new().width(Length::Fill),
            text(state.last_action.clone()).size(12.0).color(t.muted_foreground),
            Space::new().width(Length::Fixed(16.0)),
            theme_toggle(t),
        ]
        .spacing(8)
        .align_y(Vertical::Center),
    )
    .padding(Padding::from([10.0, 16.0]))
    .width(Length::Fill)
    .style({
        let t = *t;
        move |_| container::Style {
            background: Some(Background::Color(t.background)),
            text_color: Some(t.foreground),
            border: Border {
                color: t.border,
                width: 0.0,
                radius: 0.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        }
    });

    let sidebar = build_sidebar(state);
    let content = build_content(state);

    let body = row![sidebar, divider::vertical(t), content]
        .height(Length::Fill)
        .width(Length::Fill);

    column![title_bar, divider::horizontal(t), body]
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}

fn theme_toggle<'a>(t: &AppTheme) -> Element<'a, Message> {
    let tt = *t;
    let glyph = match t.appearance {
        Appearance::Light => IconName::Moon,
        Appearance::Dark => IconName::Sun,
    };
    button(
        row![icon(t, glyph, 16.0), text("Toggle theme").size(13.0).color(t.foreground)]
            .spacing(8)
            .align_y(Vertical::Center),
    )
    .padding(Padding::from([6.0, 12.0]))
    .on_press(Message::ThemeToggle)
    .style(move |_, status| {
        use button::Status::*;
        let bg = match status {
            Hovered => tt.accent,
            Pressed => tt.muted,
            _ => iced::Color::TRANSPARENT,
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: tt.foreground,
            border: Border {
                color: tt.border,
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        }
    })
    .into()
}

fn build_sidebar<'a>(state: &'a State) -> Element<'a, Message> {
    let t = &state.theme;
    let mut nav = column![].spacing(2).padding(Padding::from([12.0, 10.0]));

    let mut current_cat: Option<Category> = None;
    for (page, label, cat) in Page::ALL {
        if Some(*cat) != current_cat {
            current_cat = Some(*cat);
            if current_cat != Some(Category::Basic)
                || matches!(page, Page::Home)
            {
                nav = nav.push(Space::new().height(Length::Fixed(10.0)));
            }
            nav = nav.push(text(category_label(*cat)).size(11.0).color(t.muted_foreground));
            nav = nav.push(Space::new().height(Length::Fixed(2.0)));
        }
        nav = nav.push(sidebar_link(t, *page, label, state.page == *page));
    }

    let tt = *t;
    container(scrollable(nav))
        .width(Length::Fixed(220.0))
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(tt.sidebar)),
            text_color: Some(tt.sidebar_foreground),
            border: Border::default(),
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}

fn category_label(c: Category) -> &'static str {
    match c {
        Category::Basic => "Basic",
        Category::Display => "Display",
        Category::Layout => "Layout",
        Category::Form => "Form",
        Category::Data => "Data",
        Category::Charts => "Charts",
    }
}

fn sidebar_link<'a>(
    t: &AppTheme,
    page: Page,
    label: &'static str,
    active: bool,
) -> Element<'a, Message> {
    let tt = *t;
    button(text(label).size(13.0))
        .width(Length::Fill)
        .padding(Padding::from([6.0, 10.0]))
        .on_press(Message::PageSelected(page))
        .style(move |_, status| {
            use button::Status::*;
            let (bg, fg) = if active {
                (tt.sidebar_accent, tt.foreground)
            } else {
                match status {
                    Hovered => (tt.accent, tt.foreground),
                    Pressed => (tt.muted, tt.foreground),
                    _ => (iced::Color::TRANSPARENT, tt.sidebar_foreground),
                }
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: fg,
                border: Border {
                    color: iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: 6.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            }
        })
        .into()
}

fn build_content<'a>(state: &'a State) -> Element<'a, Message> {
    let t = &state.theme;
    let page_label = Page::ALL
        .iter()
        .find(|(p, _, _)| *p == state.page)
        .map(|(_, l, _)| *l)
        .unwrap_or("");

    let header = column![
        text(page_label).size(28.0).color(t.foreground),
        text(page_description(state.page))
            .size(14.0)
            .color(t.muted_foreground),
    ]
    .spacing(6);

    let body: Element<'_, Message> = match state.page {
        Page::Home => home_page(state),
        Page::Button => demos::button_demo::view(t),
        Page::Input => demos::input_demo::view(state, t),
        Page::Checkbox => demos::checkbox_demo::view(state, t),
        Page::Radio => demos::radio_demo::view(state, t),
        Page::Switch => demos::switch_demo::view(state, t),
        Page::Badge => demos::badge_demo::view(t),
        Page::Tag => demos::tag_demo::view(t),
        Page::Alert => demos::alert_demo::view(t),
        Page::Divider => demos::divider_demo::view(t),
        Page::Markdown => demos::markdown_demo::view(state, t),
        Page::Progress => demos::progress_demo::view(state, t),
        Page::Slider => demos::slider_demo::view(state, t),
        Page::Spinner => demos::spinner_demo::view(state, t),
        Page::Skeleton => demos::skeleton_demo::view(t),
        Page::Tooltip => demos::tooltip_demo::view(t),
        Page::Avatar => demos::avatar_demo::view(t),
        Page::Link => demos::link_demo::view(t),
        Page::Kbd => demos::kbd_demo::view(t),
        Page::Rating => demos::rating_demo::view(state, t),
        Page::Label => demos::label_demo::view(state, t),
        Page::Icon => demos::icon_demo::view(t),
        Page::Tabs => demos::tabs_demo::view(state, t),
        Page::Accordion => demos::accordion_demo::view(state, t),
        Page::Collapsible => demos::collapsible_demo::view(state, t),
        Page::Breadcrumb => demos::breadcrumb_demo::view(t),
        Page::Pagination => demos::pagination_demo::view(state, t),
        Page::Stepper => demos::stepper_demo::view(state, t),
        Page::GroupBox => demos::group_box_demo::view(t),
        Page::TitleBar => demos::title_bar_demo::view(t),
        Page::MenuBar => demos::menu_bar_demo::view(state, t),
        Page::ContextMenu => demos::context_menu_demo::view(state, t),
        Page::Dialog => demos::dialog_demo::view(state, t),
        Page::Resizable => demos::resizable_demo::view(state, t),
        Page::Dock => demos::dock_demo::view(state, t),
        Page::Sidebar => demos::sidebar_demo::view(state, t),
        Page::Select => demos::select_demo::view(state, t),
        Page::NumberInput => demos::number_input_demo::view(state, t),
        Page::OtpInput => demos::otp_input_demo::view(state, t),
        Page::Calendar => demos::calendar_demo::view(state, t),
        Page::DatePicker => demos::date_picker_demo::view(state, t),
        Page::HoverCard => demos::hover_card_demo::view(t),
        Page::Sheet => demos::sheet_demo::view(state, t),
        Page::DropdownMenu => demos::dropdown_menu_demo::view(state, t),
        Page::ButtonGroup => demos::button_group_demo::view(state, t),
        Page::ToggleButton => demos::toggle_button_demo::view(state, t),
        Page::Form => demos::form_demo::view(state, t),
        Page::Table => demos::table_demo::view(state, t),
        Page::DataTablePage => demos::data_table_demo::view(state, t),
        Page::List => demos::list_demo::view(state, t),
        Page::DescriptionList => demos::description_list_demo::view(t),
        Page::Tree => demos::tree_demo::view(state, t),
        Page::LineChart => demos::line_chart_demo::view(t),
        Page::BarChart => demos::bar_chart_demo::view(t),
        Page::AreaChart => demos::area_chart_demo::view(t),
        Page::PieChart => demos::pie_chart_demo::view(t),
        Page::CandlestickChart => demos::candlestick_chart_demo::view(t),
        Page::Html => demos::html_demo::view(state, t),
        Page::CodeEditor => demos::code_editor_demo::view(state, t),
        Page::TextView => demos::text_view_demo::view(state, t),
        Page::Notification => demos::notification_demo::view(state, t),
        Page::TimePicker => demos::time_picker_demo::view(state, t),
        Page::ColorPicker => demos::color_picker_demo::view(state, t),
        Page::VirtualList => demos::virtual_list_demo::view(state, t),
    };

    let tt = *t;
    let content = column![header, Space::new().height(Length::Fixed(16.0)), body]
        .spacing(8)
        .padding(Padding::from([24.0, 32.0]));

    container(scrollable(content))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(tt.background)),
            text_color: Some(tt.foreground),
            border: Border::default(),
            shadow: Shadow::default(),
            snap: true,
        })
        .into()
}

fn page_description(page: Page) -> &'static str {
    match page {
        Page::Home => "A faithful iced 0.14 port of longbridge/gpui-component.",
        Page::Button => "Clickable button with variants, sizes, and states.",
        Page::Input => "Text input with sizes, labels, and password mode.",
        Page::Checkbox => "Boolean selection with label.",
        Page::Radio => "Mutually-exclusive single selection.",
        Page::Switch => "Toggle between on / off states.",
        Page::Badge => "Small inline status indicator.",
        Page::Tag => "Lightweight text label.",
        Page::Alert => "Block-level contextual message.",
        Page::Divider => "Horizontal or vertical separator.",
        Page::Markdown => "Rich text rendering for Markdown / GFM content.",
        Page::Progress => "Linear progress indicator.",
        Page::Slider => "Continuous value selector.",
        Page::Spinner => "Animated loading indicator.",
        Page::Skeleton => "Placeholder for loading content.",
        Page::Tooltip => "Contextual popover shown on hover.",
        Page::Avatar => "Circular user identity.",
        Page::Link => "Inline anchor-style link.",
        Page::Kbd => "Keyboard shortcut display.",
        Page::Rating => "Star-based rating input.",
        Page::Label => "Form field label with optional required marker.",
        Page::Icon => "Icon set using Unicode glyphs.",
        Page::Tabs => "Tabbed interface with switched content panels.",
        Page::Accordion => "Stack of expandable sections.",
        Page::Collapsible => "Single-section expand / collapse.",
        Page::Breadcrumb => "Hierarchical navigation trail.",
        Page::Pagination => "Numbered page controls with truncation.",
        Page::Stepper => "Step-by-step progress indicator.",
        Page::GroupBox => "Titled, bordered container.",
        Page::TitleBar => "App chrome with title and window controls.",
        Page::MenuBar => "Horizontal strip of labeled menus — desktop app chrome.",
        Page::ContextMenu => "Right-click anywhere in the child to open a menu at the cursor.",
        Page::Dialog => "Modal dialog with variants and actions.",
        Page::Resizable => "Pane grid with draggable splitters.",
        Page::Dock => "Tabbed panels arranged in a pane grid.",
        Page::Sidebar => "Composable navigation sidebar.",
        Page::Select => "Pick-list and filterable combobox.",
        Page::NumberInput => "Numeric input with inc/dec steppers.",
        Page::OtpInput => "Segmented input for one-time codes.",
        Page::Calendar => "Month grid for picking a date.",
        Page::DatePicker => "Button trigger with anchored calendar.",
        Page::HoverCard => "Rich hover-triggered popover.",
        Page::Sheet => "Slide-in side panel over a backdrop.",
        Page::DropdownMenu => "Button with an anchored menu of actions.",
        Page::ButtonGroup => "Segmented buttons sharing an outer border.",
        Page::ToggleButton => "Stateful button that persists a pressed look.",
        Page::Form => "label / input / help-text layout helper.",
        Page::Table => "Column-and-row layout with typed cells and sorting.",
        Page::DataTablePage => "Wrapper over table for simple string-only data.",
        Page::List => "Selectable rows with optional leading / trailing slots.",
        Page::DescriptionList => "Aligned term / definition pairs.",
        Page::Tree => "Recursive nested list with expand / collapse.",
        Page::LineChart => "Multi-series linear plot with axis and legend.",
        Page::BarChart => "Grouped bars on a band x-scale.",
        Page::AreaChart => "Filled region below each line series.",
        Page::PieChart => "Pie and donut variants with a legend.",
        Page::CandlestickChart => "OHLC bodies, wicks, and a volume strip for trading data.",
        Page::Html => "Lightweight HTML subset rendered through the Markdown pipeline.",
        Page::CodeEditor => "Monospaced text editor with syntect-powered syntax highlighting.",
        Page::TextView => "Read-only selectable text view.",
        Page::Notification => "Toast stack with info / success / warning / error variants.",
        Page::TimePicker => "Hour / minute / second selector with a popover trigger.",
        Page::ColorPicker => "Palette, HSLA sliders, and hex input in an anchored panel.",
        Page::VirtualList => "Viewport-virtualised scrolling list for huge datasets.",
    }
}

fn home_page<'a>(state: &'a State) -> Element<'a, Message> {
    let t = &state.theme;
    column![
        text("Welcome!").size(22.0).color(t.foreground),
        text("This app showcases a set of iced 0.14 components modeled after the gpui-component library from longbridge.").size(14.0).color(t.muted_foreground),
        Space::new().height(Length::Fixed(12.0)),
        row![
            button_ex(t, "Explore buttons", Variant::Primary, Size::Md, Some(Message::PageSelected(Page::Button)), false, false),
            button_ex(t, "View layout", Variant::Secondary, Size::Md, Some(Message::PageSelected(Page::Tabs)), false, false),
        ].spacing(8),
        Space::new().height(Length::Fixed(24.0)),
        container(
            column![
                text("Theme").size(14.0).color(t.foreground),
                text("Click the button in the top right to toggle between light and dark.").size(13.0).color(t.muted_foreground),
            ].spacing(6)
        )
        .padding(Padding::from([14.0, 16.0]))
        .style({
            let tt = *t;
            move |_| container::Style {
                background: Some(Background::Color(tt.muted)),
                text_color: Some(tt.foreground),
                border: Border {
                    color: tt.border,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            }
        })
        .max_width(540),
    ]
    .spacing(8)
    .align_x(Horizontal::Left)
    .into()
}
