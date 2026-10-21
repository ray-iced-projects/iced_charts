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
            kind: ChartKind::BarStacked,
            title: Some("Website Traffic Sources".into()),
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            anchor: Default::default(),
            x_title: Some("Month".to_string()),
            y_title: Some("Sessions".to_string()),
            source: None,
            by_rows: false,
            pivot: None,
        };
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["January", "February", "March", "April", "May", "June"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Organic".into(),
                    values: vec![
                        Some(3200.0), Some(3500.0), Some(4100.0),
                        Some(4800.0), Some(5200.0), Some(5600.0),
                    ],
                    kind: ChartKind::BarStacked,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "Paid".into(),
                    values: vec![
                        Some(1200.0), Some(1400.0), Some(1600.0),
                        Some(1800.0), Some(2000.0), Some(2200.0),
                    ],
                    kind: ChartKind::BarStacked,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "Social".into(),
                    values: vec![
                        Some(800.0), Some(950.0), Some(1100.0),
                        Some(1250.0), Some(1400.0), Some(1550.0),
                    ],
                    kind: ChartKind::BarStacked,
                    color: series_color(&theme, 2),
                    ..Default::default()
                },
                SeriesData {
                    name: "Referral".into(),
                    values: vec![
                        Some(400.0), Some(450.0), Some(500.0),
                        Some(550.0), Some(600.0), Some(650.0),
                    ],
                    kind: ChartKind::BarStacked,
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
