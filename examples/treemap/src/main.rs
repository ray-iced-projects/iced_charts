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
            kind: ChartKind::Treemap,
            title: Some("Storage Disk Usage".into()),
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            x_title: None,
            y_title: None,
            source: None,
            by_rows: false,
            pivot: None,
        };
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["Documents", "Images", "Videos", "Audio", "Other", "System"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Treemap".into(),
                    values: vec![
                        Some(125.0), Some(450.0), Some(850.0),
                        Some(200.0), Some(180.0), Some(300.0),
                    ],
                    kind: ChartKind::Treemap,
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
