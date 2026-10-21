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
            kind: ChartKind::BarStacked100,
            title: Some("Customer Satisfaction by Region".into()),
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            x_title: Some("Region".to_string()),
            y_title: Some("Percentage (%)".to_string()),
            source: None,
            by_rows: false,
            pivot: None,
        };
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["North America", "Europe", "Asia", "South America", "Australia"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Very Satisfied".into(),
                    values: vec![
                        Some(45.0), Some(52.0), Some(38.0),
                        Some(35.0), Some(48.0),
                    ],
                    kind: ChartKind::BarStacked100,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "Satisfied".into(),
                    values: vec![
                        Some(35.0), Some(30.0), Some(40.0),
                        Some(42.0), Some(38.0),
                    ],
                    kind: ChartKind::BarStacked100,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "Neutral".into(),
                    values: vec![
                        Some(15.0), Some(12.0), Some(15.0),
                        Some(18.0), Some(10.0),
                    ],
                    kind: ChartKind::BarStacked100,
                    color: series_color(&theme, 2),
                    ..Default::default()
                },
                SeriesData {
                    name: "Dissatisfied".into(),
                    values: vec![
                        Some(5.0), Some(6.0), Some(7.0),
                        Some(5.0), Some(4.0),
                    ],
                    kind: ChartKind::BarStacked100,
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
