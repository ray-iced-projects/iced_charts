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
        title.content = "Annual Budget by Department".into();
        
        let mut x_title = ChartText::new(TextKind::XTitle);
        x_title.content = "Department".into();

        let mut y_title = ChartText::new(TextKind::YTitle);
        y_title.content = "Budget ($K)".into();

        let chart = 
            Chart::new(
                1,
                ChartKind::BarClustered,
            )
            .title(title)
            .x_title(x_title)
            .y_title(y_title);

        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["Engineering", "Sales", "Marketing", "Operations", "HR"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "2023".into(),
                    values: vec![
                        Some(450.0), Some(280.0), Some(150.0),
                        Some(320.0), Some(120.0),
                    ],
                    kind: ChartKind::BarClustered,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "2024".into(),
                    values: vec![
                        Some(520.0), Some(310.0), Some(180.0),
                        Some(350.0), Some(140.0),
                    ],
                    kind: ChartKind::BarClustered,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "2025 (Projected)".into(),
                    values: vec![
                        Some(580.0), Some(340.0), Some(200.0),
                        Some(380.0), Some(160.0),
                    ],
                    kind: ChartKind::BarClustered,
                    color: series_color(&theme, 2),
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
