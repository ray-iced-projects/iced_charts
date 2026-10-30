//! Series colors and theme-based color palettes

use once_cell::sync::Lazy;
use iced::{Color, Theme, color};

/// Cap on points per series (keeps whole-column references from exploding).
pub(crate) const MAX_POINTS: usize = 100_000;
/// Cap on series per chart.
pub(crate) const MAX_SERIES: usize = 255;
/// Cap on trendlines drawn per series.
pub(crate) const MAX_TRENDLINES: usize = 16;

/// Tints used for series beyond the 10 base colors: lighter, darker, lighter still…
const CYCLE_TINTS: [f64; 6] = [0.0, 0.4, -0.25, 0.6, -0.5, 0.2];



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

/// Tint as in ECMA-376: lighten towards white (positive) or darken (negative) in HSL luminance.
fn apply_tint(rgb: [u8; 3], tint: f64) -> [u8; 3] {
    if tint == 0.0 {
        return rgb;
    }
    let (h, s, l) = rgb_to_hsl(rgb);
    let l = if tint < 0.0 { l * (1.0 + tint) } else { l * (1.0 - tint) + tint };
    hsl_to_rgb(h, s, l.clamp(0.0, 1.0))
}

fn rgb_to_hsl([r, g, b]: [u8; 3]) -> (f64, f64, f64) {
    let (r, g, b) = (r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    if max == min {
        return (0.0, 0.0, l);
    }
    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
    let h = if max == r {
        (g - b) / d + if g < b { 6.0 } else { 0.0 }
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    (h / 6.0, s, l)
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> [u8; 3] {
    if s == 0.0 {
        let v = (l * 255.0).round() as u8;
        return [v, v, v];
    }
    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    let hue = |mut t: f64| {
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }
        if t < 1.0 / 6.0 {
            p + (q - p) * 6.0 * t
        } else if t < 0.5 {
            q
        } else if t < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - t) * 6.0
        } else {
            p
        }
    };
    [(hue(h + 1.0 / 3.0) * 255.0).round() as u8, (hue(h) * 255.0).round() as u8, (hue(h - 1.0 / 3.0) * 255.0).round() as u8]
}


// Theme-specific 10-color palettes (from iced theme color schemes)
// I'm not the best at color picking and have extra ones if you want to
// change.  If you find a better selection. Let me know.
static CATPPUCCIN_FRAPPE: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xa6d189),       // #a6d189 - Green
        color!(0xf4b8e4),       // #f4b8e4 - Pink
        color!(0xe78284),       // #e78284 - Red
        color!(0xe5c890),       // #e5c890 - Yellow
        color!(0x81c8be),       // #81c8be - Teal
        color!(0x8caaee),       // #8caaee - Blue
        color!(0xca9ee6),       // #ca9ee6 - Mauve/Purple
        color!(0xef9f76),       // #ef9f76 - Peach
        color!(0x99d1db),       // #99d1db - Sky
        color!(0x737994),       // #737994 - Gray
        // color!(0xeebebe),       // #eebebe - Peach/Pink
        // color!(0xc6d0f5),       // #c6d0f5 - Light text
        // color!(0xf2d5cf),       // #f2d5cf - Light pink
        // color!(0xa5adce),       // #a5adce - Lavender
        // color!(0x85c1dc),       // #85c1dc - Sapphire
        // color!(0xea999c),       // #ea999c - Maroon
    ]);

static CATPPUCCIN_LATTE: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x40a02b), 	    // #40a02b - Green
        color!(0xea76cb), 	    // #ea76cb - Pink
        color!(0xd20f39), 	    // #d20f39 - Red
        color!(0xdf8e1d), 	    // #df8e1d - Yellow
        color!(0x179299), 	    // #179299 - Teal
        color!(0x1e66f5), 	    // #1e66f5 - Blue
        color!(0x8839ef), 	    // #8839ef - Mauve/Purple
        color!(0xfe640b), 	    // #fe640b - Peach
        color!(0x04a5e5), 	    // #04a5e5 - Sky
        color!(0x4c4f69), 	    // #4c4f69 - Dark
        // color!(0x9ca0b0), 	    // #9ca0b0 - Gray
        // color!(0xdd7878), 	    // #dd7878 - Rosewater/Pink
        // color!(0xdc8a78), 	    // #dc8a78 - Flamingo
        // color!(0x6c6f85), 	    // #6c6f85 - Light Gray
        // color!(0x209fb5), 	    // #209fb5 - Sapphire
        // color!(0xe64553), 	    // #e64553 - Maroon
    ]);

static CATPPUCCIN_MACCHIATO: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xa6da95), 	    // #a6da95 - Green
        color!(0xf5bde6), 	    // #f5bde6 - Pink
        color!(0xed8796), 	    // #ed8796 - Red
        color!(0xeed49f), 	    // #eed49f - Yellow
        color!(0x8bd5ca), 	    // #8bd5ca - Teal
        color!(0x8aadf4), 	    // #8aadf4 - Blue
        color!(0xc6a0f6), 	    // #c6a0f6 - Mauve (purple)
        color!(0xf5a97f), 	    // #f5a97f - Peach
        color!(0x91d7e3), 	    // #91d7e3 - Sky
        color!(0x6e738d), 	    // #6e738d - Gray
        // color!(0xf0c6c6), 	    // #f0c6c6 - Rosewater
        // color!(0xcad3f5), 	    // #cad3f5 - Light Blue
        // color!(0xf4dbd6), 	    // #f4dbd6 - Flamingo
        // color!(0xa5adcb), 	    // #a5adcb - Lavender
        // color!(0x7dc4e4), 	    // #7dc4e4 - Sapphire
        // color!(0xee99a0), 	    // #ee99a0 - Maroon
    ]);
    
static CATPPUCCIN_MOCHA: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xa6e3a1), 	    // #a6e3a1 - Green
        color!(0xf5c2e7), 	    // #f5c2e7 - Pink
        color!(0xf38ba8), 	    // #f38ba8 - Red
        color!(0xf9e2af), 	    // #f9e2af - Yellow
        color!(0x94e2d5), 	    // #94e2d5 - Teal
        color!(0x89b4fa), 	    // #89b4fa - Blue
        color!(0xcba6f7), 	    // #cba6f7 - Mauve (purple)
        color!(0xfab387), 	    // #fab387 - Peach
        color!(0x89dceb), 	    // #89dceb - Sky
        color!(0x6c7086), 	    // #6c7086 - Gray
        // color!(0xf2cdcd), 	    // #f2cdcd - Rosewater
        // color!(0xcdd6f4), 	    // #cdd6f4 - Light Blue
        // color!(0xf5e0dc), 	    // #f5e0dc - Flamingo
        // color!(0xa6adc8), 	    // #a6adc8 - Lavender
        // color!(0x74c7ec), 	    // #74c7ec - Sapphire
        // color!(0xeba0ac), 	    // #eba0ac - Maroon
    ]);

static DARK: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x5470c6),       // #5470c6 - Blue
        color!(0x91cc75),       // #91cc75 - Green
        color!(0xfac858),       // #fac858 - Yellow
        color!(0xee6666),       // #ee6666 - Red
        color!(0x73c0de),       // #73c0de - Light Blue
        color!(0xfc8452),       // #fc8452 - Orange
        color!(0x9a60b4),       // #9a60b4 - Purple
        color!(0xea7ccc),       // #ea7ccc - Pink
        color!(0x3ba272),       // #3ba272 - Dark Green
        color!(0x66e6bf),       // #66e6bf - Mint
        // color!(0x66a6e6),       // #66a6e6 - Light Blue
        // color!(0xe68c66),       // #e68c66 - Peach
        // color!(0xa666e6),       // #a666e6 - Lavender
        // color!(0xe6bf66),       // #e6bf66 - Gold
        // color!(0xe69966),       // #e69966 - Bronze
        // color!(0xbf66e6),       // #bf66e6 - Violet
        // color!(0x66e699),       // #66e699 - Light Green
        // color!(0xe666bf),       // #e666bf - Magenta
        // color!(0x99e666),       // #99e666 - Lime
        // color!(0x6699e6),       // #6699e6 - Royal Blue
    ]);

static DRACULA: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xfc97af),       // #fc97af - Pink
        color!(0x87f7cf),       // #87f7cf - Mint
        color!(0xf7f494),       // #f7f494 - Light Yellow
        color!(0x72ccff),       // #72ccff - Sky Blue
        color!(0xd4a4eb),       // #d4a4eb - Lavender
        color!(0xd2f5a6),       // #d2f5a6 - Light Green
        color!(0x76f2f2),       // #76f2f2 - Cyan
        color!(0xffb380),       // #ffb380 - Light Orange
        color!(0x99ff99),       // #99ff99 - Bright Green
        color!(0xe6b3ff),       // #e6b3ff - Light Purple
        // color!(0xf7c5a0),       // #f7c5a0 - Peach
        // color!(0xffcce6),       // #ffcce6 - Light Pink
        // color!(0xcce6ff),       // #cce6ff - Pale Blue
        // color!(0xffe6b3),       // #ffe6b3 - Cream
        // color!(0xb3ffe6),       // #b3ffe6 - Pale Mint
        // color!(0xe6ffcc),       // #e6ffcc - Pale Yellow
        // color!(0xffd9d9),       // #ffd9d9 - Blush
        // color!(0xd9d9ff),       // #d9d9ff - Periwinkle
        // color!(0xd9ffd9),       // #d9ffd9 - Pale Green
        // color!(0xffe6d9),       // #ffe6d9 - Apricot
    ]);

static FERRA: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xe06b75),	    // #e06b75 - Ember
        color!(0xb1b695),	    // #b1b695 - Sage
        color!(0xf5d76e),	    // #f5d76e - Honey
        color!(0x8fb5d1),	    // #8fb5d1 - Soft Blue
        color!(0xfecdb2),	    // #fecdb2 - Blush
        color!(0xa18fc9),	    // #a18fc9 - Lavender
        color!(0x6db5a6),	    // #6db5a6 - Muted Teal
        color!(0xf6b6c9),	    // #f6b6c9 - Rose
        color!(0xd1d1e0),	    // #d1d1e0 - Mist
        color!(0xd4a373),	    // #d4a373 - Warm Tan
        // color!(0xffa07a),	    // #ffa07a - Coral
        // color!(0xc98ca6),	    // #c98ca6 - Dusty Rose
        // color!(0xe8a87c),	    // #e8a87c - Light Coral
        // color!(0xe5b3c4),	    // #e5b3c4 - Blush Pink
        // color!(0xb8a3d1),	    // #b8a3d1 - Light Purple
        // color!(0xd4c07a),	    // #d4c07a - Golden Wheat
        // color!(0x9eb5a8),	    // #9eb5a8 - Dusty Sage
        // color!(0xe39b8f),	    // #e39b8f - Salmon
        // color!(0xa3c4d1),	    // #a3c4d1 - Sky Blue
        // color!(0xd1a38f),	    // #d1a38f - Warm Beige
    ]);

static GRUVBOX_DARK: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x2662d9),       // #2662d9 - Blue
        color!(0xe23670),       // #e23670 - Pink
        color!(0x2eb88a),       // #2eb88a - Green
        color!(0xe88c30),       // #e88c30 - Orange
        color!(0xaf57db),       // #af57db - Purple
        color!(0x40f2a6),       // #40f2a6 - Mint
        color!(0xd9d940),       // #d9d940 - Yellow
        color!(0x40d9d9),       // #40d9d9 - Cyan
        color!(0xd94040),       // #d94040 - Red
        color!(0xbff259),       // #bff259 - Lime
        // color!(0x3b86f7),       // #3b86f7 - Light Blue
        // color!(0xf17e92),       // #f17e92 - Coral
        // color!(0xf2a640),       // #f2a640 - Gold
        // color!(0xa640f2),       // #a640f2 - Violet
        // color!(0xf240a6),       // #f240a6 - Magenta
        // color!(0x40a6f2),       // #40a6f2 - Sky Blue
        // color!(0xf259bf),       // #f259bf - Rose
        // color!(0x59bff2),       // #59bff2 - Light Blue
        // color!(0xbf59f2),       // #bf59f2 - Lavender
    ]);

static GRUVBOX_LIGHT: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xd87c7c),       // #d87c7c - Dusty Rose
        color!(0x61a0a8),       // #61a0a8 - Muted Teal
        color!(0xd7ab82),       // #d7ab82 - Warm Tan
        color!(0x6e7074),       // #6e7074 - Charcoal
        color!(0x919e8b),       // #919e8b - Sage Green
        color!(0xcc7e63),       // #cc7e63 - Terracotta
        color!(0x724e58),       // #724e58 - Mauve
        color!(0x4b565b),       // #4b565b - Dark Slate
        color!(0x8ca673),       // #8ca673 - Moss Green
        color!(0xbf8ca6),       // #bf8ca6 - Dusty Pink
        // color!(0xefa18d),       // #efa18d - Soft Coral
        // color!(0x787464),       // #787464 - Olive Brown
        // color!(0xa67359),       // #a67359 - Burnt Sienna
        // color!(0x738ca6),       // #738ca6 - Steel Blue
        // color!(0xa69973),       // #a69973 - Khaki
        // color!(0x8c73a6),       // #8c73a6 - Plum
        // color!(0xb3a68c),       // #b3a68c - Beige
        // color!(0x73a68c),       // #73a68c - Seafoam
        // color!(0xa68c73),       // #a68c73 - Coffee
        // color!(0x99808c),       // #99808c - Taupe
    ]);

static KANAGAWA_DRAGON: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xe46876),	    // #e46876 - red 
        color!(0x8a9a7b),	    // #8a9a7b - green 
        color!(0xe6c384),	    // #e6c384 - yellow 
        color!(0x7fb4ca),	    // #7fb4ca - blue 
        color!(0x938aa9),	    // #938aa9 - magenta 
        color!(0x7aa89f),	    // #7aa89f - cyan 
        color!(0xb98d7b),	    // #b98d7b - Orange
        color!(0xc5c9c5),	    // #c5c9c5 - white 
        color!(0xa6a69c),	    // #a6a69c - black 
        color!(0x8ba4b0),	    // #8ba4b0 - blue 
        // color!(0xc4746e),	    // #c4746e - red 
        // color!(0xc4b28a),	    // #c4b28a - yellow 
        // color!(0xa292a3),	    // #a292a3 - magenta 
        // color!(0x8ea4a2),	    // #8ea4a2 - cyan 
        // color!(0xc8c093),	    // #c8c093 - white 
        // color!(0x87a987),	    // #87a987 - green 
        // color!(0x8992a7),	    // #8992a7 - Violet
        // color!(0xa8a48d),	    // #a8a48d - Ash
        // color!(0x9cabca),	    // #9cabca - Blue 2
        // color!(0x957f5f),	    // #957f5f - Brown
    ]);
        
static KANAGAWA_LOTUS: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![ 
        color!(0xc84053),	    // #c84053 - Red 
        color!(0x6f894e),	    // #6f894e - Green 
        color!(0x4d699b),	    // #4d699b - blue 
        color!(0xb35b79),	    // #b35b79 - magenta 
        color!(0x597b75),	    // #597b75 - cyan 
        color!(0xd7474b),	    // #f00e16 - Dark Red 
        color!(0x6693bf),	    // #6693bf - Blue 
        color!(0x836f4a),	    // #9fc27f - Light Yellow 
        color!(0x624c83),	    // #624c83 - Magenta 
        color!(0x5e857a),	    // #5e857a - Cyan 
        // color!(0x77713f),	    // #a39833 - Yellow-Green
        // color!(0x545464),	    // #545464 - Light Blue 
        // color!(0x8a8980),	    // #8a8980 - Grey 
        // color!(0x6e915f),	    // #1b5503 - Green 
        // color!(0x43436c),	    // #43436c - Blue 
    ]);

static KANAGAWA_WAVE: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xe82424),	    // #e82424 - Red 
        color!(0x98bb6c),	    // #98bb6c - Green 
        color!(0xe6c384),	    // #e6c384 - Yellow 
        color!(0x7fb4ca),	    // #7fb4ca - Blue 
        color!(0x957fb8),	    // #957fb8 - Magenta 
        color!(0x6a9589),	    // #6a9589 - Cyan 
        color!(0xc34043),	    // #c34043 - Red 
        color!(0x76946a),	    // #76946a - Green 
        color!(0x938aa9),	    // #938aa9 - Magenta 
        color!(0x7aa89f),	    // #7aa89f - Cyan 
        // color!(0xc0a36e),	    // #c0a36e - Yellow 
        // color!(0x7e9cd8),	    // #7e9cd8 - Blue 
        // color!(0xc8c093),	    // #c8c093 - White 
        // color!(0x727169),	    // #727169 - Black 
        // color!(0xdcd7ba),	    // #dcd7ba - White 
    ]);

static LIGHT: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x5470c6),       // #5470c6 Blue
        color!(0x91cc75),       // #91cc75 Green
        color!(0xfac858),       // #fac858 Yellow
        color!(0xee6666),       // #ee6666 Red
        color!(0xfc8452),       // #fc8452 Orange
        color!(0x9a60b4),       // #9a60b4 Purple
        color!(0xea7ccc),       // #ea7ccc Pink
        color!(0x33bdd9),       // #33bdd9 Cyan
        color!(0x3ba272),       // #3ba272 Dark Green
        color!(0xd85454),       // #d85454 Dark Red
        // color!(0x73c0de),       // #73c0de Light Blue
        // color!(0x7354d9),       // #7354d9 Indigo
        // color!(0xd9a633),       // #d9a633 Gold
        // color!(0x54d9a6),       // #54d9a6 Mint
        // color!(0xd97333),       // #d97333 Brown
        // color!(0xa633d9),       // #a633d9 Violet
        // color!(0x33d973),       // #33d973 Lime
        // color!(0xd933a6),       // #d933a6 Magenta
        // color!(0x73d933),       // #73d933 Chartreuse
        // color!(0x3373d9),       // #3373d9 Royal Blue
    ]);

static MOONFLY: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xff5d5d),	    // #ff5d5d
        color!(0x8cc85f),	    // #8cc85f
        color!(0xe3c78a),	    // #e3c78a
        color!(0x80a0ff),	    // #80a0ff
        color!(0xcf87e8),	    // #cf87e8
        color!(0x79dac8),	    // #79dac8
        color!(0xff5189),	    // #ff5189
        color!(0x36c692),	    // #36c692
        color!(0x74b2ff),	    // #74b2ff
        color!(0xae81ff),	    // #ae81ff
        // color!(0x323437),	    // #323437
        // color!(0xc6c6c6),	    // #c6c6c6
        // color!(0x949494),	    // #949494
        // color!(0xc6c684),	    // #c6c684
        // color!(0x85dc85),	    // #85dc85
        // color!(0xe4e4e4),	    // #e4e4e4
    ]);

static NIGHTFLY: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x82aaff),	    // #82aaff - blue 
        color!(0x21c7a8),	    // #21c7a8 - green 
        color!(0xae81ff),	    // #ae81ff - magenta 
        color!(0xff5874),	    // #ff5874 - red 
        color!(0xecc48d),	    // #ecc48d - yellow 
        color!(0x7fdbca),	    // #7fdbca - cyan 
        color!(0xa1cd5e),	    // #a1cd5e - green 
        color!(0xc792ea),	    // #c792ea - magenta 
        color!(0xfc514e),	    // #fc514e - red 
        color!(0xe3d18a),	    // #e3d18a - yellow 
        // color!(0xd6deeb),	    // #d6deeb - white 
        // color!(0xa1aab8),	    // #a1aab8 - white 
    ]);

static NORD: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x7eb26d),       // #7eb26d Green
        color!(0xeab839),       // #eab839 Yellow/Gold
        color!(0x59b3bf),       // #59b3bf Cyan
        color!(0xbf1b00),       // #bf1b00 Red
        color!(0x1f78c1),       // #1f78c1 Blue
        color!(0x705da0),       // #705da0 Purple
        color!(0xff6b9d),       // #ff6b9d Pink
        color!(0x4ecdc4),       // #4ecdc4 Teal
        color!(0xff9f43),       // #ff9f43 Orange
        color!(0x508642),       // #508642 Dark Green
        // color!(0xe5ac0e),       // #e5ac0e Orange
        // color!(0xffe66d),       // #ffe66d Light Yellow
        // color!(0xa8e6cf),       // #a8e6cf Light Green
        // color!(0xff8b94),       // #ff8b94 Coral
        // color!(0xb4a7d6),       // #b4a7d6 Lavender
        // color!(0xffd93d),       // #ffd93d Golden Yellow
        // color!(0x6bcf7f),       // #6bcf7f Mint Green
        // color!(0x74b9ff),       // #74b9ff Sky Blue
        // color!(0xfd79a8),       // #fd79a8 Rose
        // color!(0xfdcb6e),       // #fdcb6e Peach
    ]);

static OXOCARBON: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x08bdba),	    // #08bdba
        color!(0x525252),	    // #808080 
        color!(0xee5396),	    // #ee5396
        color!(0xFF6F00),	    // #FF6F00
        color!(0x0f62fe),	    // #0f62fe
        color!(0x673AB7),	    // #673AB7
        color!(0x42be65),	    // #42be65
        color!(0xbe95ff),	    // #be95ff
        color!(0x90A4AE),	    // #90A4AE
        color!(0x37474F),	    // #466c80
        //color!(0xff7eb6),	    // #ff7eb6
    ]);

static SOLARIZED_DARK: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0xb58900),	        // #b58900
        color!(0xcb4b16),	        // #cb4b16
        color!(0xd33682),	        // #d33682
        color!(0x6c71c4),	        // #6c71c4
        color!(0x2aa198),	        // #2aa198
        color!(0x859900),	        // #859900
        color!(0x586e75),	        // #586e75
        color!(0xeee8d5),	        // #eee8d5
    ]);

static SOLARIZED_LIGHT: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x5b8ff9),       // #5b8ff9 Blue
        color!(0x5ad8a6),       // #5ad8a6 Green
        color!(0xf6bd16),       // #f6bd16 Yellow
        color!(0x6f5ef9),       // #6f5ef9 Purple
        color!(0xff9845),       // #ff9845 Orange
        color!(0x40bf73),       // #40bf73 Forest Green
        color!(0xfa5973),       // #fa5973 Red
        color!(0x945fb9),       // #945fb9 Violet
        color!(0x40d9bf),       // #40d9bf Turquoise
        color!(0xd940a6),       // #d940a6 Magenta
        // color!(0x5d7092),       // #5d7092 Gray Blue
        // color!(0x6dc8ec),       // #6dc8ec Light Blue
        // color!(0x7340bf),       // #7340bf Indigo
        // color!(0xd9a640),       // #d9a640 Gold
        // color!(0xbf7340),       // #bf7340 Brown
        // color!(0xa640d9),       // #a640d9 Purple
        // color!(0x40a6d9),       // #40a6d9 Sky Blue
        // color!(0x8cd940),       // #8cd940 Lime
        // color!(0xd98c40),       // #d98c40 Orange Brown
        // color!(0x408cd9),       // #408cd9 Royal Blue
    ]);

static TOKYO_NIGHT: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x2959aa),	    // #2959aa
        color!(0x979db8),	    // #979db8
        color!(0xc2985b),	    // #c2985b
        color!(0x85353e),	    // #85353e
        color!(0x8f5e15),	    // #8f5e15
        color!(0xc1c2c7),	    // #c1c2c7
        color!(0x707280),	    // #707280
        color!(0x5a607d),	    // #5a607d
        color!(0xcdced1),	    // #cdced1
        color!(0x942f2f),	    // #942f2f
        // color!(0x363c4d),	    // #363c4d
        // color!(0x4a5272),	    // #4a5272
        // color!(0xe8e9ed),	    // #e8e9ed
        // color!(0x3d59a1),	    // #3d59a1
    ]);

static TOKYO_NIGHT_LIGHT: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x3fb1e3),       // #246e8d Blue
        color!(0x6be6c1),       // #22473d Green
        color!(0xa0a7e6),       // #a0a7e6 Lavender
        color!(0xd9bf73),       // #d9bf73 Gold
        color!(0xe68cbf),       // #e68cbf Pink
        color!(0x73bf8c),       // #73bf8c Sea Green
        color!(0xbf73e6),       // #bf73e6 Purple
        color!(0xe6738c),       // #e6738c Coral
        color!(0x626c91),       // #626c91 Blue Gray
        color!(0xe6d973),       // #e6d973 Yellow
        // color!(0xc4ebad),       // #c4ebad Light Green
        // color!(0x96dee8),       // #96dee8 Cyan
        // color!(0x8ce673),       // #8ce673 Lime
        // color!(0x738ce6),       // #738ce6 Blue
        // color!(0xbfe68c),       // #bfe68c Light Lime
        // color!(0x8cbfe6),       // #8cbfe6 Light Blue
        // color!(0xe6bf8c),       // #e6bf8c Peach
        // color!(0xa673d9),       // #a673d9 Violet
        // color!(0xd9a673),       // #d9a673 Tan
        // color!(0x73d9a6),       // #73d9a6 Teal
    ]);

static TOKYO_NIGHT_STORM: Lazy<Vec<Color>> = Lazy::new(|| 
    vec![
        color!(0x7e83b2),       // #7e83b2
        color!(0xa9b1d6),	    // #a9b1d6
        color!(0x6183bb),	    // #6183bb
        color!(0x3d59a1),       // #3d59a1
        color!(0xc0caf5),	    // #c0caf5
        color!(0xc2985b),	    // #c2985b
        color!(0xe0af68),	    // #e0af68
        color!(0x963c47),	    // #963c47
        color!(0x545c7e),       // #545c7e
        color!(0x5a607d),	    // #5a607d
        // color!(0x8089b3),	    // #8089b3
        // color!(0x41496b),	    // #41496b
        // color!(0x4a5272),	    // #4a5272
        // color!(0xbbc2e0),	    // #bbc2e0
        // color!(0x85353e),	    // #85353e
    ]);
