# iced_charts

A charting library for the [Iced](https://github.com/iced-rs/iced) GUI framework with zoom/pan support.

![Demo GIF - Charts in action](./demo.gif)


## Features

- **20+ Chart Types**: Line, Area, Bar (clustered/stacked), Column, Scatter, Bubble, Pie, Doughnut, Radar, Funnel, Waterfall, Histogram, Box-Whisker, Stock (OHLC), Sunburst, Treemap, and more
- **Interactive Controls**: Scroll to zoom, drag to pan, double-click to reset
- **Theme-Aware Colors**: Uniform series colors in OKLCH color space, seeded from the active theme's primary accent and background
- **Category & Numeric Axes**: Handles both categorical (line/bar) and numeric (scatter/stock) data seamlessly
- **Trendlines**: Linear regression and moving averages on supported chart types
- **YAML Configuration**: Load charts from `.yml` files with optional CSV data sources

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


## Examples

The repo includes many examples:

```bash
cargo run --example area
cargo run --example area_stacked
cargo run --example bar_clustered
cargo run --example bar_stacked
cargo run --example bar_stacked_100
cargo run --example box_whisker
cargo run --example bubble
cargo run --example column_clustered
cargo run --example column_stacked
cargo run --example column_stacked_100
cargo run --example combo
cargo run --example doughnut
cargo run --example funnel
cargo run --example histogram
cargo run --example line
cargo run --example line_markers
cargo run --example line_stacked
cargo run --example multi_charts      # Charts loaded via a menu for file selection (native dialog, rfd crates)
cargo run --example pie
cargo run --example radar
cargo run --example scatter
cargo run --example stock
cargo run --example sunburst
cargo run --example treemap
cargo run --example waterfall
```

## License

Licensed under your project's license. See [LICENSE](./LICENSE) for details.

## Contributing

Bug reports and suggestions are welcomed, just open an issue.
