
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
            kind: ChartKind::ColumnStacked100,
            title: Some("Market Share Distribution".into()),
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            anchor: Default::default(),
            x_title: Some("Year".to_string()),
            y_title: Some("Percentage (%)".to_string()),
            source: None,
            by_rows: false,
            pivot: None,
        };
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["2020", "2021", "2022", "2023", "2024"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Company A".into(),
                    values: vec![
                        Some(35.0), Some(32.0), Some(28.0),
                        Some(25.0), Some(22.0),
                    ],
                    kind: ChartKind::ColumnStacked100,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "Company B".into(),
                    values: vec![
                        Some(28.0), Some(30.0), Some(32.0),
                        Some(35.0), Some(38.0),
                    ],
                    kind: ChartKind::ColumnStacked100,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "Company C".into(),
                    values: vec![
                        Some(20.0), Some(22.0), Some(25.0),
                        Some(28.0), Some(30.0),
                    ],
                    kind: ChartKind::ColumnStacked100,
                    color: series_color(&theme, 2),
                    ..Default::default()
                },
                SeriesData {
                    name: "Others".into(),
                    values: vec![
                        Some(17.0), Some(16.0), Some(15.0),
                        Some(12.0), Some(10.0),
                    ],
                    kind: ChartKind::ColumnStacked100,
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
