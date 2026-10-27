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
        title.content = "Product Feature Comparison".into();

        let chart = 
            Chart::new(
                1,
                ChartKind::Radar,
            )
            .title(title)
            .label_size(12.0);

        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["Performance", "Reliability", "Security", "Usability", "Cost", "Support"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Product A".into(),
                    values: vec![
                        Some(8.5), Some(9.0), Some(8.2),
                        Some(7.8), Some(8.5), Some(9.2),
                    ],
                    kind: ChartKind::Radar,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "Product B".into(),
                    values: vec![
                        Some(7.2), Some(8.1), Some(9.0),
                        Some(8.8), Some(7.5), Some(7.8),
                    ],
                    kind: ChartKind::Radar,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "Product C".into(),
                    values: vec![
                        Some(8.8), Some(8.5), Some(7.9),
                        Some(8.2), Some(9.2), Some(8.5),
                    ],
                    kind: ChartKind::Radar,
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
