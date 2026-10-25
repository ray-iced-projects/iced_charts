use std::f32::consts::PI;

use iced::widget::{PickList, container, row};
use iced::{Theme, Widget};
use iced_charts::model::features::{ChartText, TextKind};
use iced_charts::{Chart, ChartData, ChartKind, SeriesData, State, Trendline, TrendlineData, TrendlineKind};
use iced_charts::resolve::series_color;
use iced_charts::model::style::Theme as ChartTheme;
use iced_charts::model::series_colors::{theme_strings, str_to_theme};

const TITLE_SIZE: f32 = 16.0;
const _LABEL_SIZE: f32 = 10.0;
const AXIS_TITLE_SIZE: f32 = 12.0;


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
        title.content = "Website Traffic with Moving Average Trend".into();
        title.size = TITLE_SIZE;

        let mut x_title = ChartText::new(TextKind::XAxis);
        x_title.content = "Days".into();
        x_title.size = AXIS_TITLE_SIZE;
        
        let mut y_title = ChartText::new(TextKind::YAxis);
        y_title.content = "Dailey Visitors".into();
        y_title.rotation = PI/2.0;
        y_title.size = AXIS_TITLE_SIZE;
        
        let selected_theme = Theme::TokyoNight;
        let bkg = selected_theme.palette().background;
        title.color = bkg.strongest.text;
        x_title.color = bkg.strong.text;
        y_title.color = bkg.strong.text;
        
        let chart = 
            Chart::new(
                1,
                ChartKind::Scatter,
            )
            .title(title)
            .x_title(x_title)
            .y_title(y_title);

        let theme = ChartTheme::default();

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
                    kind: ChartKind::Scatter,
                    color: series_color(&theme, 0),
                    trendlines: vec![
                        TrendlineData {
                            spec: Trendline {
                                kind: TrendlineKind::Linear,
                                name: Some("Linear Trend".into()),
                                forward: 0.0,
                                backward: 0.0,
                                intercept: None,
                                display_equation: true,
                                display_r_squared: true,
                                color: None,
                                width: Some(2.5),
                                dash: None,
                            },
                            color: [0xFF, 0x80, 0x00, 0xFF],  // Orange for the trendline
                        }
                    ],
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
            Message::OnThemeSelect(str) => {
                self.selected_theme = str_to_theme(&str);
                self.selected_theme_str = str;
                self.grid.request_redraw();
            }
        }
    }

    fn theme(&self) -> Theme {
        self.selected_theme.clone()
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
}
