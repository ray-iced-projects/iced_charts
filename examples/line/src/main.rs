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
            kind: ChartKind::Line,
            title: Some("Monthly Sales".into()),
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            anchor: Default::default(),
            x_title: Some("Months".to_string()),
            y_title: Some("Millions".to_string()),
            source: None,
            by_rows: false,
            pivot: None,
        };
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["Jan", "Feb", "Mar", "Apr", "May", "Jun"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Sales".into(),
                    values: vec![
                        Some(12.0), Some(28.0), Some(19.0),
                        Some(35.0), Some(22.0), Some(41.0),
                    ],
                    kind: ChartKind::Line,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "Target".into(),
                    values: vec![
                        Some(20.0), Some(20.0), Some(25.0),
                        Some(25.0), Some(30.0), Some(30.0),
                    ],
                    kind: ChartKind::Line,
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
