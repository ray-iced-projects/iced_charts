use iced::widget::{container, row, PickList};
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
        title.content = "Storage Disk Usage".into();

        let chart = Chart::new(1, ChartKind::Treemap)
            .title(title);
        
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["Documents", "Images", "Videos", "Audio", "Other", "System"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Treemap".into(),
                    values: vec![
                        Some(125.0), Some(450.0), Some(850.0),
                        Some(200.0), Some(180.0), Some(300.0),
                    ],
                    kind: ChartKind::Treemap,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
            ],
            palette: vec![
                series_color(&theme, 1),
                series_color(&theme, 2),
                series_color(&theme, 3),
                series_color(&theme, 4),
                series_color(&theme, 5),
                series_color(&theme, 6),
            ],
        };
        let selected_theme = Theme::TokyoNight;
        Example { 
            grid: ChartState::new(chart, data),
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
