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
            kind: ChartKind::Area,
            title: Some("Carbon Emissions Over Time".into()),
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            anchor: Default::default(),
            x_title: Some("Year".to_string()),
            y_title: Some("Emissions (Mt CO2)".to_string()),
            source: None,
            by_rows: false,
            pivot: None,
        };
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["2018", "2019", "2020", "2021", "2022", "2023"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Manufacturing".into(),
                    values: vec![
                        Some(45.2), Some(48.1), Some(52.3),
                        Some(55.8), Some(58.5), Some(61.2),
                    ],
                    kind: ChartKind::Area,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "Transportation".into(),
                    values: vec![
                        Some(32.5), Some(35.2), Some(38.1),
                        Some(40.5), Some(42.8), Some(45.3),
                    ],
                    kind: ChartKind::Area,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "Energy".into(),
                    values: vec![
                        Some(28.7), Some(31.4), Some(33.6),
                        Some(36.2), Some(39.1), Some(42.5),
                    ],
                    kind: ChartKind::Area,
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
