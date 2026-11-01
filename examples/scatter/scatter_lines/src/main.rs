
use iced::widget::{PickList, container, row};
use iced::{Theme, Widget};
use iced_charts::model::features::{ChartText, TextKind};
use iced_charts::model::theme_helpers::{str_to_theme, theme_strings};
use iced_charts::{ChartState, Chart, ChartData, ChartKind, SeriesData};


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
        title.content = "Revenue vs Marketing Spend Correlation".into();

        let mut x_title = ChartText::new(TextKind::XTitle);
        x_title.content = "Marketing Spend ($K)".into();
        
        let mut y_title = ChartText::new(TextKind::YTitle);
        y_title.content = "Revenue ($M)".into();
        
        let chart = 
            Chart::new(
                1,
                ChartKind::Scatter,
            )
            .title(title)
            .x_title(x_title)
            .y_title(y_title);

        let theme = Theme::TokyoNight;

        let data = ChartData {
            categories: vec![],
            series: vec![
                SeriesData {
                    name: "Q1-Q4 2023".into(),
                    x: Some(vec![
                        Some(15.0), Some(25.0), Some(35.0),
                        Some(45.0), Some(55.0), Some(65.0),
                        Some(75.0), Some(85.0),
                    ]),
                    values: vec![
                        Some(25.5), Some(35.2), Some(48.1),
                        Some(52.3), Some(61.5), Some(68.8),
                        Some(75.2), Some(82.1),
                    ],
                    color: chart.series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "Q1-Q4 2024".into(),
                    x: Some(vec![
                        Some(18.0), Some(28.0), Some(38.0),
                        Some(48.0), Some(58.0), Some(68.0),
                        Some(78.0), Some(88.0),
                    ]),
                    values: vec![
                        Some(28.2), Some(38.5), Some(51.3),
                        Some(56.8), Some(65.2), Some(72.5),
                        Some(79.8), Some(87.5),
                    ],
                    color: chart.series_color(&theme, 1),
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
            Message::OnThemeSelect(str) => {
                self.selected_theme = str_to_theme(&str);
                self.selected_theme_str = str;
                self.grid.request_redraw();
            }
        }
    }

    fn theme(&self) -> Theme {
        self.selected_theme.clone()
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
    
}
