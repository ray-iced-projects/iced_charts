//! Series colours derived from the active iced theme.
//!
//! Series colours are generated uniformally in the OKLCH colour space.
//! The starting hue is seeded from the theme's primary colour,
//! lightness is chosen from the theme's background, and
//! successive series step around the hue wheel so they stay evenly distinguishable.

use iced::{Color, Theme};

/// Cap on points per series (keeps whole-column references from exploding).
pub(crate) const MAX_POINTS: usize = 100_000;
/// Cap on series per chart.
pub(crate) const MAX_SERIES: usize = 255;
/// Cap on trendlines drawn per series.
pub(crate) const MAX_TRENDLINES: usize = 16;


pub fn series_color(theme: &Theme, index: usize, dark_luminance_threshold: f32) -> iced::Color{
    let theme_color = theme.palette().background.base.color;
    
    // Determine if background is dark using the configured threshold
    let is_dark = calculate_luminance(theme_color) < dark_luminance_threshold;
    
    let color = generate_series_color(theme_color, index, is_dark);
    if index == 0 {dbg!(&color);}
    color
}

fn calculate_luminance(color: Color) -> f32 {
    // WCAG relative luminance calculation
    0.2126 * color.r + 0.7152 * color.g + 0.0722 * color.b
}

fn generate_series_color(_bg_color: Color, index: usize, is_dark_bg: bool) -> Color {
    // Golden ratio conjugate (~137.508 degrees) distributes hues maximally
    let golden_ratio_conjugate = 0.618;
    let mut hue = 0.35; // Starting hue seed

    // Increment hue based on index using the golden ratio
    hue = (hue + golden_ratio_conjugate * (index as f64)) % 1.0;
    let hue_degrees = hue * 360.0;

    // Interleave Lightness and Saturation so adjacent colors look completely different
    let (saturation, lightness) = if is_dark_bg {
        match index % 3 {
            0 => (0.95, 0.85), // Very light and vivid
            1 => (0.75, 0.65), // Medium-light, slightly muted
            _ => (0.90, 0.75), // Crisp pastel
        }
    } else {
        match index % 3 {
            0 => (0.95, 0.25), // Deep, dark and vivid
            1 => (0.75, 0.45), // Medium-dark, slightly muted
            _ => (0.85, 0.35), // Solid earth tone
        }
    };

    // Convert HSL to RGB Color
    hsl_to_color(hue_degrees as f32, saturation as f32, lightness as f32)
}

fn hsl_to_color(h: f32, s: f32, l: f32) -> iced::Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match h {
        h if h < 60.0  => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _              => (c, 0.0, x),
    };

    iced::Color::from_rgb(
        (r + m).clamp(0.0, 1.0),
        (g + m).clamp(0.0, 1.0),
        (b + m).clamp(0.0, 1.0),
    )
}



