
use std::f32::consts::PI;

use iced::{Color, Font, Pixels, Point, alignment::{self, Vertical}, widget::{canvas, text::{Ellipsis, LineHeight, Shaping, Wrapping}}};
use iced::advanced::text::Alignment;
use serde::{Deserialize, Serialize};


const _AXIS_TITLE_SIZE: f32 = 10.0;

#[derive(Clone, Debug, Default)]
pub struct Chart {
    pub id: usize,
    pub kind: ChartKind,
    pub title: Option<ChartText>,
    pub x_title: Option<ChartText>,
    pub y_title: Option<ChartText>,
    pub series: Vec<Series>,
    pub legend: LegendPos,
    pub data_labels: bool,
    pub gridlines: bool,
    pub style: u32,
    pub source: Option<String>,
    pub by_rows: bool,
}

impl Chart {
    pub fn default(&self) -> Self {
        Self {
            id: 0,
            kind: ChartKind::Line,
            title: None,
            x_title: None,
            y_title: None,
            legend: LegendPos::Bottom,
            gridlines: true,
            data_labels: false,
            style: 0,
            series: vec![],
            source: None,
            by_rows: false,
        }
    }
    pub fn new(
        id: usize, 
        kind: ChartKind,
    ) -> Self {
            Self {
                id,
                kind,
                ..Default::default()
            }
    }

    pub fn title(mut self, title: ChartText) -> Self {
        self.title = Some(title);
        self
    }

    pub fn x_title(mut self, title: ChartText) -> Self {
        self.x_title = Some(title);
        self
    }

    pub fn y_title(mut self, title: ChartText) -> Self {
        self.y_title = Some(title);
        self
    }
}

#[derive(Debug, Clone)]
pub struct ChartText {
    pub x: f32,
    pub y: f32,
    pub text_kind: TextKind,
    pub content: String,
    pub position: Point,
    pub max_width: f32,
    pub color: Color,
    pub size: f32,
    pub line_height: f32,
    pub font: Font,
    pub align_x: Alignment,
    pub align_y: alignment::Vertical,
    pub shaping: Shaping,
    pub wrapping: Wrapping,
    pub ellipsis: Ellipsis,
    pub rotation: f32,
}

impl ChartText {
    pub fn new(kind: TextKind) -> ChartText {
        let (size, rotation) = match kind {
            TextKind::Title => (16.0, 0.0),
            TextKind::XTitle => (12.0, 0.0),
            TextKind::YTitle => (12.0, PI/2.0),
            TextKind::XAxisLabels => (12.0, 0.0),
            TextKind::YAxisLabels => (12.0, 0.0),
            _ => (10.0, 0.0)

        };
        ChartText {
            x: 0.0,
            y: 0.0,
            text_kind: kind,
            content: String::new(),
            position: Point::ORIGIN,
            max_width: f32::INFINITY,
            color: Color::BLACK,
            size: size,
            line_height: 1.2,
            font: Font::default(),
            align_x: Alignment::Default,
            align_y: Vertical::Top,
            shaping: Shaping::default(),
            wrapping: Wrapping::default(),
            ellipsis: Ellipsis::default(),
            rotation,
        }
    }

    pub fn to_iced(self, theme: &iced::Theme, kind: TextKind) -> canvas::Text {
        let bkg = theme.palette().background;
        let color = match kind {
            TextKind::Title => bkg.strongest.text,
            TextKind::XTitle => bkg.strongest.text,
            TextKind::YTitle => bkg.strongest.text,
            TextKind::XAxisLabels => bkg.strong.text,
            TextKind::YAxisLabels => bkg.strong.text,
            TextKind::Y2AxisLabels => bkg.strong.text,
            TextKind::Text => bkg.base.text,
        };

        let (align_x, align_y) = match kind {
            TextKind::Title => ( Alignment::Left, Vertical::Top ),
            TextKind::XTitle => ( Alignment::Center, Vertical::Top),
            TextKind::YTitle =>  ( Alignment::Center, Vertical::Top),
            TextKind::XAxisLabels => ( Alignment::Center, Vertical::Top ),
            TextKind::YAxisLabels => ( Alignment::Right, Vertical::Center ),
            TextKind::Y2AxisLabels => ( Alignment::Left, Vertical::Center ),
            TextKind::Text => ( Alignment::Left, Vertical::Center ),
        };

        canvas::Text { 
            content: self.content, 
            position: self.position, 
            max_width: self.max_width, 
            color: color, 
            size: Pixels(self.size), 
            line_height: LineHeight::Relative(self.line_height), 
            font: self.font, 
            align_x, 
            align_y, 
            shaping: self.shaping, 
            wrapping: self.wrapping, 
            ellipsis: self.ellipsis 
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextKind {
    #[default]
    Text,
    Title,
    XTitle,
    YTitle,
    XAxisLabels,
    YAxisLabels,
    Y2AxisLabels,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChartKind {
    Area,
    AreaStacked,
    BarClustered,
    BarStacked,
    BarStacked100,
    BoxWhisker,
    Bubble,
    #[default]
    ColumnClustered,
    ColumnStacked,
    ColumnStacked100,
    Combo,
    Doughnut,
    Funnel,
    Histogram,
    Line,
    LineMarkers,
    LineStacked,
    Pie,
    Radar,
    Scatter,
    ScatterLines,
    Stock,
    Sunburst,
    Treemap,
    Waterfall,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegendPos {
    None,
    #[default]
    Bottom,
    Top,
    Left,
    Right,
}

/// DrawingML preset line dash (`a:prstDash`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineDash {
    #[default]
    Solid,
    Dot,
    Dash,
    LgDash,
    DashDot,
    LgDashDot,
    LgDashDotDot,
    SysDash,
    SysDot,
    SysDashDot,
    SysDashDotDot,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Series {
    /// Formula text for the series name (`Sheet1!$B$1`) or a literal.
    pub name: Option<String>,
    pub categories: Option<String>,
    pub values: String,
    /// X values for scatter / bubble sizes.
    pub bubble_sizes: Option<String>,
    pub color: Option<Color>,
    /// Combo charts: this series drawn as a line on the secondary axis.
    pub secondary: bool,
    pub kind: Option<ChartKind>,
    /// Draw line segments as a smoothed (spline) curve.
    #[serde(default)]
    pub smooth: bool,
    /// Trendlines fitted to this series (Excel allows several per series).
    #[serde(default)]
    pub trendlines: Vec<Trendline>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SparklineKind {
    #[default]
    Line,
    Column,
    WinLoss,
}

/// Regression a trendline fits (`c:trendlineType`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrendlineKind {
    #[default]
    Linear,
    /// `y = c·e^(b·x)`.
    Exponential,
    /// `y = c·ln(x) + b`.
    Logarithmic,
    /// Polynomial of this order (2..=6).
    Polynomial(u8),
    /// `y = c·x^b`.
    Power,
    /// Average of this many consecutive points (2..=255).
    MovingAverage(u32),
}

/// A chart trendline (Add Chart Element ▸ Trendline, Format Trendline pane).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Trendline {
    pub kind: TrendlineKind,
    /// Custom name shown in the legend; `None` = automatic ("Linear (Sales)").
    pub name: Option<String>,
    /// Forecast past the last point, in x units (categories on a category axis).
    pub forward: f64,
    /// Forecast before the first point, in x units.
    pub backward: f64,
    /// Set Intercept: fixed y at x = 0 (linear, exponential, polynomial).
    pub intercept: Option<f64>,
    pub display_equation: bool,
    pub display_r_squared: bool,
    /// Line colour; `None` = the series colour.
    pub color: Option<Color>,
    /// Line width in points; `None` = default.
    pub width: Option<f32>,
    /// Line dash; `None` = default.
    pub dash: Option<LineDash>,
}
