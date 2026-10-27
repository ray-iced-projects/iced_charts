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
        title.content = "Sales and Profit Analysis".into();
        
        let mut x_title = ChartText::new(TextKind::XTitle);
        x_title.content = "Quarter".into();

        let mut y_title = ChartText::new(TextKind::YTitle);
        y_title.content = "Amount ($K)".into();

        let chart = 
            Chart::new(
                1,
                ChartKind::Combo,
            )
            .title(title)
            .x_title(x_title)
            .y_title(y_title);

        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["Q1", "Q2", "Q3", "Q4"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Revenue".into(),
                    values: vec![
                        Some(450.0), Some(520.0), Some(480.0),
                        Some(600.0),
                    ],
                    kind: ChartKind::ColumnClustered,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "Profit Margin %".into(),
                    values: vec![
                        Some(18.5), Some(22.0), Some(20.5),
                        Some(25.0),
                    ],
                    kind: ChartKind::Line,
                    color: series_color(&theme, 1),
                    secondary: true,
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
