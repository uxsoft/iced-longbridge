fn main() -> iced::Result {
    iced::application(demo::State::default, demo::update, demo::view)
        .title(title)
        .theme(demo::theme)
        .subscription(demo::subscription)
        .window_size((1180.0, 760.0))
        .run()
}

fn title(_: &demo::State) -> String {
    String::from("Iced-Longbridge Component Showcase")
}
