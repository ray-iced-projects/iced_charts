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
        title.content = "Carbon Emissions Over Time".into();
        
        let mut x_title = ChartText::new(TextKind::XTitle);
        x_title.content = "Year".into();

        let mut y_title = ChartText::new(TextKind::YTitle);
        y_title.content = "Emissions (Mt CO2)".into();

        let chart = 
            Chart::new(
                1,
                ChartKind::Area,
            )
            .title(title)
            .x_title(x_title)
            .y_title(y_title);
            
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["2018", "2019", "2020", "2021", "2022", "2023"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Manufacturing".into(),
                    values: vec![
                        Some(45.2), Some(48.1), Some(52.3),
                        Some(55.8), Some(58.5), Some(61.2),
                    ],
                    kind: ChartKind::Area,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "Transportation".into(),
                    values: vec![
                        Some(32.5), Some(35.2), Some(38.1),
                        Some(40.5), Some(42.8), Some(45.3),
                    ],
                    kind: ChartKind::Area,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "Energy".into(),
                    values: vec![
                        Some(28.7), Some(31.4), Some(33.6),
                        Some(36.2), Some(39.1), Some(42.5),
                    ],
                    kind: ChartKind::Area,
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
