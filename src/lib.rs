//! Iced charts (layer L3, toolkit-free).
use iced::mouse;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::{
    Color, Fill, Point, Radians, Rectangle, Renderer, Size, Theme,
    Vector, Widget,
};


pub mod axis;
pub mod core;
pub mod model;
pub mod numfmt;
pub mod render;
pub mod resolve;
pub mod trend;


pub use model::features::{Chart, ChartKind, LegendPos, SparklineKind, Trendline, TrendlineKind};
use serde::{Deserialize, Serialize};

pub use axis::{Scale, format_number, nice_scale};
pub use render::render;

use crate::model::features::ChartText;


pub type Rgba = [u8; 4];

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
        fill: Option<Rgba>,
        stroke: Option<(Rgba, f32)>,
        radius: f32,
    },
    Line {
        pts: Vec<[f32; 2]>,
        color: Rgba,
        width: f32,
        dash: bool,
    },
    Polygon {
        pts: Vec<[f32; 2]>,
        fill: Rgba,
        stroke: Option<(Rgba, f32)>,
    },
    /// Pie/doughnut sector: centre, radii, angles in radians (0 = 12 o'clock, clockwise).
    Wedge {
        cx: f32,
        cy: f32,
        r_outer: f32,
        r_inner: f32,
        a0: f32,
        a1: f32,
        fill: Rgba,
        stroke: Option<(Rgba, f32)>,
    },
    Circle {
        cx: f32,
        cy: f32,
        r: f32,
        fill: Option<Rgba>,
        stroke: Option<(Rgba, f32)>,
    },
    /// Text anchored at `(x, y)` per `align`/`valign`, rotated by `rotation` radians about the anchor.
    Text(ChartText),
    TitleText(ChartText),
    YText(ChartText),
    XText(ChartText),
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
    #[serde(default)]
    pub palette: Vec<Rgba>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SeriesData {
    pub name: String,
    pub values: Vec<Option<f64>>,
    /// Numeric X values (scatter, bubble).
    pub x: Option<Vec<Option<f64>>>,
    /// Bubble sizes.
    pub sizes: Option<Vec<Option<f64>>>,
    pub color: Rgba,
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

/// A trendline to draw: the model settings with the line colour resolved against the theme.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrendlineData {
    pub spec: Trendline,
    pub color: Rgba,
}

impl Default for SeriesData {
    fn default() -> Self {
        SeriesData {
            name: String::new(),
            values: vec![],
            x: None,
            sizes: None,
            color: [0x15, 0x60, 0x82, 0xFF],
            kind: ChartKind::ColumnClustered,
            secondary: false,
            smooth: false,
            trendlines: vec![],
            number_format: "General".into(),
        }
    }
}

pub struct State {
    cache: canvas::Cache,
    chart: Chart,
    data: ChartData,
}

impl State {
    pub fn new(chart: Chart, data: ChartData) -> Self {
        State { cache: canvas::Cache::default(), chart, data }
    }

    pub fn view<'a>(&'a self) -> impl Widget<()> {
        Canvas::new(Grid { state: self })
            .width(Fill)
            .height(Fill)
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear();
    }
}

struct Grid<'a> {
    state: &'a State,
}

impl canvas::Program<()> for Grid<'_> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let content = self.state.cache.draw(renderer, bounds.size(), |frame| {
            let prims = render(
                &self.state.chart,
                &self.state.data,
                bounds.width,
                bounds.height,
                &ApproxMeasure,
            );
            for prim in &prims {
                draw_prim(frame, prim);
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

fn rgba_to_color(c: crate::Rgba) -> Color {
    Color::from_rgba8(c[0], c[1], c[2], c[3] as f32 / 255.0)
}

fn draw_prim(frame: &mut Frame, prim: &Prim) {
    
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
            if let Some(rgba) = fill {
                frame.fill(&path, rgba_to_color(*rgba));
            }
            if let Some((rgba, lw)) = stroke {
                frame.stroke(
                    &path,
                    Stroke::default().with_color(rgba_to_color(*rgba)).with_width(*lw),
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
                Stroke::default().with_color(rgba_to_color(*color)).with_width(*width),
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
            frame.fill(&path, rgba_to_color(*fill));
            if let Some((rgba, lw)) = stroke {
                frame.stroke(
                    &path,
                    Stroke::default().with_color(rgba_to_color(*rgba)).with_width(*lw),
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
            frame.fill(&path, rgba_to_color(*fill));
            if let Some((rgba, lw)) = stroke {
                frame.stroke(
                    &path,
                    Stroke::default().with_color(rgba_to_color(*rgba)).with_width(*lw),
                );
            }
        }
        Prim::Circle { cx, cy, r, fill, stroke } => {
            let path = Path::circle(Point::new(*cx, *cy), *r);
            if let Some(rgba) = fill {
                frame.fill(&path, rgba_to_color(*rgba));
            }
            if let Some((rgba, lw)) = stroke {
                frame.stroke(
                    &path,
                    Stroke::default().with_color(rgba_to_color(*rgba)).with_width(*lw),
                );
            }
        },
        Prim::Text(ct) => {
            frame.with_save(|f| {
                f.translate(Vector::new(ct.x, ct.y));
                if ct.rotation != 0.0 {
                    f.rotate(Radians(ct.rotation));
                }
                f.fill_text(ct.clone().to_iced());
            });
        }, 
        Prim::TitleText(ct) => {
            frame.with_save(|f| {
                f.translate(Vector::new(ct.x, ct.y));
                
                if ct.rotation != 0.0 {
                    f.rotate(Radians(ct.rotation));
                }
                f.fill_text(ct.clone().to_iced());
            });
        },
        Prim::XText(ct) => {
            frame.with_save(|f| {
                f.translate(Vector::new(ct.x, ct.y));
                if ct.rotation != 0.0 {
                    f.rotate(Radians(ct.rotation));
                }
                f.fill_text(ct.clone().to_iced());
            });
        },
        Prim::YText(ct) => {
            frame.with_save(|f| {
                f.translate(Vector::new(ct.x, ct.y));
                if ct.rotation != 0.0 {
                    f.rotate(Radians(ct.rotation));
                }
                f.fill_text(ct.clone().to_iced());
            });
        }
    }
}
