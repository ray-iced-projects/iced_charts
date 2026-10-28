//! Chart configuration loading from YAML or JSON files.
use serde::{Deserialize, Serialize};
use crate::model::features::{ChartText, TextKind};
use crate::model::style::ChartTheme;
use crate::resolve::series_color;
use crate::{Chart, ChartData, ChartKind, ChartState, LegendPos, SeriesData};

/// Configuration for loading charts from YAML/JSON
#[derive(Debug, Serialize, Deserialize)]
pub struct ChartConfig {
    pub charts: Vec<ChartConfigItem>,
}

/// Individual chart configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct ChartConfigItem {
    pub id: usize,
    pub kind: ChartKind,
    pub title: Option<String>,
    pub x_title: Option<String>,
    pub y_title: Option<String>,
    #[serde(default)]
    pub legend: Option<LegendPos>,
    #[serde(default)]
    pub gridlines: Option<bool>,
    #[serde(default)]
    pub data_labels: Option<bool>,
    #[serde(default)]
    pub label_size: Option<f32>,
    pub series: Vec<SeriesConfig>,
}

/// Series configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct SeriesConfig {
    pub name: String,
    #[serde(default)]
    pub kind: Option<ChartKind>,
    #[serde(default)]
    pub x_values: Vec<f64>,
    pub y_values: Vec<f64>,
    #[serde(default)]
    pub sizes: Vec<f64>,
}

impl SeriesConfig {
    /// Build a `SeriesData` from this config, resolving colour from the theme.
    pub fn build_series(&self, index: usize, kind: ChartKind, theme: &ChartTheme) -> SeriesData {
        SeriesData {
            name: self.name.clone().into(),
            x: if self.x_values.is_empty() {
                None
            } else {
                Some(self.x_values.iter().map(|&v| Some(v)).collect())
            },
            values: self.y_values.iter().map(|&v| Some(v)).collect(),
            sizes: if self.sizes.is_empty() {
                None
            } else {
                Some(self.sizes.iter().map(|&v| Some(v)).collect())
            },
            kind: self.kind.unwrap_or(kind),
            color: series_color(theme, index),
            ..Default::default()
        }
    }
}

impl ChartConfigItem {
    /// Create a ChartText from the title field
    pub fn make_title(&self) -> ChartText {
        let mut text = ChartText::new(TextKind::Title);
        if let Some(ref t) = self.title {
            text.content = t.clone().into();
        }
        text
    }

    /// Create a ChartText from the x_title field
    pub fn make_x_title(&self) -> ChartText {
        let mut text = ChartText::new(TextKind::XTitle);
        if let Some(ref t) = self.x_title {
            text.content = t.clone().into();
        }
        text
    }

    /// Create a ChartText from the y_title field
    pub fn make_y_title(&self) -> ChartText {
        let mut text = ChartText::new(TextKind::YTitle);
        if let Some(ref t) = self.y_title {
            text.content = t.clone().into();
        }
        text
    }

    /// Build a fully-configured `Chart` (titles, legend, gridlines, labels) from this config.
    pub fn build_chart(&self) -> Chart {
        let mut chart = Chart::new(self.id, self.kind)
            .title(self.make_title())
            .x_title(self.make_x_title())
            .y_title(self.make_y_title())
            .legend(self.legend.unwrap_or(LegendPos::Bottom));

        if let Some(gridlines) = self.gridlines {
            chart = chart.gridlines(gridlines);
        }
        if let Some(data_labels) = self.data_labels {
            chart.data_labels = data_labels;
        }
        if let Some(label_size) = self.label_size {
            chart = chart.label_size(label_size);
        }
        chart
    }

    /// Build the `ChartData` (all series with theme-resolved colours) from this config.
    pub fn build_data(&self, theme: &ChartTheme) -> ChartData {
        let series = self
            .series
            .iter()
            .enumerate()
            .map(|(idx, s)| s.build_series(idx, self.kind, theme))
            .collect();

        ChartData {
            categories: vec![],
            series,
            palette: vec![],
        }
    }

    /// Build a ready-to-render `ChartState` (chart + data) from this config.
    pub fn build_state(&self, theme: &ChartTheme) -> ChartState {
        ChartState::new(self.build_chart(), self.build_data(theme))
    }
}

/// Load and deserialize chart configuration from YAML string
pub fn load_chart_from_yaml(content: &str) -> Result<ChartConfig, serde_yaml::Error> {
    serde_yaml::from_str(content)
}

/// Load and deserialize chart configuration from JSON string
pub fn load_chart_from_json(content: &str) -> Result<ChartConfig, serde_json::Error> {
    serde_json::from_str(content)
}
