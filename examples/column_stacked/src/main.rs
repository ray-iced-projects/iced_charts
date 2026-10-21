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
            kind: ChartKind::ColumnStacked,
            title: Some("Product Sales by Category".into()),
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            anchor: Default::default(),
            x_title: Some("Month".to_string()),
            y_title: Some("Units Sold".to_string()),
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
                    name: "Electronics".into(),
                    values: vec![
                        Some(100.0), Some(120.0), Some(150.0),
                        Some(130.0), Some(160.0), Some(180.0),
                    ],
                    kind: ChartKind::ColumnStacked,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "Clothing".into(),
                    values: vec![
                        Some(80.0), Some(95.0), Some(110.0),
                        Some(105.0), Some(120.0), Some(140.0),
                    ],
                    kind: ChartKind::ColumnStacked,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "Home & Garden".into(),
                    values: vec![
                        Some(60.0), Some(75.0), Some(85.0),
                        Some(95.0), Some(110.0), Some(125.0),
                    ],
                    kind: ChartKind::ColumnStacked,
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
