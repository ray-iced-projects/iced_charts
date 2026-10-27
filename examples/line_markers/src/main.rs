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
        title.content = "Stock Price Trends".into();

        let mut x_title = ChartText::new(TextKind::XTitle);
        x_title.content = "Week".into();
        
        let mut y_title = ChartText::new(TextKind::YTitle);
        y_title.content = "Price ($)".into();
        
        let chart = 
            Chart::new(
                1,
                ChartKind::LineMarkers,
            )
            .title(title)
            .x_title(x_title)
            .y_title(y_title);
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: ["Week 1", "Week 2", "Week 3", "Week 4", "Week 5", "Week 6", "Week 7", "Week 8"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            series: vec![
                SeriesData {
                    name: "Tech Stock".into(),
                    values: vec![
                        Some(145.50), Some(148.20), Some(143.80),
                        Some(151.30), Some(155.90), Some(153.40),
                        Some(158.70), Some(162.50),
                    ],
                    kind: ChartKind::LineMarkers,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "Finance Stock".into(),
                    values: vec![
                        Some(89.20), Some(91.50), Some(93.10),
                        Some(90.80), Some(95.30), Some(98.50),
                        Some(101.20), Some(103.80),
                    ],
                    kind: ChartKind::LineMarkers,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "Healthcare Stock".into(),
                    values: vec![
                        Some(210.40), Some(212.80), Some(215.30),
                        Some(218.90), Some(220.50), Some(219.70),
                        Some(222.10), Some(225.60),
                    ],
                    kind: ChartKind::LineMarkers,
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
