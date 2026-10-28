
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

        let mut title = ChartText::new(TextKind::Title);
        title.content = "Company Growth Metrics".into();
        
        let mut x_title = ChartText::new(TextKind::XTitle);
        x_title.content = "Years in Operation".into();

        let mut y_title = ChartText::new(TextKind::YTitle);
        y_title.content = "Annual Revenue ($M)".into();

        let chart = 
            Chart::new(
                1,
                ChartKind::Bubble,
            )
            .title(title)
            .x_title(x_title)
            .y_title(y_title);

        let theme = Theme::TokyoNight;

        let data = ChartData {
            categories: vec![],
            series: vec![
                SeriesData {
                    name: "Tech Startups".into(),
                    x: Some(vec![
                        Some(1.0), Some(2.0), Some(3.0),
                        Some(4.0), Some(5.0), Some(6.0),
                    ]),
                    values: vec![
                        Some(2.5), Some(5.2), Some(8.1),
                        Some(12.5), Some(18.3), Some(25.7),
                    ],
                    kind: ChartKind::Bubble,
                    color: series_color(&theme, 0),
                    sizes: Some(vec![
                        Some(1.5), Some(2.1), Some(2.8),
                        Some(3.5), Some(4.2), Some(5.1),
                    ]),
                    ..Default::default()
                },
                SeriesData {
                    name: "Finance Companies".into(),
                    x: Some(vec![
                        Some(2.0), Some(3.5), Some(5.0),
                        Some(6.5), Some(8.0), Some(9.5),
                    ]),
                    values: vec![
                        Some(8.0), Some(12.5), Some(18.2),
                        Some(25.5), Some(32.1), Some(40.8),
                    ],
                    kind: ChartKind::Bubble,
                    color: series_color(&theme, 1),
                    sizes: Some(vec![
                        Some(2.2), Some(2.8), Some(3.5),
                        Some(4.1), Some(4.8), Some(5.5),
                    ]),
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
