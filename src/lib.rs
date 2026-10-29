//! Iced charts (layer L3, toolkit-free).
use iced::mouse;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::{
    Point, Radians, Rectangle, Renderer, Size, Theme,
    Vector, Widget,
};


pub mod axis;
pub mod core;
pub mod model;
pub mod numfmt;
pub mod render;
pub mod resolve;
pub mod trend;
pub mod config;
pub mod csv_data;


pub use model::features::{Chart, ChartKind, LegendPos, SparklineKind, Trendline, TrendlineKind};
use serde::{Deserialize, Serialize};

pub use axis::{Scale, format_number, nice_scale};
pub use render::render;
pub use config::load_chart_from_yaml;

use crate::model::features::ChartText;

mod color_serde {
    use serde::{Deserialize, Deserializer};
    use iced::Color;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Color>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rgba_vec: Vec<[u8; 4]> = Deserialize::deserialize(deserializer)?;
        Ok(rgba_vec
            .into_iter()
            .map(|[r, g, b, a]| Color::from_rgba(
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                a as f32 / 255.0,
            ))
            .collect())
    }

    pub mod single_color {
        use serde::{Deserialize, Deserializer, Serialize, Serializer};
        use iced::Color;

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
        where
            D: Deserializer<'de>,
        {
            let [r, g, b, a] = <[u8; 4]>::deserialize(deserializer)?;
            Ok(Color::from_rgba(
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                a as f32 / 255.0,
            ))
        }

        pub fn serialize<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let [r, g, b, a] = [
                (color.r * 255.0) as u8,
                (color.g * 255.0) as u8,
                (color.b * 255.0) as u8,
                (color.a * 255.0) as u8,
            ];
            [r, g, b, a].serialize(serializer)
        }
    }
}


#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum HAlign {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum VAlign {
    #[default]
    Top,
    Middle,
    Bottom,
}

/// A drawing primitive in chart coordinates (origin top-left, y down, units = points/pixels).
#[derive(Clone, Debug)]
pub enum Prim {
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        fill: Option<iced::Color>,
        stroke: Option<(iced::Color, f32)>,
        radius: f32,
    },
    Line {
        pts: Vec<[f32; 2]>,
        color: iced::Color,
        width: f32,
        dash: bool,
    },
    Polygon {
        pts: Vec<[f32; 2]>,
        fill: iced::Color,
        stroke: Option<(iced::Color, f32)>,
    },
    /// Pie/doughnut sector: centre, radii, angles in radians (0 = 12 o'clock, clockwise).
    Wedge {
        cx: f32,
        cy: f32,
        r_outer: f32,
        r_inner: f32,
        a0: f32,
        a1: f32,
        fill: iced::Color,
        stroke: Option<(iced::Color, f32)>,
    },
    Circle {
        cx: f32,
        cy: f32,
        r: f32,
        fill: Option<iced::Color>,
        stroke: Option<(iced::Color, f32)>,
    },
    Grid {
        pts: Vec<[f32; 2]>,
        width: f32,
    },
    Text(ChartText),
}

/// Measures text width (the UI passes real font metrics; tests use a fixed-width approximation).
pub trait Measure {
    fn text_width(&self, text: &str, size: f32, bold: bool) -> f32;
}

/// Fixed-width approximation: `0.55 × size` per character (bold 10% wider).
#[derive(Clone, Copy, Debug, Default)]
pub struct ApproxMeasure;

impl Measure for ApproxMeasure {
    fn text_width(&self, text: &str, size: f32, bold: bool) -> f32 {
        let n = text.chars().count() as f32;
        n * 0.55 * size * if bold { 1.1 } else { 1.0 }
    }
}

/// Resolved data: series names, categories and values pulled from the workbook.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ChartData {
    pub categories: Vec<String>,
    pub series: Vec<SeriesData>,
    /// Workbook-theme colours for the points of pie-like charts, repeating (point `j` uses entry `j % len`); empty uses the default theme.
    #[serde(default, deserialize_with = "color_serde::deserialize")]
    pub palette: Vec<iced::Color>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SeriesData {
    pub name: String,
    pub values: Vec<Option<f64>>,
    /// Numeric X values (scatter, bubble).
    pub x: Option<Vec<Option<f64>>>,
    /// Bubble sizes.
    pub sizes: Option<Vec<Option<f64>>>,
    #[serde(with = "color_serde::single_color")]
    pub color: iced::Color,
    /// Per-series kind (combo charts).
    pub kind: ChartKind,
    /// Plotted against the secondary (right) value axis.
    pub secondary: bool,
    /// Draw line segments as a smoothed (spline) curve.
    pub smooth: bool,
    /// Trendlines to fit and draw over this series.
    pub trendlines: Vec<TrendlineData>,
    /// Number format code of the source values ("General" by default).
    pub number_format: String,
}

impl Default for SeriesData {
    fn default() -> Self {
        SeriesData {
            name: String::new(),
            values: vec![],
            x: None,
            sizes: None,
            color: iced::Color::from_packed_rgba8(0x156082, 1.0),
            kind: ChartKind::ColumnClustered,
            secondary: false,
            smooth: false,
            trendlines: vec![],
            number_format: "General".into(),
        }
    }
}

/// A trendline to draw: the model settings with the line colour resolved against the theme.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrendlineData {
    pub spec: Trendline,
    #[serde(with = "color_serde::single_color")]
    pub color: iced::Color,
}


pub struct ChartState {
    cache: canvas::Cache,
    chart: Chart,
    data: ChartData,
}

impl ChartState {
    pub fn new(chart: Chart, data: ChartData) -> Self {
        ChartState { 
            cache: canvas::Cache::default(), 
            chart, 
            data, 
        }
    }

    pub fn view<'a>(&'a self) -> impl Widget<()> {
        Canvas::new(Grid { state: self })
            .width(self.chart.width)
            .height(self.chart.height)
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear();
    }

    pub fn set_width(&mut self, width: f32) {
        self.chart.width = width;
    }

    pub fn set_height(&mut self, height: f32) {
        self.chart.height = height;
    }
}

struct Grid<'a> {
    state: &'a ChartState,
}

impl <Message>canvas::Program<Message> for Grid<'_> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let content = self.state.cache.draw(renderer, bounds.size(), |frame| {
            // Draw the canvas border first using the theme's text color
            let border_color = theme.palette().background.weak.text;
            frame.stroke(
                &Path::rectangle(
                    Point::new(0.0, 0.0),
                    Size::new(bounds.width, bounds.height),
                ),
                Stroke::default()
                    .with_color(border_color)
                    .with_width(1.0),
            );

            let prims = render(
                &self.state.chart,
                &self.state.data,
                bounds.width,
                bounds.height,
                &ApproxMeasure,
            );
            for prim in &prims {
                draw_prim(frame, prim, theme);
            }
        });
        vec![content]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(bounds) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}

fn draw_prim(frame: &mut Frame, prim: &Prim, theme: &Theme) {
    
    match prim {
        Prim::Rect { x, y, w, h, fill, stroke, radius } => {
            let path = if *radius > 0.0 {
                let r = radius.min(*w / 2.0).min(*h / 2.0);
                Path::new(|b| {
                    b.move_to(Point::new(*x + r, *y));
                    b.line_to(Point::new(*x + *w - r, *y));
                    b.arc_to(Point::new(*x + *w, *y), Point::new(*x + *w, *y + r), r);
                    b.line_to(Point::new(*x + *w, *y + *h - r));
                    b.arc_to(Point::new(*x + *w, *y + *h), Point::new(*x + *w - r, *y + *h), r);
                    b.line_to(Point::new(*x + r, *y + *h));
                    b.arc_to(Point::new(*x, *y + *h), Point::new(*x, *y + *h - r), r);
                    b.line_to(Point::new(*x, *y + r));
                    b.arc_to(Point::new(*x, *y), Point::new(*x + r, *y), r);
                    b.close();
                })
            } else {
                Path::rectangle(Point::new(*x, *y), Size::new(*w, *h))
            };
            if let Some(color) = fill {
                frame.fill(&path, *color);
            }
            if let Some((color, lw)) = stroke {
                frame.stroke(
                    &path,
                    Stroke::default().with_color(*color).with_width(*lw),
                );
            }
        }
        Prim::Line { pts, color, width, .. } => {
            if pts.len() < 2 {
                return;
            }
            let path = Path::new(|b| {
                if let Some(first) = pts.first() {
                    b.move_to(Point::new(first[0], first[1]));
                    for pt in pts.iter().skip(1) {
                        b.line_to(Point::new(pt[0], pt[1]));
                    }
                }
            });
            frame.stroke(
                &path,
                Stroke::default().with_color(*color).with_width(*width),
            );
        }
        Prim::Grid { pts, width } => {
            let path = Path::new(|b| {
                if let Some(first) = pts.first() {
                    b.move_to(Point::new(first[0], first[1]));
                    for pt in pts.iter().skip(1) {
                        b.line_to(Point::new(pt[0], pt[1]));
                    }
                }
            });
            let bkg = theme.palette().background;
            let color = bkg.weak.text.scale_alpha(0.8);

            frame.stroke(
                &path,
                Stroke::default().with_color(color).with_width(*width),
            );
        }
        Prim::Polygon { pts, fill, stroke } => {
            if pts.len() < 3 {
                return;
            }
            let path = Path::new(|b| {
                if let Some(first) = pts.first() {
                    b.move_to(Point::new(first[0], first[1]));
                    for pt in pts.iter().skip(1) {
                        b.line_to(Point::new(pt[0], pt[1]));
                    }
                    b.close();
                }
            });
            frame.fill(&path, *fill);
            if let Some((color, lw)) = stroke {
                frame.stroke(
                    &path,
                    Stroke::default().with_color(*color).with_width(*lw),
                );
            }
        }
        Prim::Wedge { cx, cy, r_outer, r_inner, a0, a1, fill, stroke } => {
            // 0 = 12-o'clock clockwise; convert to standard math angles for trig
            use std::f32::consts::PI;
            let to_math = |a: f32| a - PI / 2.0;
            let (a_start, a_end) = (to_math(*a0), to_math(*a1));
            let steps =
                (((a_end - a_start).abs() * *r_outer) as usize).clamp(8, 64);
            let path = Path::new(|b| {
                b.move_to(Point::new(
                    *cx + *r_outer * a_start.cos(),
                    *cy + *r_outer * a_start.sin(),
                ));
                for i in 1..=steps {
                    let a = a_start + (a_end - a_start) * i as f32 / steps as f32;
                    b.line_to(Point::new(
                        *cx + *r_outer * a.cos(),
                        *cy + *r_outer * a.sin(),
                    ));
                }
                if *r_inner > 0.0 {
                    for i in (0..=steps).rev() {
                        let a = a_start + (a_end - a_start) * i as f32 / steps as f32;
                        b.line_to(Point::new(
                            *cx + *r_inner * a.cos(),
                            *cy + *r_inner * a.sin(),
                        ));
                    }
                } else {
                    b.line_to(Point::new(*cx, *cy));
                }
                b.close();
            });
            frame.fill(&path, *fill);
            if let Some((color, lw)) = stroke {
                frame.stroke(
                    &path,
                    Stroke::default().with_color(*color).with_width(*lw),
                );
            }
        }
        Prim::Circle { cx, cy, r, fill, stroke } => {
            let path = Path::circle(Point::new(*cx, *cy), *r);
            if let Some(color) = fill {
                frame.fill(&path, *color);
            }
            if let Some((color, lw)) = stroke {
                frame.stroke(
                    &path,
                    Stroke::default().with_color(*color).with_width(*lw),
                );
            }
        },
        Prim::Text(ct) => {
            frame.with_save(|f| {
                // Set the actual position instead of just translating to it
                let mut text_obj = ct.clone().to_iced(theme, ct.text_kind);
                text_obj.position = Point::new(ct.x, ct.y);
                
                if ct.rotation != 0.0 {
                    f.translate(Vector::new(ct.x, ct.y));
                    f.rotate(Radians(ct.rotation));
                    // For rotated text, need to reset position since we translated
                    text_obj.position = Point::new(0.0, 0.0);
                }
                
                f.fill_text(text_obj);
            });
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_color_conversion() {
        use serde_json::json;

        let json_data = json!({
            "categories": ["A", "B"],
            "series": [],
            "palette": [
                [255, 0, 0, 255],     // Red
                [0, 255, 0, 255],     // Green
                [0, 0, 255, 255],     // Blue
            ]
        });

        let result: Result<ChartData, _> = serde_json::from_value(json_data);
        match result {
            Ok(chart_data) => {
                assert_eq!(chart_data.palette.len(), 3);
                assert_eq!(chart_data.palette[0].r, 1.0);
                assert_eq!(chart_data.palette[0].g, 0.0);
                assert_eq!(chart_data.palette[0].b, 0.0);
            }
            Err(e) => panic!("Deserialization failed: {}", e),
        }
    }

    #[test]
    fn test_color_values_in_range() {
        // Test that series_color produces valid iced::Color values

        let theme = Theme::TokyoNight;

        for i in 0..12 {
            let color = crate::resolve::series_color(&theme, i);
            assert!(color.r >= 0.0 && color.r <= 1.0, "Red component out of range: {}", color.r);
            assert!(color.g >= 0.0 && color.g <= 1.0, "Green component out of range: {}", color.g);
            assert!(color.b >= 0.0 && color.b <= 1.0, "Blue component out of range: {}", color.b);
            assert!(color.a >= 0.0 && color.a <= 1.0, "Alpha component out of range: {}", color.a);
        }
    }

    #[test]
    fn test_csv_hybrid_index_columns() {
        // Paired/wide CSV with duplicate headers: select columns by index.
        use crate::config::load_chart_from_yaml;

        let yaml = r#"
charts:
  - id: 1
    kind: Scatter
    title: "CSV index"
    data_source:
      file: "examples/data/scatter.csv"
    series:
      - name: "Data 0"
        x_column: 1
        y_column: 2
      - name: "Inline"
        x_values: [0.0]
        y_values: [0.0]
"#;
        let config = load_chart_from_yaml(yaml).expect("parse yaml");
        let data = config.charts[0]
            .build_data(&Theme::Dark)
            .expect("build data from csv");

        assert_eq!(data.series.len(), 2);
        // scatter.csv has 200 data rows; CSV-backed series should match.
        assert_eq!(data.series[0].values.len(), 200);
        assert!(data.series[0].x.as_ref().unwrap().len() == 200);
        // Inline series coexists unchanged.
        assert_eq!(data.series[1].values.len(), 1);
    }

    #[test]
    fn test_csv_hybrid_named_columns() {
        // Tidy CSV with unique headers: select columns by name + category column.
        use crate::config::load_chart_from_yaml;

        let yaml = r#"
charts:
  - id: 1
    kind: Stock
    title: "CSV named"
    data_source:
      file: "examples/data/PFE_ohlc.csv"
      category_column: "Date"
    series:
      - name: "Close"
        y_column: "Close"
"#;
        let config = load_chart_from_yaml(yaml).expect("parse yaml");
        let data = config.charts[0]
            .build_data(&Theme::Dark)
            .expect("build data from csv");

        assert_eq!(data.series.len(), 1);
        // PFE_ohlc.csv has 201 data rows.
        assert_eq!(data.series[0].values.len(), 201);
        assert_eq!(data.categories.len(), 201);
        assert_eq!(data.categories[0], "2025-01-02");
    }
}

