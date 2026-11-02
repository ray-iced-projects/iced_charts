
use iced::widget::{PickList, container, row};
use iced::{Theme, Widget};
use iced_charts::model::features::{ChartText, TextKind};
use iced_charts::{ChartState, Chart, ChartData, ChartKind, LegendPos, SeriesData};
use iced_charts::model::theme_helpers::{theme_strings, str_to_theme};


pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .theme(|ex: &Example| ex.theme())
        .window_size( (1200, 600) )
        .centered()
        .run()
}

struct Example {
    grid1: ChartState,
    grid2: ChartState,
    selected_theme_str: String,
    selected_theme: Theme,
}

impl Default for Example {
    fn default() -> Self {
        let (col_chart, col_data) = column_chart();
        let (pie_chart, pie_data) = pie_chart();
        Example { 
            grid1: ChartState::new(col_chart, col_data),
            grid2: ChartState::new(pie_chart, pie_data),
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
                self.grid1.request_redraw();
                self.grid2.request_redraw();
            }
        }
    }

    fn view(&self) -> impl Widget<Message> {

        let pl = PickList::new(
                Some(self.selected_theme_str.clone()),
                theme_strings(), 
                |s: &String| s.clone(),
            )
            .width(150.0)
            .on_select(Message::OnThemeSelect)
            .boxed();
        
        row(vec![
            pl,
            container(self.grid1.view().map(|_| unreachable!()))
                .padding(20).boxed(),
            container(self.grid2.view().map(|_| unreachable!()))
                .padding(20).boxed()
        ]).padding(20.0).boxed()
    }

    fn theme(&self) -> Theme {
        self.selected_theme.clone()
    }
}


fn load_col_data(index: usize) -> Vec<Option<f64>> {
    let raw: Vec<Vec<f64>> = vec![
    vec![48_200.0, 51_340.0, 56_010.0, 61_870.0],
    vec![39_150.0, 37_980.0, 42_400.0, 45_120.0],
    vec![55_300.0, 58_910.0, 57_230.0, 63_480.0],
    vec![61_020.0, 64_400.0, 70_150.0, 76_390.0],
    vec![28_740.0, 31_260.0, 33_980.0, 36_510.0],
    vec![72_410.0, 81_630.0, 95_220.0, 112_840.0],
    ];
    let pivoted = pivot_regions(&raw);
    pivoted[index].clone()
}

fn load_col_cats() -> Vec<String> {
    vec!["North".into(), "South".into(), "East".into(),
        "West".into(), "Central".into(), "Online".into(),
        ]
}

fn pivot_regions(raw: &[Vec<f64>]) -> Vec<Vec<Option<f64>>> {
    let rows = raw.len();      // regions
    let cols = raw[0].len();   // quarters

    (0..cols)
        .map(|c| {
            (0..rows)
                .map(|r| Some(raw[r][c]))
                .collect()
        })
        .collect()
}

fn column_chart() -> (Chart, ChartData) {
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
            .width(500.0)
            .height(500.0)
            .title(title)
            .x_title(x_title)
            .y_title(y_title)
            .legend(LegendPos::Bottom);

        let theme = Theme::TokyoNight;

        let col_data = ChartData {
            categories: load_col_cats(),
            series: vec![
                SeriesData {
                    name: "Q1".into(),
                    values: load_col_data(0),
                    color: chart.series_color(&theme, 0),
                    ..Default::default()
                },
                SeriesData {
                    name: "Q2".into(),
                    values: load_col_data(1),
                    color: chart.series_color(&theme, 1),
                    ..Default::default()
                },
                SeriesData {
                    name: "Q3".into(),
                    values: load_col_data(2),
                    color: chart.series_color(&theme, 2),
                    ..Default::default()
                },
                SeriesData {
                    name: "Q4".into(),
                    values: load_col_data(3),
                    color: chart.series_color(&theme, 4), // color sequence changed
                    ..Default::default()
                },
            ],
            palette: vec![],
        };

        (chart, col_data)
        
}

fn pie_chart() -> (Chart, ChartData) {
    let mut title = ChartText::new(TextKind::Title);
        title.content = "Quarterly Sales".into();

        let chart = 
            Chart::new(
                1,
                ChartKind::Pie,
            )
            .width(500.0)
            .height(500.0)
            .title(title)
            .legend(LegendPos::Right);
        
        let theme = Theme::TokyoNight;

        let data:Vec<Option<f64>> = vec![
            217_420.0, 164_650.0, 234_920.0, 271_960.0, 130_490.0, 362_100.0 ]
            .iter().map(|d| Some(d/1381540.0)).collect();
        
        let data_len = data.len();

        let pie_data = ChartData {
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

        (chart, pie_data)

}
