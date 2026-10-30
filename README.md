# iced_charts

A comprehensive charting library for the [Iced](https://github.com/iced-rs/iced) GUI framework, providing interactive, theme-aware visualizations with zoom/pan support.

![Demo GIF - Charts in action](./demo.gif)

## Overview

`iced_charts` brings publication-quality charts to Iced applications with minimal setup. Charts automatically adapt to your app's theme, support interactive zoom and pan, and render thousands of data points efficiently.

## Features

- **20+ Chart Types**: Line, Area, Bar (clustered/stacked), Column, Scatter, Bubble, Pie, Doughnut, Radar, Funnel, Waterfall, Histogram, Box-Whisker, Stock (OHLC), Sunburst, Treemap, and more
- **Interactive Controls**: Scroll to zoom (toward cursor), drag to pan, double-click to reset
- **Theme-Aware Colors**: Generates distinct, perceptually uniform series colors in OKLCH color space, seeded from the active theme's primary accent and background
- **Category & Numeric Axes**: Handles both categorical (line/bar) and numeric (scatter/stock) data seamlessly
- **Trendlines**: Linear regression and moving averages on supported chart types
- **YAML Configuration**: Load charts from `.yml` files with optional CSV data sources
- **High Performance**: Decimates dense line series, limits markers/labels, renders efficiently even with 100k+ points
- **Type-Safe**: Full Rust type safety; serialization via `serde` for data interchange

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
iced_charts = { path = "path/to/iced_charts" }
```

Requires `iced` (uses the git checkout at the version in this repo).

## Quick Start

### Basic Usage

```rust
use iced_charts::{Chart, ChartData, ChartKind, ChartState, SeriesData};
use iced::widget::canvas::Canvas;
use iced::Element;

// Create chart metadata
let chart = Chart {
    kind: ChartKind::Line,
    title: Some("My Chart".to_string()),
    ..Default::default()
};

// Populate with data
let mut data = ChartData::default();
data.categories = vec!["Jan", "Feb", "Mar"].iter().map(|s| s.to_string()).collect();
data.series.push(SeriesData {
    name: "Series 1".to_string(),
    values: vec![Some(10.0), Some(15.0), Some(12.0)],
    ..Default::default()
});

// Create the chart widget
let mut state = ChartState::new(chart, data);
let view = state.view();

// In your app's view, return the chart as an Element
```

### From YAML Configuration

```rust
use iced_charts::load_chart_from_yaml;

let (chart, data) = load_chart_from_yaml("path/to/chart.yml")
    .expect("Failed to load chart");
let state = ChartState::new(chart, data);
```

Example `chart.yml`:
```yaml
charts:
  - id: 1
    kind: Line
    title: "Sales Over Time"
    x_title: "Month"
    y_title: "Revenue ($)"
    legend: "Right"
    data_source:
      file: "sales.csv"
      category_column: "Month"
    series:
      - name: "Region A"
        y_column: "Revenue_A"
      - name: "Region B"
        y_column: "Revenue_B"
```

## Chart Types

Supported chart kinds (with interactive zoom/pan where applicable):

| Category | Types |
|----------|-------|
| **Line-Based** | Line, Line+Markers, Line Stacked, Line Stacked 100%, Area, Area Stacked |
| **Bar/Column** | Bar (Clustered/Stacked/Stacked 100%), Column (Clustered/Stacked/Stacked 100%) |
| **Scatter/Bubble** | Scatter, Scatter+Lines, Bubble |
| **Pie-Style** | Pie, Doughnut |
| **Statistical** | Histogram, Box-Whisker |
| **Financial** | Stock (OHLC candles) |
| **Hierarchical** | Sunburst, Treemap |
| **Other** | Funnel, Waterfall, Combo, Radar |

## Interactive Features

### Zoom & Pan

- **Scroll** over the plot area to zoom toward the cursor on both axes
- **Drag** to pan smoothly across the data
- **Double-click** to reset zoom/pan to auto-scale

Works on:
- **Line charts**: zoom/pan on both category (X) and numeric (Y) axes
- **Scatter/Bubble**: zoom/pan on both numeric X and Y axes
- **Stock charts**: zoom/pan on both date (category X) and price (Y) axes

### Axis Label Density

Labels automatically thin out when zoomed or panned narrow, preventing overlap, and become denser as you zoom in to reveal more detail. The X-axis (category or numeric) label step size is intelligently computed so it looks good at any zoom level.

## Color Generation

Series colors are generated in **OKLCH** (perceptually uniform) color space:

1. **Hue seed** from the theme's primary accent (with fallback to blue for near-grey accents)
2. **Lightness** chosen from the theme's background luminance:
   - Dark themes get light, high-contrast series
   - Light themes get darker series to maintain visual weight
3. **Even spacing**: first 8 series are evenly distributed around the hue wheel; later series interleave via bit-reversal, ensuring unlimited distinct colors

### Benefits
- **Theme-aware**: automatically adapts to Light, Dark, CatppuccinFrappe, Dracula, Nord, TokyoNight, custom themes, etc.
- **Colorblind-friendly**: distinct hues at consistent saturation/lightness
- **No hand-tuning needed**: works with any theme or custom color scheme

## Series Data & Configuration

### SeriesData Fields

```rust
pub struct SeriesData {
    pub name: String,
    pub values: Vec<Option<f64>>,
    pub color: Option<Color>,                 // Auto-assigned if None
    pub smooth: bool,                         // Curve lines smoothly
    pub number_format: String,                // "General", "Percent", "Currency", etc.
    pub sizes: Option<Vec<Option<f64>>>,      // For bubble charts
    pub x: Option<Vec<Option<f64>>>,          // For scatter/bubble charts
    pub trendlines: Vec<Trendline>,
    pub data_labels: Option<bool>,
    // ... more fields
}
```

### Trendlines

Attach linear or moving-average trendlines to any series:

```rust
use iced_charts::{SeriesData, Trendline, TrendlineKind};

let mut series = SeriesData::default();
series.trendlines.push(Trendline {
    kind: TrendlineKind::Linear,
    name: Some("Trend".to_string()),
    ..Default::default()
});
```

## Examples

The repo includes runnable examples:

- **`line`** — Simple line chart demonstrating zoom/pan
- **`charts`** (multi-chart example) — File browser to load various chart configurations from YAML
  - Navigate to `examples/multi_charts/data/` and load `stock_PFE.yml` for an interactive 200-point stock chart with candles

```bash
cargo run --example line          # Line chart with zoom/pan
cargo run --example charts        # Multi-chart browser (try stock_PFE.yml)
```

## Architecture

### Layers

1. **L0 (Core)**: `axis.rs` — coordinate mapping, scale/tick generation
2. **L1 (Model)**: `model/`, `core/` — chart configuration, data structures, number formatting
3. **L2 (Rendering)**: `render.rs` — layout and primitive generation for all chart types
4. **L3 (Iced Integration)**: `lib.rs` — canvas `Program`, interactive event handling, theme integration

### Interaction Flow

1. User scrolls/drags over the chart canvas
2. `Grid::update()` converts mouse events to `ChartMessage` (Zoom/Pan/Reset)
3. App calls `ChartState::apply(msg)` to update the internal `ChartView`
4. Next `view()` call passes the view to `render_scene()`, which applies overrides and returns both primitives and layout metadata
5. Primitives are drawn with clipping so geometry stays inside the plot area

## Performance Notes

- **Point limits**: 100k points per series, 255 series per chart
- **Line decimation**: Lines with >2000 points are decimated (min/max per pixel column)
- **Marker limit**: 5000 markers drawn per series
- **Data labels**: skipped when a series exceeds 200 points
- **Caching**: chart primitives are cached; redraw only on data or view change

## Configuration Reference

### Chart Fields (YAML or Rust)

```yaml
charts:
  - id: 1
    kind: Line                          # Chart type
    title: "Chart Title"
    x_title: "X Axis Label"
    y_title: "Y Axis Label"
    legend: "Right"                     # "Top", "Bottom", "Left", "Right", "None"
    gridlines: true
    data_labels: false
    label_size: 10.0
    width: 800.0
    height: 600.0
    data_source:
      file: "data.csv"
      category_column: "Date"
    series:
      - name: "Series 1"
        y_column: "Column1"
        color: [255, 0, 0, 255]         # Optional; auto-assigned otherwise
        smooth: false
        number_format: "General"
        trendlines:
          - kind: "Linear"
            name: "Trend"
```

## License

Licensed under your project's license. See [LICENSE](./LICENSE) for details.

## Contributing

Bug reports and suggestions welcome! For major changes, please open an issue first.

