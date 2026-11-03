# iced_charts

A charting library for the [Iced](https://github.com/iced-rs/iced) GUI framework with zoom/pan support.

![Demo GIF - Charts in action](./demo.gif)


## Features

- **20+ Chart Types**: See examples
- **Theme-Aware Colors**: Uniform series colors in OKLCH color space, seeded from the active theme's primary accent and background
- **Category & Numeric Axes**: Handles both categorical (line/bar) and numeric (scatter/stock) data seamlessly
- **Trendlines**: Linear regression and moving averages on supported chart types
- **YAML Configuration**: Load charts from `.yml` files with optional CSV data sources (see the stock examples)
- **Pan/Zoom** support for scatter, line, stock ( individual axis zooming later )

## Code Sources
A year ago (end of 2025), I developed an iced charts program but had a number of bugs in it that were difficult to fix, so never published it.  Being busy with other things, I set it aside and as you know, time flies. So Iced has changed a lot during the last year and so I decided to go back to the old charts repo in Oct 2026 and restart the development.  

About this time, the Craft projects were being published and I took a look at the GridCraft repo.  I found some code that solved my major problem of fitting things on the canvas by repeating or finalizing the settings, which I had not thought about.  Therefore, this repo has a small part of the GridCraft, chart only part with bugs fixed and many hard coded parameters removed, my old repo, and some new things like theme based colors (which I'm still not totally happy with).  

In addition, this repo has the pan and zoom which GridCraft doesn't have.  GridCraft also uses egui and of course, this uses Iced.  My old repo has both pan/zoom and individual axis scrolling along with up to 4 left and right axes.  This repo only has one left and one right.  I'll update this later with axis scrolling and add at least two axes per side, hopefully.

I use Claude to help with things, especially for debugging.  I find debugging a gui especially difficult with one having to visually find the issues and trace the code throught the canvas and the messaging system.  Claude is invaluable in this aspect.

In the example selectable_charts, I use a iced_menu repo I made a few months ago, https://github.com/ray-iced-projects/iced_menu.

This repo points to the current Master version of Iced, not the crates version.  I wanted to use the latest changes.

## Examples

- The colors used are mostly the first few theme colors but are selected by an index so easily changed.
- Each example also has a selectable theme, so give it a try.
- The examples show two different methods of creating a chart, code in the view() section or by using a yml file.
- In the selectable_charts folder, you have a config.yml that when loaded will create a user_config.yml.
- If you look at the line_2_charts' yml file you'll see that you can override the global settings for the canvas size.
- There are many chart examples but these really only cover a handful of chart type with slight modifications of the parameters.
- Enjoy!

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
cargo run --example line_2_charts
cargo run --example line_markers
cargo run --example line_stacked
cargo run --example selectable_charts # Charts loaded via a menu for file selection (native dialog, rfd crates)
cargo run --example pie
cargo run --example radar
cargo run --example scatter
cargo run --example stock
cargo run --example sunburst
cargo run --example treemap
cargo run --example waterfall
```

## License

MIT Licensed.

## Contributing

Bug reports and suggestions are welcomed, just open an issue.
