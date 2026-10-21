
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
            kind: ChartKind::BoxWhisker,
            title: Some("Test Score Distribution".into()),
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            x_title: Some("Class".to_string()),
            y_title: Some("Score".to_string()),
            source: None,
            by_rows: false,
            pivot: None,
        };
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["Class A", "Class B", "Class C", "Class D"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Math Scores".into(),
                    values: vec![
                        Some(65.0), Some(72.0), Some(78.0),
                        Some(68.0),
                    ],
                    kind: ChartKind::BoxWhisker,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "English Scores".into(),
                    values: vec![
                        Some(72.0), Some(75.0), Some(82.0),
                        Some(76.0),
                    ],
                    kind: ChartKind::BoxWhisker,
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
