use iced::widget::{PickList, container, row};
use iced::{Theme, Widget};
use iced_charts::model::features::{ChartText, TextKind};
use iced_charts::{Chart, ChartData, ChartKind, ChartState, LegendPos, SeriesData};
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
        title.content = "Quarterly Sales".into();

        let chart = 
            Chart::new(
                1,
                ChartKind::Pie,
            )
            .title(title)
            .legend(LegendPos::Right);
        
        let theme = Theme::TokyoNight;

        let data = load_data();
        let data_len = data.len();

        let data = ChartData {
            categories: vec!["North".into(), "South".into(), "East".into(), 
                            "West".into(), "Central".into(), "Online".into()],
            series: vec![
                SeriesData {
                    name: "Market Share".into(),
                    values: data,
                    ..Default::default()
                },
            ],
            palette: (0..data_len).map(|d| chart.series_color(&theme, d)).collect(),
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


fn load_data() -> Vec<Option<f64>> {
    let data = vec![
        217_420.0, 164_650.0, 234_920.0, 271_960.0,
        130_490.0, 362_100.0 ];
    
    data.iter().map(|d| Some(d/1381540.0)).collect()
    
}