//! Series colors and theme-based color palettes

use once_cell::sync::Lazy;
use iced::{Color, Theme, color};
use crate::model::style::apply_tint;

/// Cap on points per series (keeps whole-column references from exploding).
pub(crate) const MAX_POINTS: usize = 100_000;
/// Cap on series per chart.
pub(crate) const MAX_SERIES: usize = 255;
/// Cap on trendlines drawn per series.
pub(crate) const MAX_TRENDLINES: usize = 16;

/// Tints used for series beyond the 10 base colors: lighter, darker, lighter still…
const CYCLE_TINTS: [f64; 6] = [0.0, 0.4, -0.25, 0.6, -0.5, 0.2];

// ============================================================================
// Theme-specific 10-color palettes (from iced theme color schemes)
// ============================================================================

static CATPPUCCIN_FRAPPE: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xa6d189), color!(0xf4b8e4), color!(0xe78284), color!(0xe5c890),
        color!(0x81c8be), color!(0x8caaee), color!(0xca9ee6), color!(0xef9f76),
        color!(0x99d1db), color!(0x737994),
    ]);

static CATPPUCCIN_LATTE: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x40a02b), color!(0xea76cb), color!(0xd20f39), color!(0xdf8e1d),
        color!(0x179299), color!(0x1e66f5), color!(0x8839ef), color!(0xfe640b),
        color!(0x04a5e5), color!(0x4c4f69),
    ]);

static CATPPUCCIN_MACCHIATO: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xa6da95), color!(0xf5bde6), color!(0xed8796), color!(0xeed49f),
        color!(0x8bd5ca), color!(0x8aadf4), color!(0xc6a0f6), color!(0xf5a97f),
        color!(0x91d7e3), color!(0x6e738d),
    ]);
    
static CATPPUCCIN_MOCHA: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xa6e3a1), color!(0xf5c2e7), color!(0xf38ba8), color!(0xf9e2af),
        color!(0x94e2d5), color!(0x89b4fa), color!(0xcba6f7), color!(0xfab387),
        color!(0x89dceb), color!(0x6c7086),
    ]);

static DARK: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x5470c6), color!(0x91cc75), color!(0xfac858), color!(0xee6666),
        color!(0x73c0de), color!(0xfc8452), color!(0x9a60b4), color!(0xea7ccc),
        color!(0x3ba272), color!(0x66e6bf),
    ]);

static DRACULA: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xfc97af), color!(0x87f7cf), color!(0xf7f494), color!(0x72ccff),
        color!(0xd4a4eb), color!(0xd2f5a6), color!(0x76f2f2), color!(0xffb380),
        color!(0x99ff99), color!(0xe6b3ff),
    ]);

static FERRA: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xe06b75), color!(0xb1b695), color!(0xf5d76e), color!(0x8fb5d1),
        color!(0xfecdb2), color!(0xa18fc9), color!(0x6db5a6), color!(0xf6b6c9),
        color!(0xd1d1e0), color!(0xd4a373),
    ]);

static GRUVBOX_DARK: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x2662d9), color!(0xe23670), color!(0x2eb88a), color!(0xe88c30),
        color!(0xaf57db), color!(0x40f2a6), color!(0xd9d940), color!(0x40d9d9),
        color!(0xd94040), color!(0xbff259),
    ]);

static GRUVBOX_LIGHT: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xd87c7c), color!(0x61a0a8), color!(0xd7ab82), color!(0x6e7074),
        color!(0x919e8b), color!(0xcc7e63), color!(0x724e58), color!(0x4b565b),
        color!(0x8ca673), color!(0xbf8ca6),
    ]);

static KANAGAWA_DRAGON: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xe46876), color!(0x8a9a7b), color!(0xe6c384), color!(0x7fb4ca),
        color!(0x938aa9), color!(0x7aa89f), color!(0xb98d7b), color!(0xc5c9c5),
        color!(0xa6a69c), color!(0x8ba4b0),
    ]);
        
static KANAGAWA_LOTUS: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xc84053), color!(0x6f894e), color!(0x4d699b), color!(0xb35b79),
        color!(0x597b75), color!(0xd7474b), color!(0x6693bf), color!(0x836f4a),
        color!(0x624c83), color!(0x5e857a),
    ]);

static KANAGAWA_WAVE: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xe82424), color!(0x98bb6c), color!(0xe6c384), color!(0x7fb4ca),
        color!(0x957fb8), color!(0x6a9589), color!(0xc34043), color!(0x76946a),
        color!(0x938aa9), color!(0x7aa89f),
    ]);

static LIGHT: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x5470c6), color!(0x91cc75), color!(0xfac858), color!(0xee6666),
        color!(0xfc8452), color!(0x9a60b4), color!(0xea7ccc), color!(0x33bdd9),
        color!(0x3ba272), color!(0xd85454),
    ]);

static MOONFLY: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xff5d5d), color!(0x8cc85f), color!(0xe3c78a), color!(0x80a0ff),
        color!(0xcf87e8), color!(0x79dac8), color!(0xff5189), color!(0x36c692),
        color!(0x74b2ff), color!(0xae81ff),
    ]);

static NIGHTFLY: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x82aaff), color!(0x21c7a8), color!(0xae81ff), color!(0xff5874),
        color!(0xecc48d), color!(0x7fdbca), color!(0xa1cd5e), color!(0xc792ea),
        color!(0xfc514e), color!(0xe3d18a),
    ]);

static NORD: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x7eb26d), color!(0xeab839), color!(0x59b3bf), color!(0xbf1b00),
        color!(0x1f78c1), color!(0x705da0), color!(0xff6b9d), color!(0x4ecdc4),
        color!(0xff9f43), color!(0x508642),
    ]);

static OXOCARBON: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x08bdba), color!(0x525252), color!(0xee5396), color!(0xFF6F00),
        color!(0x0f62fe), color!(0x673AB7), color!(0x42be65), color!(0xbe95ff),
        color!(0x90A4AE), color!(0x37474F),
    ]);

static SOLARIZED_DARK: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xb58900), color!(0xcb4b16), color!(0xd33682), color!(0x6c71c4),
        color!(0x2aa198), color!(0x859900), color!(0x586e75), color!(0xeee8d5),
        color!(0xb58900), color!(0xcb4b16),
    ]);

static SOLARIZED_LIGHT: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x5b8ff9), color!(0x5ad8a6), color!(0xf6bd16), color!(0x6f5ef9),
        color!(0xff9845), color!(0x40bf73), color!(0xfa5973), color!(0x945fb9),
        color!(0x40d9bf), color!(0xd940a6),
    ]);

static TOKYO_NIGHT: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x2959aa), color!(0x979db8), color!(0xc2985b), color!(0x85353e),
        color!(0x8f5e15), color!(0xc1c2c7), color!(0x707280), color!(0x5a607d),
        color!(0xcdced1), color!(0x942f2f),
    ]);

static TOKYO_NIGHT_LIGHT: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x3fb1e3), color!(0x6be6c1), color!(0xa0a7e6), color!(0xd9bf73),
        color!(0xe68cbf), color!(0x73bf8c), color!(0xbf73e6), color!(0xe6738c),
        color!(0x626c91), color!(0xe6d973),
    ]);

static TOKYO_NIGHT_STORM: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x7e83b2), color!(0xa9b1d6), color!(0x6183bb), color!(0x3d59a1),
        color!(0xc0caf5), color!(0xc2985b), color!(0xe0af68), color!(0x963c47),
        color!(0x545c7e), color!(0x5a607d),
    ]);

// ============================================================================
// Palette resolution functions
// ============================================================================

/// Get the 10-color palette for a given iced Theme.
fn get_palette_for_theme(theme: &Theme) -> &'static Vec<Color> {
    match theme {
        Theme::Light => &LIGHT,
        Theme::Dark => &DARK,
        Theme::CatppuccinFrappe => &CATPPUCCIN_FRAPPE,
        Theme::CatppuccinLatte => &CATPPUCCIN_LATTE,
        Theme::CatppuccinMacchiato => &CATPPUCCIN_MACCHIATO,
        Theme::CatppuccinMocha => &CATPPUCCIN_MOCHA,
        Theme::Dracula => &DRACULA,
        Theme::Ferra => &FERRA,
        Theme::GruvboxDark => &GRUVBOX_DARK,
        Theme::GruvboxLight => &GRUVBOX_LIGHT,
        Theme::KanagawaDragon => &KANAGAWA_DRAGON,
        Theme::KanagawaLotus => &KANAGAWA_LOTUS,
        Theme::KanagawaWave => &KANAGAWA_WAVE,
        Theme::Moonfly => &MOONFLY,
        Theme::Nightfly => &NIGHTFLY,
        Theme::Nord => &NORD,
        Theme::Oxocarbon => &OXOCARBON,
        Theme::SolarizedDark => &SOLARIZED_DARK,
        Theme::SolarizedLight => &SOLARIZED_LIGHT,
        Theme::TokyoNight => &TOKYO_NIGHT,
        Theme::TokyoNightLight => &TOKYO_NIGHT_LIGHT,
        Theme::TokyoNightStorm => &TOKYO_NIGHT_STORM,
        Theme::Custom(_) => &DARK, // Fallback for custom themes
    }
}

/// Colour of the `i`-th series: first 10 from theme-specific palette, then tinted variations.
/// 
/// For series 0-9, returns the base palette color.
/// For series 10+, cycles through the palette with tint variations.
pub fn series_color(iced_theme: &Theme, i: usize) -> iced::Color {
    let palette = get_palette_for_theme(iced_theme);
    
    if i < 10 {
        // Use the base palette color directly
        palette[i]
    } else {
        // Cycle through palette with tint variations for indices 10+
        let palette_idx = (i - 10) % 10;
        let tint_idx = ((i - 10) / 10) % CYCLE_TINTS.len();
        let base_color = palette[palette_idx];
        
        // Extract RGB from the iced::Color
        let rgb = [
            (base_color.r * 255.0) as u8,
            (base_color.g * 255.0) as u8,
            (base_color.b * 255.0) as u8,
        ];
        
        let tint = CYCLE_TINTS.get(tint_idx).copied().unwrap_or(0.0);
        let [r, g, b] = apply_tint(rgb, tint);
        iced::Color::from_rgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
    }
}

/// The first `n` series colours for the given iced theme.
pub fn default_palette(iced_theme: &Theme, n: usize) -> Vec<iced::Color> {
    (0..n.min(4096)).map(|i| series_color(iced_theme, i)).collect()
}
