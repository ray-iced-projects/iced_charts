use iced::widget::{container, row, PickList};
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
        title.content = "Stock Price Candlesticks".into();
        
        let mut x_title = ChartText::new(TextKind::XTitle);
        x_title.content = "Date".into();

        let mut y_title = ChartText::new(TextKind::YTitle);
        y_title.content = "Price ($)".into();

        let chart = Chart::new(1, ChartKind::Stock)
            .title(title)
            .x_title(x_title)
            .y_title(y_title);
        
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["Day 1", "Day 2", "Day 3", "Day 4", "Day 5"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Open".into(),
                    values: vec![
                        Some(150.0), Some(152.0), Some(149.0),
                        Some(154.0), Some(158.0),
                    ],
                    kind: ChartKind::Stock,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "High".into(),
                    values: vec![
                        Some(156.0), Some(158.0), Some(155.0),
                        Some(162.0), Some(166.0),
                    ],
                    kind: ChartKind::Stock,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "Low".into(),
                    values: vec![
                        Some(148.0), Some(150.0), Some(147.0),
                        Some(151.0), Some(156.0),
                    ],
                    kind: ChartKind::Stock,
                    color: series_color(&theme, 2),
                    ..Default::default()
                },
                SeriesData {
                    name: "Close".into(),
                    values: vec![
                        Some(155.0), Some(151.0), Some(152.0),
                        Some(159.0), Some(162.0),
                    ],
                    kind: ChartKind::Stock,
                    color: series_color(&theme, 3),
                    ..Default::default()
                },
            ],
            palette: vec![],
        };
        let selected_theme = Theme::TokyoNight;
        Example { 
            grid: State::new(chart, data),
            selected_theme_str: "TokyoNight".to_string(),
            selected_theme,
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
                self.selected_theme_str = theme_str;
                self.selected_theme = str_to_theme(&self.selected_theme_str);
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
            .on_select(Message::OnThemeSelect);
        
        row![
            pl,
            container(self.grid.view().map(|_| unreachable!()))
                .padding(20)
        ].padding(20.0)
    }

    fn theme(&self) -> Theme {
        self.selected_theme.clone()
    }
}

