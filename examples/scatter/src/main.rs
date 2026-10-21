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
            kind: ChartKind::Scatter,
            title: Some("Four Quadrant Analysis - Centered at Origin (0, 0)".into()),
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            x_title: Some("X Axis (-10 to 10)".to_string()),
            y_title: Some("Y Axis (-10 to 10)".to_string()),
            source: None,
            by_rows: false,
            pivot: None,
        };
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: vec![],
            series: vec![
                // Quadrant I (Top Right): X > 0, Y > 0
                SeriesData {
                    name: "Quadrant I (+, +)".into(),
                    x: Some(vec![Some(5.0), Some(3.0), Some(7.0), Some(5.0), Some(6.0), Some(4.0)]),
                    values: vec![Some(5.0), Some(7.0), Some(2.0), Some(7.0), Some(4.0), Some(8.0)],
                    kind: ChartKind::Scatter,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                // Quadrant II (Top Left): X < 0, Y > 0
                SeriesData {
                    name: "Quadrant II (-, +)".into(),
                    x: Some(vec![Some(-5.0), Some(-4.0), Some(-7.0), Some(-5.0), Some(-4.0), Some(-7.0)]),
                    values: vec![Some(5.0), Some(3.0), Some(6.0), Some(4.0), Some(7.0), Some(5.0)],
                    kind: ChartKind::Scatter,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                // Quadrant III (Bottom Left): X < 0, Y < 0
                SeriesData {
                    name: "Quadrant III (-, -)".into(),
                    x: Some(vec![Some(-5.0), Some(-3.0), Some(-6.0), Some(-5.0), Some(-3.0), Some(-6.0)]),
                    values: vec![Some(-5.0), Some(-6.0), Some(-2.0), Some(-4.0), Some(-7.0), Some(-3.0)],
                    kind: ChartKind::Scatter,
                    color: series_color(&theme, 2),
                    ..Default::default()
                },
                // Quadrant IV (Bottom Right): X > 0, Y < 0
                SeriesData {
                    name: "Quadrant IV (+, -)".into(),
                    x: Some(vec![Some(5.0), Some(4.0), Some(6.0), Some(5.0), Some(4.0), Some(6.0)]),
                    values: vec![Some(-5.0), Some(-4.0), Some(-7.0), Some(-6.0), Some(-3.0), Some(-5.0)],
                    kind: ChartKind::Scatter,
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
