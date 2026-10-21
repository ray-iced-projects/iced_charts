use iced::widget::container;
use iced::{Theme, Widget};
use iced_charts::{State, Chart, ChartData, ChartKind, LegendPos, SeriesData, series_color, model::Theme as ChartTheme};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .theme(Theme::TokyoNight)
        .run()
}

struct Example {
    grid: State,
}

impl Default for Example {
    fn default() -> Self {
        let chart = Chart {
            id: 1,
            kind: ChartKind::Stock,
            title: Some("Stock Price Candlesticks".into()),
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            anchor: Default::default(),
            x_title: Some("Date".to_string()),
            y_title: Some("Price ($)".to_string()),
            source: None,
            by_rows: false,
            pivot: None,
        };
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["Day 1", "Day 2", "Day 3", "Day 4", "Day 5"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Open".into(),
                    values: vec![
                        Some(150.0), Some(152.0), Some(149.0),
                        Some(154.0), Some(158.0),
                    ],
                    kind: ChartKind::Stock,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "High".into(),
                    values: vec![
                        Some(156.0), Some(158.0), Some(155.0),
                        Some(162.0), Some(166.0),
                    ],
                    kind: ChartKind::Stock,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "Low".into(),
                    values: vec![
                        Some(148.0), Some(150.0), Some(147.0),
                        Some(151.0), Some(156.0),
                    ],
                    kind: ChartKind::Stock,
                    color: series_color(&theme, 2),
                    ..Default::default()
                },
                SeriesData {
                    name: "Close".into(),
                    values: vec![
                        Some(155.0), Some(151.0), Some(152.0),
                        Some(159.0), Some(162.0),
                    ],
                    kind: ChartKind::Stock,
                    color: series_color(&theme, 3),
                    ..Default::default()
                },
            ],
            palette: vec![],
        };
        Example { grid: State::new(chart, data) }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Example {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> impl Widget<Message> {
        container(self.grid.view().map(|_| unreachable!()))
            .padding(20)
    }
}

