//! Chart widgets — canvas-based plots for numeric and categorical data.
//!
//! All charts share a small plotting core:
//! - [`scale`] maps data values to pixel coordinates.
//! - [`axis`] draws x/y axis, ticks, labels and gridlines.
//!
//! Each chart type is an `iced::canvas::Program` and can be obtained as an
//! `Element` via the small `*_chart(theme, data)` helpers.

pub mod area;
pub mod axis;
pub mod bar;
pub mod candlestick;
pub mod interactive;
pub mod line;
pub mod pie;
pub mod scale;

#[allow(unused_imports)]
pub use area::{area_chart, AreaChart};
#[allow(unused_imports)]
pub use bar::{bar_chart, BarChart};
#[allow(unused_imports)]
pub use candlestick::{candlestick_chart, Candle, CandlestickChart};
#[allow(unused_imports)]
pub use interactive::{ChartHover, ChartHoverEntry, ChartState, ChartWidget};
#[allow(unused_imports)]
pub use line::{line_chart, LineChart, Series};
#[allow(unused_imports)]
pub use pie::{pie_chart, PieChart, PieSlice};
