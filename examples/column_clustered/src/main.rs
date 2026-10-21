
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
            kind: ChartKind::ColumnClustered,
            title: Some("Quarterly Revenue by Region".into()),
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            x_title: Some("Quarter".to_string()),
            y_title: Some("Revenue ($M)".to_string()),
            source: None,
            by_rows: false,
            pivot: None,
        };
        let theme = ChartTheme::default();
        let data = ChartData {
            categories: ["Q1", "Q2", "Q3", "Q4"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "North".into(),
                    values: vec![
                        Some(45.0), Some(52.0), Some(48.0),
                        Some(61.0),
                    ],
                    kind: ChartKind::ColumnClustered,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "South".into(),
                    values: vec![
                        Some(38.0), Some(42.0), Some(55.0),
                        Some(49.0),
                    ],
                    kind: ChartKind::ColumnClustered,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "East".into(),
                    values: vec![
                        Some(52.0), Some(58.0), Some(62.0),
                        Some(71.0),
                    ],
                    kind: ChartKind::ColumnClustered,
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
