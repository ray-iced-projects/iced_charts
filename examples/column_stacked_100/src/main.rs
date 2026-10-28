
use iced::widget::{PickList, container, row};
use iced::{Theme, Widget};
use iced_charts::model::features::{ChartText, TextKind};
use iced_charts::{ChartState, Chart, ChartData, ChartKind, SeriesData};
use iced_charts::resolve::series_color;
use iced_charts::model::style::ChartTheme;
use iced_charts::model::series_colors::{theme_strings, str_to_theme};


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
        title.content = "Market Share Distribution".into();
        
        let mut x_title = ChartText::new(TextKind::XTitle);
        x_title.content = "Year".into();

        let mut y_title = ChartText::new(TextKind::YTitle);
        y_title.content = "Percentage (%)".into();

        let chart = 
            Chart::new(
                1,
                ChartKind::ColumnStacked100,
            )
            .title(title)
            .x_title(x_title)
            .y_title(y_title);

        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["2020", "2021", "2022", "2023", "2024"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Company A".into(),
                    values: vec![
                        Some(35.0), Some(32.0), Some(28.0),
                        Some(25.0), Some(22.0),
                    ],
                    kind: ChartKind::ColumnStacked100,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "Company B".into(),
                    values: vec![
                        Some(28.0), Some(30.0), Some(32.0),
                        Some(35.0), Some(38.0),
                    ],
                    kind: ChartKind::ColumnStacked100,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "Company C".into(),
                    values: vec![
                        Some(20.0), Some(22.0), Some(25.0),
                        Some(28.0), Some(30.0),
                    ],
                    kind: ChartKind::ColumnStacked100,
                    color: series_color(&theme, 2),
                    ..Default::default()
                },
                SeriesData {
                    name: "Others".into(),
                    values: vec![
                        Some(17.0), Some(16.0), Some(15.0),
                        Some(12.0), Some(10.0),
                    ],
                    kind: ChartKind::ColumnStacked100,
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
