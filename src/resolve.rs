//! Series colors

use crate::model::style::{ChartTheme, apply_tint};

/// Cap on points per series (keeps whole-column references from exploding).
pub(crate) const MAX_POINTS: usize = 100_000;
/// Cap on series per chart.
pub(crate) const MAX_SERIES: usize = 255;
/// Cap on trendlines drawn per series.
pub(crate) const MAX_TRENDLINES: usize = 16;

/// Tints used for series beyond the six theme accents: lighter, darker, lighter still…
const CYCLE_TINTS: [f64; 6] = [0.0, 0.4, -0.25, 0.6, -0.5, 0.2];
/// `series_color` repeats after this many colours (six accents × the tints).
// pub(crate) const PALETTE_CYCLE: usize = 6 * CYCLE_TINTS.len();

/// Colour of the `i`-th series: theme accents 1–6, then tinted variations of them.
pub fn series_color(theme: &ChartTheme, i: usize) -> iced::Color {
    let accent = i % 6;
    let cycle = (i / 6) % CYCLE_TINTS.len();
    let base = theme.colors.get(4 + accent).copied().unwrap_or(0x156082);
    let rgb = [(base >> 16) as u8, (base >> 8) as u8, base as u8];
    let tint = CYCLE_TINTS.get(cycle).copied().unwrap_or(0.0);
    let [r, g, b] = apply_tint(rgb, tint);
    iced::Color::from_rgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

/// The first `n` series colours for `theme`.
pub fn default_palette(theme: &ChartTheme, n: usize) -> Vec<iced::Color> {
    (0..n.min(4096)).map(|i| series_color(theme, i)).collect()
}
