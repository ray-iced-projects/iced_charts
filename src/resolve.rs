//! Series colours derived from the active iced theme.
//!
//! Series colours are generated uniformally in the OKLCH colour space.
//! The starting hue is seeded from the theme's primary colour,
//! lightness is chosen from the theme's background, and
//! successive series step around the hue wheel so they stay evenly distinguishable.

use iced::{Color, Theme};
use std::f64::consts::TAU;

/// Cap on points per series (keeps whole-column references from exploding).
pub(crate) const MAX_POINTS: usize = 100_000;
/// Cap on series per chart.
pub(crate) const MAX_SERIES: usize = 255;
/// Cap on trendlines drawn per series.
pub(crate) const MAX_TRENDLINES: usize = 16;

/// Hues per block: the first `BLOCK` series are spaced evenly around the wheel; later blocks
/// interleave between them (via a bit-reversal offset) so colours keep filling the gaps.
const BLOCK: usize = 8;
/// Target chroma (vividness) in OKLCH. Moderate so colours read as data, not neon.
const CHROMA: f64 = 0.125;

/// Colour of the `i`-th series for the given theme.
///
/// Evenly spaced OKLCH hues seeded from the theme's primary accent, at a lightness suited to the
/// theme's background. Works for any index and any (including custom) theme.
// pub fn series_color(iced_theme: &Theme, i: usize) -> Color {
//     let (h0, l_base) = theme_seed(iced_theme);
//     let block = i / BLOCK;
//     let slot = i % BLOCK;
//     let hue_step = TAU / BLOCK as f64;
//     // Bit-reversal offset interleaves later blocks between the first block's hues.
//     let hue = h0 + (slot as f64 + radical_inverse2(block)) * hue_step;
//     // Gentle lightness variation for later blocks so wrapped-around hues stay distinct.
//     let l_off = match block % 3 {
//         0 => 0.0,
//         1 => -0.07,
//         _ => 0.06,
//     };
//     let l = (l_base + l_off).clamp(0.35, 0.92);
//     oklch_to_color(l, CHROMA, hue)
// }

/// Starting hue (radians) and base lightness, derived from the theme's primary accent and background.
// fn theme_seed(theme: &Theme) -> (f64, f64) {
//     let palette = theme.palette();
//     // Hue from the theme's primary accent; fall back to blue if the accent is near-grey.
//     let (_, c, h) = color_to_oklch(palette.primary.base.color);
//     let h0 = if c > 0.02 { h } else { 264.0_f64.to_radians() };
//     // Dark background -> light series; light background -> darker series.
//     let (bg_l, _, _) = color_to_oklch(palette.background.base.color);
//     let l_base = lerp(0.82, 0.56, bg_l.clamp(0.0, 1.0));
//     (h0, l_base)
// }

// // --- OKLCH / sRGB conversions

// // sRGB → OKLCH `(L, C, h)` with `h` in radians.
// fn color_to_oklch(c: Color) -> (f64, f64, f64) {
//     let lin = [srgb_to_linear(c.r), srgb_to_linear(c.g), srgb_to_linear(c.b)];
//     let (l, a, b) = linear_to_oklab(lin);
//     (l, (a * a + b * b).sqrt(), b.atan2(a))
// }

// // OKLCH → `iced::Color`, reducing chroma as needed so the result lands inside the sRGB gamut
// // (keeps the requested lightness and hue exactly, only desaturating when unrepresentable).
// fn oklch_to_color(l: f64, c: f64, h: f64) -> Color {
//     let mut chroma = c;
//     if !in_gamut(l, chroma, h) {
//         let (mut lo, mut hi) = (0.0, c);
//         for _ in 0..24 {
//             let mid = 0.5 * (lo + hi);
//             if in_gamut(l, mid, h) {
//                 lo = mid;
//             } else {
//                 hi = mid;
//             }
//         }
//         chroma = lo;
//     }
//     let [r, g, b] = oklch_to_linear(l, chroma, h);
//     Color::from_rgb(
//         linear_to_srgb(r).clamp(0.0, 1.0),
//         linear_to_srgb(g).clamp(0.0, 1.0),
//         linear_to_srgb(b).clamp(0.0, 1.0),
//     )
// }

// fn in_gamut(l: f64, c: f64, h: f64) -> bool {
//     let [r, g, b] = oklch_to_linear(l, c, h);
//     let ok = |v: f32| v >= -1e-4 && v <= 1.0 + 1e-4;
//     ok(r) && ok(g) && ok(b)
// }

// fn oklch_to_linear(l: f64, c: f64, h: f64) -> [f32; 3] {
//     oklab_to_linear(l, c * h.cos(), c * h.sin())
// }

// fn srgb_to_linear(c: f32) -> f64 {
//     let c = c as f64;
//     if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
// }

// fn linear_to_srgb(c: f32) -> f32 {
//     let c = c as f64;
//     let v = if c <= 0.0031308 { 12.92 * c } else { 1.055 * c.powf(1.0 / 2.4) - 0.055 };
//     v as f32
// }

// fn linear_to_oklab([r, g, b]: [f64; 3]) -> (f64, f64, f64) {
//     let l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
//     let m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
//     let s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;
//     let (l_, m_, s_) = (l.cbrt(), m.cbrt(), s.cbrt());
//     (
//         0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
//         1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
//         0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
//     )
// }

// fn oklab_to_linear(l: f64, a: f64, b: f64) -> [f32; 3] {
//     let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
//     let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
//     let s_ = l - 0.0894841775 * a - 1.2914855480 * b;
//     let (l3, m3, s3) = (l_ * l_ * l_, m_ * m_ * m_, s_ * s_ * s_);
//     [
//         (4.0767416621 * l3 - 3.3077115913 * m3 + 0.2309699292 * s3) as f32,
//         (-1.2684380046 * l3 + 2.6097574011 * m3 - 0.3413193965 * s3) as f32,
//         (-0.0041960863 * l3 - 0.7034186147 * m3 + 1.7076147010 * s3) as f32,
//     ]
// }

// // Van der Corput (bit-reversal) sequence in base 2, giving an offset in `[0, 1)`.
// fn radical_inverse2(mut n: usize) -> f64 {
//     let mut inv = 0.0;
//     let mut base = 0.5;
//     while n > 0 {
//         inv += (n & 1) as f64 * base;
//         n >>= 1;
//         base *= 0.5;
//     }
//     inv
// }

// fn lerp(a: f64, b: f64, t: f64) -> f64 {
//     a + (b - a) * t
// }

pub fn series_color(theme: &iced::Theme) {



    // Test with a Dark Theme Background
    let dark_series = generate_series_colors("#1a1a24", 15);
    println!("Dark Theme Pack (15 colors):\n{:#?}", dark_series);

    // Test with a Light Theme Background
    let light_series = generate_series_colors("#fcfcfc", 15);
    println!("\nLight Theme Pack (15 colors):\n{:#?}", light_series);
}
fn generate_series_colors(bg_hex: &str, num_series: usize) -> Vec<String> {
    // 1. Parse background hex to RGB (0.0 to 1.0)
    let bg_hex = bg_hex.trim_start_matches('#');
    let bg_r = u8::from_str_radix(&bg_hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
    let bg_g = u8::from_str_radix(&bg_hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
    let bg_b = u8::from_str_radix(&bg_hex[4..6], 16).unwrap_or(0) as f32 / 255.0;

    // 2. Simple relative luminance calculation to determine if background is light or dark
    // Formula matches WCAG standards
    let bg_luminance = 0.2126 * bg_r + 0.7152 * bg_g + 0.0722 * bg_b;
    let is_dark_bg = bg_luminance < 0.25;

    let mut colors = Vec::with_capacity(num_series);
    
    // Golden ratio conjugate (~137.508 degrees) distributes hues maximally
    let golden_ratio_conjugate = 0.618033988749895;
    let mut hue = 0.35; // Starting hue seed

    for i in 0..num_series {
        // Increment hue using the golden ratio to spread them across the 0.0 - 1.0 wheel
        hue = (hue + golden_ratio_conjugate) % 1.0;
        let hue_degrees = hue * 360.0;

        // Interleave Lightness and Saturation so adjacent colors look completely different
        let (saturation, lightness) = if is_dark_bg {
            match i % 3 {
                0 => (0.95, 0.85), // Very light and vivid
                1 => (0.75, 0.65), // Medium-light, slightly muted
                _ => (0.90, 0.75), // Crisp pastel
            }
        } else {
            match i % 3 {
                0 => (0.95, 0.25), // Deep, dark and vivid
                1 => (0.75, 0.45), // Medium-dark, slightly muted
                _ => (0.85, 0.35), // Solid earth tone
            }
        };

        // Convert HSL back to an RGB Hex String
        colors.push(hsl_to_hex(hue_degrees, saturation, lightness));
    }

    colors
}

fn hsl_to_hex(h: f32, s: f32, l: f32) -> String {
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

    format!(
        "#{:02X}{:02X}{:02X}",
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8
    )
}


