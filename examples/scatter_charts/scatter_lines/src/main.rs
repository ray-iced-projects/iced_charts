use iced::widget::container;
use iced::{Theme, Widget};
use iced_charts::{State, Chart, ChartData, ChartKind, LegendPos, SeriesData};
use iced_charts::resolve::series_color;
use iced_charts::model::style::Theme as ChartTheme;


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
            kind: ChartKind::ScatterLines,
            title: Some("Revenue vs Marketing Spend Correlation".into()),
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            x_title: Some("Marketing Spend ($K)".to_string()),
            y_title: Some("Revenue ($M)".to_string()),
            source: None,
            by_rows: false,
            pivot: None,
        };
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: vec![],
            series: vec![
                SeriesData {
                    name: "Q1-Q4 2023".into(),
                    x: Some(vec![
                        Some(15.0), Some(25.0), Some(35.0),
                        Some(45.0), Some(55.0), Some(65.0),
                        Some(75.0), Some(85.0),
                    ]),
                    values: vec![
                        Some(25.5), Some(35.2), Some(48.1),
                        Some(52.3), Some(61.5), Some(68.8),
                        Some(75.2), Some(82.1),
                    ],
                    kind: ChartKind::ScatterLines,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "Q1-Q4 2024".into(),
                    x: Some(vec![
                        Some(18.0), Some(28.0), Some(38.0),
                        Some(48.0), Some(58.0), Some(68.0),
                        Some(78.0), Some(88.0),
                    ]),
                    values: vec![
                        Some(28.2), Some(38.5), Some(51.3),
                        Some(56.8), Some(65.2), Some(72.5),
                        Some(79.8), Some(87.5),
                    ],
                    kind: ChartKind::ScatterLines,
                    color: series_color(&theme, 1),
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
