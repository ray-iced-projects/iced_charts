//! Chart configuration loading from YAML or JSON files.
use serde::{Deserialize, Serialize};
use iced::Theme;
use crate::model::features::{ChartText, TextKind, Trendline};
use crate::resolve::series_color;
use crate::csv_data::{ColumnRef, CsvError, CsvTable};
use crate::{Chart, ChartData, ChartKind, ChartState, LegendPos, SeriesData, TrendlineData};

/// Configuration for loading charts from YAML/JSON
#[derive(Debug, Serialize, Deserialize)]
pub struct ChartConfig {
    pub charts: Vec<ChartConfigItem>,
}

/// External CSV data source for a chart.
#[derive(Debug, Serialize, Deserialize)]
pub struct DataSource {
    /// Path to the CSV file (resolved relative to the process working directory).
    pub file: String,
    /// Column providing category labels (fills `ChartData.categories`).
    #[serde(default)]
    pub category_column: Option<ColumnRef>,
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
    /// Luminance threshold (0.0-1.0) for determining if background is dark (default: 0.25).
    /// Values < threshold are considered dark; >= threshold are considered light.
    #[serde(default = "default_dark_luminance_threshold")]
    pub dark_luminance_threshold: f32,
    /// Optional CSV source; series may pull columns from it instead of inline values.
    #[serde(default)]
    pub data_source: Option<DataSource>,
    pub series: Vec<SeriesConfig>,
}

fn default_dark_luminance_threshold() -> f32 {
    0.25
}

/// Series configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct SeriesConfig {
    pub name: String,
    #[serde(default)]
    pub kind: Option<ChartKind>,
    #[serde(default)]
    pub x_values: Vec<f64>,
    #[serde(default)]
    pub y_values: Vec<f64>,
    #[serde(default)]
    pub sizes: Vec<f64>,
    #[serde(default)]
    pub categories: Vec<String>,
    /// CSV column for X values (overrides `x_values` when a data source is present).
    #[serde(default)]
    pub x_column: Option<ColumnRef>,
    /// CSV column for Y values (overrides `y_values` when a data source is present).
    #[serde(default)]
    pub y_column: Option<ColumnRef>,
    /// CSV column for bubble sizes (overrides `sizes` when a data source is present).
    #[serde(default)]
    pub size_column: Option<ColumnRef>,
    /// Optional trendlines to display on this series.
    #[serde(default)]
    pub trendlines: Vec<Trendline>,
}

impl SeriesConfig {
    /// Build a `SeriesData` from this config, resolving colour from the iced theme.
    ///
    /// When `table` is present, `*_column` selectors take precedence over inline values.
    pub fn build_series(
        &self,
        index: usize,
        kind: ChartKind,
        iced_theme: &Theme,
        table: Option<&CsvTable>,
        dark_luminance_threshold: f32,
    ) -> Result<SeriesData, CsvError> {
        let values = match (&self.y_column, table) {
            (Some(col), Some(t)) => t.column_f64(col)?,
            _ => self.y_values.iter().map(|&v| Some(v)).collect(),
        };

        let x = match (&self.x_column, table) {
            (Some(col), Some(t)) => Some(t.column_f64(col)?),
            _ if !self.x_values.is_empty() => {
                Some(self.x_values.iter().map(|&v| Some(v)).collect())
            }
            _ => None,
        };

        let sizes = match (&self.size_column, table) {
            (Some(col), Some(t)) => Some(t.column_f64(col)?),
            _ if !self.sizes.is_empty() => {
                Some(self.sizes.iter().map(|&v| Some(v)).collect())
            }
            _ => None,
        };

        let trendlines = self.trendlines.iter()
            .map(|tl| TrendlineData {
                spec: tl.clone(),
                color: tl.color.unwrap_or_else(|| series_color(iced_theme, index, dark_luminance_threshold)),
            })
            .collect();

        Ok(SeriesData {
            name: self.name.clone().into(),
            x,
            values,
            sizes,
            kind: self.kind.unwrap_or(kind),
            color: series_color(iced_theme, index, dark_luminance_threshold),
            trendlines,
            ..Default::default()
        })
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
        chart.dark_luminance_threshold = self.dark_luminance_threshold;
        chart
    }

    /// Build the `ChartData` (all series with theme-resolved colours) from this config.
    ///
    /// Loads the CSV data source once (if any) and resolves category/column data.
    pub fn build_data(&self, iced_theme: &Theme) -> Result<ChartData, CsvError> {
        let table = match &self.data_source {
            Some(ds) => Some(CsvTable::load(&ds.file)?),
            None => None,
        };

        let categories = match (&self.data_source, table.as_ref()) {
            (Some(ds), Some(t)) => match &ds.category_column {
                Some(col) => t.column_strings(col)?,
                None => vec![],
            },
            _ => {
                // If no data source, use categories from the first series (if available)
                self.series.first()
                    .map(|s| s.categories.clone())
                    .unwrap_or_default()
            }
        };

        let mut series = Vec::with_capacity(self.series.len());
        for (idx, s) in self.series.iter().enumerate() {
            series.push(s.build_series(idx, self.kind, iced_theme, table.as_ref(), self.dark_luminance_threshold)?);
        }

        Ok(ChartData {
            categories,
            series,
            palette: vec![],
        })
    }

    /// Build a ready-to-render `ChartState` (chart + data) from this config.
    pub fn build_state(&self, iced_theme: &Theme) -> Result<ChartState, CsvError> {
        Ok(ChartState::new(self.build_chart(), self.build_data(iced_theme)?))
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
