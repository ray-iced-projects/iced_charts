use iced::widget::{PickList, container, row};
use iced::{Theme, Widget};
use iced_charts::model::features::{ChartText, TextKind};
use iced_charts::{ChartState, Chart, ChartData, ChartKind, SeriesData, Trendline, TrendlineKind, TrendlineData};
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
        title.content = "Website Traffic with Moving Average Trend".into();

        let mut x_title = ChartText::new(TextKind::XTitle);
        x_title.content = "Days".into();
        
        let mut y_title = ChartText::new(TextKind::YTitle);
        y_title.content = "Visitors".into();
        
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
                    name: "Daily Visitors".into(),
                    x: Some(vec![
                        Some(1.0),  Some(2.0),  Some(3.0),  Some(4.0),  Some(5.0),
                        Some(6.0),  Some(7.0),  Some(8.0),  Some(9.0),  Some(10.0),
                        Some(11.0), Some(12.0), Some(13.0), Some(14.0), Some(15.0),
                        Some(16.0), Some(17.0), Some(18.0), Some(19.0), Some(20.0),
                        Some(21.0), Some(22.0), Some(23.0), Some(24.0), Some(25.0),
                    ]),
                    values: vec![
                        Some(245.0), Some(312.0), Some(198.0), Some(421.0), Some(356.0),
                        Some(289.0), Some(445.0), Some(378.0), Some(512.0), Some(423.0),
                        Some(489.0), Some(534.0), Some(478.0), Some(612.0), Some(545.0),
                        Some(598.0), Some(623.0), Some(687.0), Some(634.0), Some(712.0),
                        Some(745.0), Some(698.0), Some(823.0), Some(756.0), Some(834.0),
                    ],
                    color: chart.series_color(&theme, 0),
                    trendlines: vec![
                        TrendlineData {
                            spec: Trendline {
                                kind: TrendlineKind::MovingAverage(3),
                                name: Some("3-Day Moving Average".into()),
                                forward: 0.0,
                                backward: 0.0,
                                intercept: None,
                                display_equation: false,
                                display_r_squared: false,
                                color: None,
                                width: Some(2.5),
                                dash: None,
                                label_size: 12.0,
                            },
                            color: iced::Color::from_rgba8(0xFF, 0x80, 0x00, 1.0),  // Orange for the trendline
                        }
                    ],
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
