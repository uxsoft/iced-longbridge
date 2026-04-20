use iced::{Element, widget::column};

use crate::{
    Message,
    components::chart::{candlestick_chart, Candle},
    demos::common::{section_caption, section_title},
    theme::AppTheme,
};

pub fn view<'a>(theme: &AppTheme) -> Element<'a, Message> {
    // Synthetic OHLC walk — enough variety to exercise bodies, wicks, volume.
    let seed: &[(f32, f32, f32, f32, f32)] = &[
        (100.0, 104.2, 98.7, 103.1, 1.2),
        (103.1, 106.4, 101.9, 102.5, 1.5),
        (102.5, 105.0, 100.1, 104.8, 1.9),
        (104.8, 108.3, 103.7, 107.9, 2.3),
        (107.9, 110.2, 106.1, 106.6, 1.7),
        (106.6, 107.4, 101.0, 101.8, 2.4),
        (101.8, 104.9, 100.5, 104.3, 1.8),
        (104.3, 109.5, 103.9, 108.7, 2.6),
        (108.7, 112.0, 107.5, 111.4, 2.9),
        (111.4, 113.8, 109.2, 110.0, 2.1),
        (110.0, 111.2, 106.8, 107.1, 1.6),
        (107.1, 109.6, 105.4, 109.0, 1.8),
        (109.0, 114.3, 108.8, 113.7, 2.7),
        (113.7, 116.9, 112.3, 115.5, 2.4),
        (115.5, 117.8, 113.0, 113.5, 1.9),
    ];

    let candles: Vec<Candle> = seed
        .iter()
        .map(|(o, h, l, c, v)| Candle::new(*o, *h, *l, *c, *v))
        .collect();

    let labels = (0..seed.len())
        .map(|i| {
            if i % 3 == 0 {
                format!("D{}", i + 1)
            } else {
                String::new()
            }
        })
        .collect();

    column![
        section_title(theme, "Candlestick chart"),
        section_caption(
            theme,
            "OHLC bodies with high/low wicks; bullish (close ≥ open) use success green, bearish use danger red. The strip below shows volume.",
        ),
        candlestick_chart::<Message>(theme, candles, labels),
    ]
    .spacing(10)
    .into()
}
