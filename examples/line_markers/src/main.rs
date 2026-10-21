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
            kind: ChartKind::LineMarkers,
            title: Some("Stock Price Trends".into()),
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            anchor: Default::default(),
            x_title: Some("Week".to_string()),
            y_title: Some("Price ($)".to_string()),
            source: None,
            by_rows: false,
            pivot: None,
        };
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["Week 1", "Week 2", "Week 3", "Week 4", "Week 5", "Week 6", "Week 7", "Week 8"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Tech Stock".into(),
                    values: vec![
                        Some(145.50), Some(148.20), Some(143.80),
                        Some(151.30), Some(155.90), Some(153.40),
                        Some(158.70), Some(162.50),
                    ],
                    kind: ChartKind::LineMarkers,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "Finance Stock".into(),
                    values: vec![
                        Some(89.20), Some(91.50), Some(93.10),
                        Some(90.80), Some(95.30), Some(98.50),
                        Some(101.20), Some(103.80),
                    ],
                    kind: ChartKind::LineMarkers,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "Healthcare Stock".into(),
                    values: vec![
                        Some(210.40), Some(212.80), Some(215.30),
                        Some(218.90), Some(220.50), Some(219.70),
                        Some(222.10), Some(225.60),
                    ],
                    kind: ChartKind::LineMarkers,
                    color: series_color(&theme, 2),
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
