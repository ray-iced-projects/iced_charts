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
            kind: ChartKind::Radar,
            title: Some("Product Feature Comparison".into()),
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            anchor: Default::default(),
            x_title: None,
            y_title: None,
            source: None,
            by_rows: false,
            pivot: None,
        };
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["Performance", "Reliability", "Security", "Usability", "Cost", "Support"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Product A".into(),
                    values: vec![
                        Some(8.5), Some(9.0), Some(8.2),
                        Some(7.8), Some(8.5), Some(9.2),
                    ],
                    kind: ChartKind::Radar,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "Product B".into(),
                    values: vec![
                        Some(7.2), Some(8.1), Some(9.0),
                        Some(8.8), Some(7.5), Some(7.8),
                    ],
                    kind: ChartKind::Radar,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "Product C".into(),
                    values: vec![
                        Some(8.8), Some(8.5), Some(7.9),
                        Some(8.2), Some(9.2), Some(8.5),
                    ],
                    kind: ChartKind::Radar,
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
