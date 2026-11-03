use iced::widget::{PickList, container, row};
use iced::{Theme, Widget};
use iced_charts::model::features::{ChartText, TextKind};
use iced_charts::model::theme_helpers::{str_to_theme, theme_strings};
use iced_charts::{Chart, ChartData, ChartKind, ChartMessage, ChartState, LegendPos, SeriesData};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .theme(|ex: &Example| ex.theme())
        .window_size((1200.0, 450.0))
        .centered()
        .run()
}

struct MultiChart {
    charts: Vec<ChartState>,
}

impl MultiChart {
    fn new(charts: Vec<ChartState>) -> Self {
        Self { charts }
    }

    fn redraw_all(&mut self) {
        for chart in &mut self.charts {
            chart.request_redraw();
        }
    }

    fn apply(&mut self, index: usize, msg: ChartMessage) {
        if let Some(chart) = self.charts.get_mut(index) {
            chart.apply(msg);
        }
    }

}

struct Example {
    charts: MultiChart,
    selected_theme_str: String,
    selected_theme: Theme,
}

impl Default for Example {
    fn default() -> Self {
        let left_chart = make_line_chart(1, "Revenue", "Month", "USD");
        let right_chart = make_line_chart(2, "Customers", "Month", "Count");

        Example {
            charts: MultiChart::new(vec![
                ChartState::new(left_chart.0, left_chart.1),
                ChartState::new(right_chart.0, right_chart.1),
            ]),
            selected_theme_str: "TokyoNight".to_string(),
            selected_theme: Theme::TokyoNight,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    OnThemeSelect(String),
    Chart { index: usize, msg: ChartMessage },
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::OnThemeSelect(theme_str) => {
                self.selected_theme_str = theme_str.clone();
                self.selected_theme = str_to_theme(&theme_str);
                self.charts.redraw_all();
            }
            Message::Chart { index, msg } => {
                self.charts.apply(index, msg);
            }
        }
    }

    fn view(&self) -> impl Widget<Message> {
        let pl = PickList::new(
            Some(self.selected_theme_str.clone()),
            theme_strings(),
            |s: &String| s.clone(),
        )
        .width(180.0)
        .on_select(Message::OnThemeSelect)
        .boxed();

        let mut children = vec![pl];
        for (index, chart) in self.charts.charts.iter().enumerate() {
            children.push(
                container(chart.view().map(move |msg| Message::Chart { index, msg }))
                    .padding(20)
                    .boxed(),
            );
        }

        row(children)
            .padding(20.0)
            .boxed()
    }

    fn theme(&self) -> Theme {
        self.selected_theme.clone()
    }
}

fn make_line_chart(id: usize, title_text: &str, x_text: &str, y_text: &str) -> (Chart, ChartData) {
    let mut title = ChartText::new(TextKind::Title);
    title.content = title_text.to_string();

    let mut x_title = ChartText::new(TextKind::XTitle);
    x_title.content = x_text.to_string();

    let mut y_title = ChartText::new(TextKind::YTitle);
    y_title.content = y_text.to_string();

    let chart = Chart::new(id, ChartKind::Line)
        .title(title)
        .x_title(x_title)
        .y_title(y_title)
        .legend(LegendPos::Bottom)
        .width(480.0)
        .height(320.0);

    let theme = Theme::TokyoNight;

    let data = ChartData {
        categories: ["Jan", "Feb", "Mar", "Apr", "May", "Jun"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        series: vec![
            SeriesData {
                name: "Actual".into(),
                values: vec![
                    Some(12.0), Some(18.0), Some(17.0), Some(23.0), Some(28.0), Some(31.0),
                ],
                color: chart.series_color(&theme, 0),
                ..Default::default()
            },
            SeriesData {
                name: "Target".into(),
                values: vec![
                    Some(10.0), Some(14.0), Some(20.0), Some(22.0), Some(26.0), Some(30.0),
                ],
                color: chart.series_color(&theme, 1),
                ..Default::default()
            },
        ],
        palette: vec![],
    };

    (chart, data)
}
