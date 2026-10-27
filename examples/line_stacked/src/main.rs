use iced::widget::{PickList, container, row};
use iced::{Theme, Widget};
use iced_charts::model::features::{ChartText, TextKind};
use iced_charts::{State, Chart, ChartData, ChartKind, SeriesData};
use iced_charts::resolve::series_color;
use iced_charts::model::style::ChartTheme;
use iced_charts::model::series_colors::{theme_strings, str_to_theme};


pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .theme(|ex: &Example| ex.theme())
        .run()
}

struct Example {
    grid: State,
    selected_theme_str: String,
    selected_theme: Theme,
}

impl Default for Example {
    fn default() -> Self {

        let mut title = ChartText::new(TextKind::Title);
        title.content = "Server Resource Utilization".into();

        let mut x_title = ChartText::new(TextKind::XTitle);
        x_title.content = "Hour".into();
        
        let mut y_title = ChartText::new(TextKind::YTitle);
        y_title.content = "Memory Usage (GB)".into();
        
        let chart = 
            Chart::new(
                1,
                ChartKind::LineStacked,
            )
            .title(title)
            .x_title(x_title)
            .y_title(y_title);
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["00:00", "04:00", "08:00", "12:00", "16:00", "20:00", "24:00"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Database".into(),
                    values: vec![
                        Some(2.5), Some(2.1), Some(3.2),
                        Some(4.1), Some(3.8), Some(3.5),
                        Some(2.8),
                    ],
                    kind: ChartKind::LineStacked,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "Cache".into(),
                    values: vec![
                        Some(1.2), Some(1.5), Some(2.1),
                        Some(3.2), Some(2.9), Some(2.1),
                        Some(1.8),
                    ],
                    kind: ChartKind::LineStacked,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "Application".into(),
                    values: vec![
                        Some(0.8), Some(0.9), Some(1.2),
                        Some(1.8), Some(1.5), Some(1.2),
                        Some(0.9),
                    ],
                    kind: ChartKind::LineStacked,
                    color: series_color(&theme, 2),
                    ..Default::default()
                },
                SeriesData {
                    name: "System".into(),
                    values: vec![
                        Some(0.5), Some(0.5), Some(0.5),
                        Some(0.6), Some(0.6), Some(0.5),
                        Some(0.5),
                    ],
                    kind: ChartKind::LineStacked,
                    color: series_color(&theme, 3),
                    ..Default::default()
                },
            ],
            palette: vec![],
        };
        Example { 
            grid: State::new(chart, data),
            selected_theme_str: "TokyoNight".to_string(),
            selected_theme: Theme::TokyoNight,
         }
    }
}

#[derive(Debug, Clone)]
enum Message {
    OnThemeSelect(String),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::OnThemeSelect(theme_str) => {
                self.selected_theme_str = theme_str.clone();
                self.selected_theme = str_to_theme(&theme_str);
                self.grid.request_redraw();
            }
        }
    }

    fn view(&self) -> impl Widget<Message> {

        let pl = PickList::new(
                Some(self.selected_theme_str.clone()),
                theme_strings(), 
                |s: &String| s.clone(),
            )
            .width(220.0)
            .on_select(Message::OnThemeSelect)
            .boxed();
        
        row(vec![
            pl,
            container(self.grid.view().map(|_| unreachable!()))
                .padding(20).boxed()
        ]).padding(20.0).boxed()
    }

    fn theme(&self) -> Theme {
        self.selected_theme.clone()
    }
}
