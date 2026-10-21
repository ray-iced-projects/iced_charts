use iced::widget::container;
use iced::{Theme, Widget};
use iced_charts::{State, Chart, ChartData, ChartKind, LegendPos, SeriesData, Trendline, TrendlineKind, TrendlineData};
use iced_charts::resolve::series_color;
use iced_charts::model::style::Theme as ChartTheme;


pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .theme(Theme::TokyoNightLight)
        .run()
}

struct Example {
    grid: State,
}

impl Default for Example {
    fn default() -> Self {
        let chart = Chart {
            id: 1,
            kind: ChartKind::Scatter,
            title: Some("Website Traffic with Moving Average Trend".into()),
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            x_title: Some("Days".to_string()),
            y_title: Some("Visitors".to_string()),
            source: None,
            by_rows: false,
            pivot: None,
        };
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: vec![],
            series: vec![
                SeriesData {
                    name: "Daily Visitors".into(),
                    x: Some(vec![
                        Some(1.0),  Some(2.0),  Some(3.0),  Some(4.0),  Some(5.0),
                        Some(6.0),  Some(7.0),  Some(8.0),  Some(9.0),  Some(10.0),
                        Some(11.0), Some(12.0), Some(13.0), Some(14.0), Some(15.0),
                        Some(16.0), Some(17.0), Some(18.0), Some(19.0), Some(20.0),
                        Some(21.0), Some(22.0), Some(23.0), Some(24.0), Some(25.0),
                    ]),
                    values: vec![
                        Some(245.0), Some(312.0), Some(198.0), Some(421.0), Some(356.0),
                        Some(289.0), Some(445.0), Some(378.0), Some(512.0), Some(423.0),
                        Some(489.0), Some(534.0), Some(478.0), Some(612.0), Some(545.0),
                        Some(598.0), Some(623.0), Some(687.0), Some(634.0), Some(712.0),
                        Some(745.0), Some(698.0), Some(823.0), Some(756.0), Some(834.0),
                    ],
                    kind: ChartKind::Scatter,
                    color: series_color(&theme, 0),
                    trendlines: vec![
                        TrendlineData {
                            spec: Trendline {
                                kind: TrendlineKind::Linear,
                                name: Some("Linear Trend".into()),
                                forward: 0.0,
                                backward: 0.0,
                                intercept: None,
                                display_equation: true,
                                display_r_squared: true,
                                color: None,
                                width: Some(2.5),
                                dash: None,
                            },
                            color: [0xFF, 0x80, 0x00, 0xFF],  // Orange for the trendline
                        }
                    ],
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
