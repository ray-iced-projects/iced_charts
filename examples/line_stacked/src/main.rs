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
            kind: ChartKind::LineStacked,
            title: Some("Server Resource Utilization".into()),
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            x_title: Some("Hour".to_string()),
            y_title: Some("Memory Usage (GB)".to_string()),
            source: None,
            by_rows: false,
            pivot: None,
        };
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["00:00", "04:00", "08:00", "12:00", "16:00", "20:00", "24:00"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Database".into(),
                    values: vec![
                        Some(2.5), Some(2.1), Some(3.2),
                        Some(4.1), Some(3.8), Some(3.5),
                        Some(2.8),
                    ],
                    kind: ChartKind::LineStacked,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "Cache".into(),
                    values: vec![
                        Some(1.2), Some(1.5), Some(2.1),
                        Some(3.2), Some(2.9), Some(2.1),
                        Some(1.8),
                    ],
                    kind: ChartKind::LineStacked,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "Application".into(),
                    values: vec![
                        Some(0.8), Some(0.9), Some(1.2),
                        Some(1.8), Some(1.5), Some(1.2),
                        Some(0.9),
                    ],
                    kind: ChartKind::LineStacked,
                    color: series_color(&theme, 2),
                    ..Default::default()
                },
                SeriesData {
                    name: "System".into(),
                    values: vec![
                        Some(0.5), Some(0.5), Some(0.5),
                        Some(0.6), Some(0.6), Some(0.5),
                        Some(0.5),
                    ],
                    kind: ChartKind::LineStacked,
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
