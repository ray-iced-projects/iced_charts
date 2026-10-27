
use iced::widget::{container, row, PickList};
use iced::{Theme, Widget};
use iced_charts::model::features::{ChartText, TextKind};
use iced_charts::{State, Chart, ChartData, ChartKind, LegendPos, SeriesData};
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
        title.content = "Test Score Distribution".into();
        
        let mut x_title = ChartText::new(TextKind::XTitle);
        x_title.content = "Class".into();

        let mut y_title = ChartText::new(TextKind::YTitle);
        y_title.content = "Score".into();

        let chart = Chart::new(1, ChartKind::BoxWhisker)
            .title(title)
            .legend(LegendPos::Bottom)
            .x_title(x_title)
            .y_title(y_title);
        
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["Class A", "Class B", "Class C", "Class D"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Math Scores".into(),
                    values: vec![
                        Some(65.0), Some(72.0), Some(78.0),
                        Some(68.0),
                    ],
                    kind: ChartKind::BoxWhisker,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "English Scores".into(),
                    values: vec![
                        Some(72.0), Some(75.0), Some(82.0),
                        Some(76.0),
                    ],
                    kind: ChartKind::BoxWhisker,
                    color: series_color(&theme, 1),
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
