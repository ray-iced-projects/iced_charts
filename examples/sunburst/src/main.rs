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
            kind: ChartKind::Sunburst,
            title: Some("Organization Hierarchy".into()),
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
            categories: ["Engineering", "Product", "Marketing", "Sales", "Operations", "Finance"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Sunburst".into(),
                    values: vec![
                        Some(450.0), Some(320.0), Some(180.0),
                        Some(280.0), Some(150.0), Some(120.0),
                    ],
                    kind: ChartKind::Sunburst,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
            ],
            palette: vec![
                series_color(&theme, 1),
                series_color(&theme, 2),
                series_color(&theme, 3),
                series_color(&theme, 4),
                series_color(&theme, 5),
                series_color(&theme, 6),
            ],
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
