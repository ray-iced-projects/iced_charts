use iced::widget::{PickList, container, row};
use iced::{Theme, Widget};
use iced_charts::model::features::{ChartText, TextKind};
use iced_charts::{ChartState, Chart, ChartData, ChartKind, SeriesData};
use iced_charts::resolve::series_color;
use iced_charts::model::theme_helpers::{theme_strings, str_to_theme};


pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .theme(|ex: &Example| ex.theme())
        .run()
}

struct Example {
    grid: ChartState,
    selected_theme_str: String,
    selected_theme: Theme,
}

impl Default for Example {
    fn default() -> Self {

        let theme = Theme::TokyoNight;

        let mut title = ChartText::new(TextKind::Title);
        title.content = "Cloud Infrastructure Costs".into();
        
        let mut x_title = ChartText::new(TextKind::XTitle);
        x_title.content = "Month".into();

        let mut y_title = ChartText::new(TextKind::YTitle);
        y_title.content = "Cost ($1000s)".into();

        let chart = 
            Chart::new(
                1,
                ChartKind::AreaStacked,
            )
            .title(title)
            .x_title(x_title)
            .y_title(y_title);

        let data = ChartData {
            categories: ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Compute".into(),
                    values: vec![
                        Some(12.5), Some(13.2), Some(14.1),
                        Some(15.3), Some(16.5), Some(17.2),
                        Some(18.1),
                    ],
                    kind: ChartKind::AreaStacked,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "Storage".into(),
                    values: vec![
                        Some(8.2), Some(8.8), Some(9.5),
                        Some(10.2), Some(11.1), Some(12.0),
                        Some(12.8),
                    ],
                    kind: ChartKind::AreaStacked,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "Networking".into(),
                    values: vec![
                        Some(4.1), Some(4.3), Some(4.6),
                        Some(5.1), Some(5.5), Some(5.8),
                        Some(6.2),
                    ],
                    kind: ChartKind::AreaStacked,
                    color: series_color(&theme, 2),
                    ..Default::default()
                },
                SeriesData {
                    name: "Support".into(),
                    values: vec![
                        Some(2.0), Some(2.1), Some(2.2),
                        Some(2.3), Some(2.4), Some(2.5),
                        Some(2.6),
                    ],
                    kind: ChartKind::AreaStacked,
                    color: series_color(&theme, 3),
                    ..Default::default()
                },
            ],
            palette: vec![],
        };
        Example { 
            grid: ChartState::new(chart, data),
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
