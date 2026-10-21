
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
            kind: ChartKind::Bubble,
            title: Some("Company Growth Metrics".into()),
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            anchor: Default::default(),
            x_title: Some("Years in Operation".to_string()),
            y_title: Some("Annual Revenue ($M)".to_string()),
            source: None,
            by_rows: false,
            pivot: None,
        };
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: vec![],
            series: vec![
                SeriesData {
                    name: "Tech Startups".into(),
                    x: Some(vec![
                        Some(1.0), Some(2.0), Some(3.0),
                        Some(4.0), Some(5.0), Some(6.0),
                    ]),
                    values: vec![
                        Some(2.5), Some(5.2), Some(8.1),
                        Some(12.5), Some(18.3), Some(25.7),
                    ],
                    kind: ChartKind::Bubble,
                    color: series_color(&theme, 0),
                    sizes: Some(vec![
                        Some(1.5), Some(2.1), Some(2.8),
                        Some(3.5), Some(4.2), Some(5.1),
                    ]),
                    ..Default::default()
                },
                SeriesData {
                    name: "Finance Companies".into(),
                    x: Some(vec![
                        Some(2.0), Some(3.5), Some(5.0),
                        Some(6.5), Some(8.0), Some(9.5),
                    ]),
                    values: vec![
                        Some(8.0), Some(12.5), Some(18.2),
                        Some(25.5), Some(32.1), Some(40.8),
                    ],
                    kind: ChartKind::Bubble,
                    color: series_color(&theme, 1),
                    sizes: Some(vec![
                        Some(2.2), Some(2.8), Some(3.5),
                        Some(4.1), Some(4.8), Some(5.5),
                    ]),
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
