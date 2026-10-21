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
            kind: ChartKind::BarClustered,
            title: Some("Annual Budget by Department".into()),
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            x_title: Some("Department".to_string()),
            y_title: Some("Budget ($K)".to_string()),
            source: None,
            by_rows: false,
            pivot: None,
        };
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["Engineering", "Sales", "Marketing", "Operations", "HR"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "2023".into(),
                    values: vec![
                        Some(450.0), Some(280.0), Some(150.0),
                        Some(320.0), Some(120.0),
                    ],
                    kind: ChartKind::BarClustered,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "2024".into(),
                    values: vec![
                        Some(520.0), Some(310.0), Some(180.0),
                        Some(350.0), Some(140.0),
                    ],
                    kind: ChartKind::BarClustered,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "2025 (Projected)".into(),
                    values: vec![
                        Some(580.0), Some(340.0), Some(200.0),
                        Some(380.0), Some(160.0),
                    ],
                    kind: ChartKind::BarClustered,
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
