//! Iced charts (layer L3, toolkit-free).
use iced::mouse;
use iced::mouse::ScrollDelta;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::{
    Event, Point, Radians, Rectangle, Renderer, Size, Theme,
    Vector, Widget,
};
use std::time::Instant;


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
pub use render::{render_scene, render_scene_with_theme, ChartView, PlotMeta};
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
    /// Clip marker: `Some(rect)` starts clipping subsequent prims to `rect`; `None` ends it.
    /// Used for zoom/pan so series geometry outside the plot area is hidden.
    Clip(Option<Rectangle>),
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
    /// Optional per-series kind override (used by combo/mixed charts).
    /// When `None`, the chart-level `Chart.kind` is used.
    pub kind: Option<ChartKind>,
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
            kind: None,
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
    view: ChartView,
}

impl ChartState {
    pub fn new(chart: Chart, data: ChartData) -> Self {
        ChartState { 
            cache: canvas::Cache::default(), 
            chart, 
            data, 
            view: ChartView::default(),
        }
    }

    pub fn view<'a>(&'a self) -> impl Widget<ChartMessage> {
        Canvas::new(Grid { state: self })
            .width(self.chart.width)
            .height(self.chart.height)
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear();
    }

    /// The current interactive view (zoom/pan overrides).
    pub fn view_state(&self) -> ChartView {
        self.view
    }

    /// Replace the interactive view (zoom/pan) and redraw.
    pub fn set_view(&mut self, view: ChartView) {
        self.view = view;
        self.cache.clear();
    }

    /// Clear any zoom/pan and return to the auto-scaled view.
    pub fn reset_view(&mut self) {
        self.view = ChartView::default();
        self.cache.clear();
    }

    /// Apply an interactive [`ChartMessage`] emitted by the canvas. Returns `true` if the view changed.
    pub fn apply(&mut self, message: ChartMessage) -> bool {
        match message {
            ChartMessage::ViewChanged(view) => {
                self.set_view(view);
                true
            }
            ChartMessage::Reset => {
                let changed = !self.view.is_default();
                self.reset_view();
                changed
            }
        }
    }

    pub fn set_width(&mut self, width: f32) {
        self.chart.width = width;
    }

    pub fn set_height(&mut self, height: f32) {
        self.chart.height = height;
    }

    pub fn canvas_size(&self) -> (f32, f32) {
        (self.chart.width, self.chart.height)
    }

    pub fn apply_default_canvas_size(&mut self, width: f32, height: f32) {
        if !self.chart.has_explicit_size {
            self.chart.width = width;
            self.chart.height = height;
        }
    }
}

/// Messages emitted by an interactive chart canvas (zoom and pan).
#[derive(Debug, Clone)]
pub enum ChartMessage {
    /// A new visible range was requested (scroll to zoom, or drag to pan).
    ViewChanged(ChartView),
    /// Reset to the auto-scaled view (double-click).
    Reset,
}

/// Canvas interaction state: drag tracking for panning plus a click timestamp for double-click.
#[derive(Debug, Default)]
pub struct Interaction {
    drag: Option<Drag>,
    last_press: Option<Instant>,
}

#[derive(Debug, Clone, Copy)]
struct Drag {
    start: Point,
    start_view: ChartView,
    meta: PlotMeta,
    moved: bool,
}

/// Cursor-anchored 1-D zoom. `anchor` is the fraction (0..1) under the cursor that stays fixed.
fn zoom_1d(min: f64, max: f64, zoom_in: bool, anchor: f64) -> Option<(f64, f64)> {
    let anchor = anchor.clamp(0.0, 1.0);
    let size = max - min;
    if !(size.is_finite() && size > 0.0) {
        return None;
    }
    let factor = 0.12;
    let av = min + anchor * size;
    let new_size = if zoom_in { size * (1.0 - factor) } else { size * (1.0 + factor) };
    if !(new_size.is_finite() && new_size > 1e-9) {
        return None;
    }
    let nmin = av - anchor * new_size;
    let nmax = av + (1.0 - anchor) * new_size;
    (nmax > nmin && nmin.is_finite() && nmax.is_finite()).then_some((nmin, nmax))
}

fn scroll_sign(delta: &ScrollDelta) -> f32 {
    match delta {
        ScrollDelta::Lines { y, .. } | ScrollDelta::Pixels { y, .. } => *y,
    }
}

/// Build the view for a scroll-zoom at `cursor`, anchored under the pointer.
fn zoom_view(view: ChartView, meta: &PlotMeta, cursor: Point, zoom_in: bool) -> Option<ChartView> {
    let plot = meta.plot;
    if plot.width <= 0.0 || plot.height <= 0.0 {
        return None;
    }
    let ax = ((cursor.x - plot.x) / plot.width).clamp(0.0, 1.0) as f64;
    let ay = (((plot.y + plot.height) - cursor.y) / plot.height).clamp(0.0, 1.0) as f64;
    let mut nv = view;
    let mut changed = false;
    if let Some(ny) = zoom_1d(meta.y.0, meta.y.1, zoom_in, ay) {
        nv.y = Some(ny);
        changed = true;
    }
    if let Some((x0, x1)) = meta.x {
        if let Some(nx) = zoom_1d(x0, x1, zoom_in, ax) {
            nv.x = Some(nx);
            changed = true;
        }
    } else if let Some((c0, c1)) = meta.xcat {
        if let Some(nc) = zoom_1d(c0, c1, zoom_in, ax) {
            nv.xcat = Some(nc);
            changed = true;
        }
    }
    changed.then_some(nv)
}

/// Build the view for a pan drag of `(dx, dy)` pixels from the drag start.
fn pan_view(drag: &Drag, dx: f32, dy: f32) -> ChartView {
    let plot = drag.meta.plot;
    let mut nv = drag.start_view;
    let (y0, y1) = drag.meta.y;
    let ddy = dy as f64 * (y1 - y0) / plot.height.max(1.0) as f64;
    nv.y = Some((y0 + ddy, y1 + ddy));
    if let Some((x0, x1)) = drag.meta.x {
        let ddx = dx as f64 * (x1 - x0) / plot.width.max(1.0) as f64;
        nv.x = Some((x0 - ddx, x1 - ddx));
    } else if let Some((c0, c1)) = drag.meta.xcat {
        let ddx = dx as f64 * (c1 - c0) / plot.width.max(1.0) as f64;
        nv.xcat = Some((c0 - ddx, c1 - ddx));
    }
    nv
}

struct Grid<'a> {
    state: &'a ChartState,
}

impl canvas::Program<ChartMessage> for Grid<'_> {
    type State = Interaction;

    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<ChartMessage>> {
        let Some(pos) = cursor.position_in(bounds) else {
            return None;
        };
        let Event::Mouse(mouse_event) = event else {
            return None;
        };

        // Lay out the chart once so we know the plot rect and the visible data ranges.
        let meta = || {
            render_scene(
                &self.state.chart,
                &self.state.data,
                bounds.width,
                bounds.height,
                &ApproxMeasure,
                self.state.view,
            )
            .1
        };

        match mouse_event {
            mouse::Event::WheelScrolled { delta } => {
                let sign = scroll_sign(delta);
                if sign == 0.0 {
                    return None;
                }
                let meta = meta()?;
                if !meta.plot.contains(pos) {
                    return None;
                }
                let view = zoom_view(self.state.view, &meta, pos, sign > 0.0)?;
                Some(canvas::Action::publish(ChartMessage::ViewChanged(view)).and_capture())
            }
            mouse::Event::ButtonPressed(mouse::Button::Left) => {
                let meta = meta()?;
                if !meta.plot.contains(pos) {
                    return None;
                }
                // A second press within the double-click window resets the view.
                if state.last_press.is_some_and(|t| t.elapsed().as_millis() < 350) {
                    state.last_press = None;
                    state.drag = None;
                    return Some(canvas::Action::publish(ChartMessage::Reset).and_capture());
                }
                state.last_press = Some(Instant::now());
                state.drag = Some(Drag { start: pos, start_view: self.state.view, meta, moved: false });
                None
            }
            mouse::Event::CursorMoved { .. } => {
                let drag = state.drag.as_mut()?;
                let dx = pos.x - drag.start.x;
                let dy = pos.y - drag.start.y;
                if !drag.moved && (dx * dx + dy * dy).sqrt() <= 3.0 {
                    return None;
                }
                drag.moved = true;
                state.last_press = None; // a drag is not a click
                let view = pan_view(drag, dx, dy);
                Some(canvas::Action::publish(ChartMessage::ViewChanged(view)).and_capture())
            }
            mouse::Event::ButtonReleased(mouse::Button::Left) => {
                state.drag = None;
                None
            }
            _ => None,
        }
    }

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
                    Point::new(0.0, 1.0),
                    Size::new(bounds.width, bounds.height-1.0),
                ),
                Stroke::default()
                    .with_color(border_color)
                    .with_width(2.0),
            );

            let (prims, _) = render_scene_with_theme(
                &self.state.chart,
                &self.state.data,
                bounds.width,
                bounds.height,
                &ApproxMeasure,
                self.state.view,
                theme,
            );
            draw_prims(frame, &prims, theme);
        });
        vec![content]
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if state.drag.is_some_and(|d| d.moved) {
            mouse::Interaction::Grabbing
        } else if cursor.is_over(bounds) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}

/// Draws a prim list, honouring [`Prim::Clip`] markers by clipping bracketed prims to the plot rect.
fn draw_prims(frame: &mut Frame, prims: &[Prim], theme: &Theme) {
    let mut i = 0;
    while i < prims.len() {
        match &prims[i] {
            Prim::Clip(Some(rect)) => {
                let start = i + 1;
                let mut end = start;
                while end < prims.len() && !matches!(prims[end], Prim::Clip(None)) {
                    end += 1;
                }
                let slice = &prims[start..end];
                frame.with_clip(*rect, |f| {
                    for p in slice {
                        draw_prim(f, p, theme);
                    }
                });
                i = end + 1;
            }
            Prim::Clip(None) => i += 1,
            other => {
                draw_prim(frame, other, theme);
                i += 1;
            }
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
        Prim::Clip(_) => {}
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
            let color = crate::resolve::series_color(&theme, i, 0.25);
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
                let csv = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                        .join("examples/selectable_charts/data/scatter.csv")
                        .to_string_lossy()
                        .replace('\\', "/");

                let yaml = format!(
                    "charts:\n  - id: 1\n    kind: Scatter\n    title: \"CSV index\"\n    data_source:\n      file: \"{}\"\n    series:\n      - name: \"Data 0\"\n        x_column: 1\n        y_column: 2\n      - name: \"Inline\"\n        x_values: [0.0]\n        y_values: [0.0]\n",
                    csv
                );
        let config = load_chart_from_yaml(&yaml).expect("parse yaml");
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
        let csv = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("examples/selectable_charts/data/stock_PFE.csv")
            .to_string_lossy()
            .replace('\\', "/");

                let yaml = format!(
                        "charts:\n  - id: 1\n    kind: Stock\n    title: \"CSV named\"\n    data_source:\n      file: \"{}\"\n      category_column: \"Date\"\n    series:\n      - name: \"Close\"\n        y_column: \"Close\"\n",
                        csv
                );
        let config = load_chart_from_yaml(&yaml).expect("parse yaml");
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

