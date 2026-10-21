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
            kind: ChartKind::Histogram,
            title: Some("Response Time Distribution".into()),
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            x_title: Some("Response Time (ms)".to_string()),
            y_title: Some("Frequency".to_string()),
            source: None,
            by_rows: false,
            pivot: None,
        };
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["0-100ms", "100-200ms", "200-300ms", "300-400ms", "400-500ms", "500ms+"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Web Server".into(),
                    values: vec![
                        Some(45.0), Some(38.0), Some(22.0),
                        Some(12.0), Some(5.0), Some(2.0),
                    ],
                    kind: ChartKind::Histogram,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "API Gateway".into(),
                    values: vec![
                        Some(35.0), Some(32.0), Some(18.0),
                        Some(10.0), Some(4.0), Some(1.0),
                    ],
                    kind: ChartKind::Histogram,
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
