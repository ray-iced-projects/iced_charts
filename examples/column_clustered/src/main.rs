
use iced::widget::{PickList, container, row};
use iced::{Theme, Widget};
use iced_charts::model::features::{ChartText, TextKind};
use iced_charts::{ChartState, Chart, ChartData, ChartKind, LegendPos, SeriesData};
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
        title.content = "Quarterly Revenue by Region".into();
        
        let mut x_title = ChartText::new(TextKind::XTitle);
        x_title.content = "Quarter".into();

        let mut y_title = ChartText::new(TextKind::YTitle);
        y_title.content = "Revenue ($M)".into();

        let chart = 
            Chart::new(
                1,
                ChartKind::ColumnClustered,
            )
            .title(title)
            .x_title(x_title)
            .y_title(y_title)
            .legend(LegendPos::Bottom);

        let theme = Theme::TokyoNight;

        let data = ChartData {
            categories: ["Q1", "Q2", "Q3", "Q4"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "North".into(),
                    values: vec![
                        Some(45.0), Some(52.0), Some(48.0),
                        Some(61.0),
                    ],
                    color: chart.series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "South".into(),
                    values: vec![
                        Some(38.0), Some(42.0), Some(55.0),
                        Some(49.0),
                    ],
                    color: chart.series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "East".into(),
                    values: vec![
                        Some(52.0), Some(58.0), Some(62.0),
                        Some(71.0),
                    ],
                    color: chart.series_color(&theme, 2),
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
