//! Theme selection utilities for UI (color palettes moved to src/resolve.rs)

use iced::Theme::{self, TokyoNight};


pub fn theme_strings() -> Vec<String> {
    vec![
        "Light".to_string(),
        "Dark".to_string(),
        "CatppuccinFrappe".to_string(),
        "CatppuccinLatte".to_string(),
        "CatppuccinMacchiato".to_string(),
        "CatppuccinMocha".to_string(),
        "Dracula".to_string(),
        "Ferra".to_string(),
        "GruvboxDark".to_string(),
        "GruvboxLight".to_string(),
        "KanagawaDragon".to_string(),
        "KanagawaLotus".to_string(),
        "KanagawaWave".to_string(),
        "Moonfly".to_string(),
        "Nightfly".to_string(),
        "Nord".to_string(),
        "Oxocarbon".to_string(),
        "SolarizedDark".to_string(),
        "SolarizedLight".to_string(),
        "TokyoNight".to_string(),
        "TokyoNightLight".to_string(),
        "TokyoNightStorm".to_string(),
    ]
}

/// Converts a combo box selection string back into a `Theme` variant.
pub fn str_to_theme(s: &str) -> Theme {
    match s {
        "Light" => Theme::Light,
        "Dark" => Theme::Dark,
        "CatppuccinFrappe" => Theme::CatppuccinFrappe,
        "CatppuccinLatte" => Theme::CatppuccinLatte,
        "CatppuccinMacchiato" => Theme::CatppuccinMacchiato,
        "CatppuccinMocha" => Theme::CatppuccinMocha,
        "Dracula" => Theme::Dracula,
        "Ferra" => Theme::Ferra,
        "GruvboxDark" => Theme::GruvboxDark,
        "GruvboxLight" => Theme::GruvboxLight,
        "KanagawaDragon" => Theme::KanagawaDragon,
        "KanagawaLotus" => Theme::KanagawaLotus,
        "KanagawaWave" => Theme::KanagawaWave,
        "Moonfly" => Theme::Moonfly,
        "Nightfly" => Theme::Nightfly,
        "Nord" => Theme::Nord,
        "Oxocarbon" => Theme::Oxocarbon,
        "SolarizedDark" => Theme::SolarizedDark,
        "SolarizedLight" => Theme::SolarizedLight,
        "TokyoNight" => Theme::TokyoNight,
        "TokyoNightLight" => Theme::TokyoNightLight,
        "TokyoNightStorm" => Theme::TokyoNightStorm,
        _ => TokyoNight,
    }
}

