
use std::path::{Path, PathBuf};

use iced::Theme;
use iced_charts::{ChartState, load_chart_from_yaml};
use native_dialog::{DialogBuilder, MessageLevel};
use rfd::AsyncFileDialog;

use crate::start_up_configs::StartupConfig;



/// Pick a YAML config file and read its contents.
pub async fn pick_yaml_file(last_folder: Option<PathBuf>) -> Result<(PathBuf, String), String> {
    let mut dialog = AsyncFileDialog::new()
        .add_filter("YAML config", &["yml", "yaml"])
        .set_title("Open chart config");

    // Set initial directory if the stored folder is still valid.
    // Some Linux dialog backends ignore or mishandle non-existent paths.
    if let Some(folder) = last_folder {
        let initial_dir = std::fs::canonicalize(&folder)
            .ok()
            .filter(|p| p.is_dir())
            .or_else(|| folder.is_dir().then_some(folder))
            .or_else(|| std::env::var_os("HOME").map(PathBuf::from));

        if let Some(dir) = initial_dir {
            dialog = dialog.set_directory(dir);
        }
    }

    let handle = dialog
        .pick_file()
        .await
        .ok_or_else(|| "cancelled".to_string())?;

    let path = handle.path().to_path_buf();
    let bytes = handle.read().await;
    let content = String::from_utf8(bytes).map_err(|e| e.to_string())?;
    Ok((path, content))
}

/// Show a native error dialog.
pub async fn show_error(message: String) {
    let _ = DialogBuilder::message()
        .set_level(MessageLevel::Error)
        .set_title("Chart load error")
        .set_text(message)
        .alert()
        .spawn()
        .await;
}

/// Parse a YAML config, resolve each data source relative to the config file, and build all charts.
pub fn build_charts_from_yaml(path: &Path, content: &str, theme: &Theme) -> Result<Vec<ChartState>, String> {
    let mut config =
        load_chart_from_yaml(content).map_err(|e| format!("Failed to parse YAML: {e}"))?;

    // Resolve relative data_source paths against the config file's directory.
    if let Some(base) = path.parent() {
        for chart in &mut config.charts {
            if let Some(ds) = &mut chart.data_source {
                let p = Path::new(&ds.file);
                if p.is_relative() {
                    ds.file = base.join(p).to_string_lossy().into_owned();
                }
            }
        }
    }

    if config.charts.is_empty() {
        return Err("Config contains no charts".to_string());
    }

    config
        .charts
        .into_iter()
        .map(|item| item.build_state(theme).map_err(|e| format!("Failed to load data: {e}")))
        .collect()
}

/// Backward-compatible helper for callers that still want the first chart only.
#[allow(dead_code)]
pub fn build_grid_from_yaml(path: &Path, content: &str, theme: &Theme) -> Result<ChartState, String> {
    build_charts_from_yaml(path, content, theme)?
        .into_iter()
        .next()
        .ok_or_else(|| "Config contains no charts".to_string())
}

pub struct LoadedChartFile {
    pub charts: Vec<ChartState>,
    pub file_path: PathBuf,
    pub file_content: String,
    pub recent_files: Vec<String>,
    pub recent_files_display: Vec<String>,
}

fn apply_canvas_dimensions(chart_state: &mut ChartState, width: f32, height: f32) {
    chart_state.apply_default_canvas_size(width, height);
}

fn display_names(paths: &[String]) -> Vec<String> {
    paths
        .iter()
        .map(|path| {
            std::path::Path::new(path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(path)
                .to_string()
        })
        .collect()
}

pub fn process_loaded_chart_file(path: PathBuf, content: String, theme: &Theme) -> Result<LoadedChartFile, String> {
    let mut charts = build_charts_from_yaml(&path, &content, theme)?;

    let mut startup_config = StartupConfig::load();
    for chart in &mut charts {
        apply_canvas_dimensions(chart, startup_config.canvas_width, startup_config.canvas_height);
        chart.request_redraw();
    }

    startup_config.add_recent_file(path.to_string_lossy().into_owned());
    startup_config.set_last_folder_from_path(&path);

    if let Err(e) = startup_config.save() {
        eprintln!("Failed to save config: {}", e);
    }

    let recent_files = startup_config.recent_files.clone();
    let recent_files_display = display_names(&recent_files);

    Ok(LoadedChartFile {
        charts,
        file_path: path,
        file_content: content,
        recent_files,
        recent_files_display,
    })
}

#[cfg(test)]
mod tests {
    use super::build_charts_from_yaml;
    use std::path::Path;
    use iced::Theme;

    #[test]
    fn build_charts_from_yaml_supports_multiple_chart_configs() {
        let yaml = r#"
charts:
  - id: 1
    kind: Line
    title: "First"
    series:
      - name: "A"
        y_values: [1.0, 2.0]
  - id: 2
    kind: ColumnClustered
    title: "Second"
    series:
      - name: "B"
        y_values: [3.0, 4.0]
"#;

        let charts = build_charts_from_yaml(Path::new("/tmp/example.yml"), yaml, &Theme::TokyoNight)
            .expect("multiple chart config should parse");

        assert_eq!(charts.len(), 2);
    }

    #[test]
    fn apply_canvas_dimensions_preserves_explicit_chart_size() {
        let yaml = r#"
charts:
  - id: 1
    kind: Line
    title: "Sized"
    width: 400.0
    height: 300.0
    series:
      - name: "A"
        y_values: [1.0, 2.0]
"#;

        let mut charts = build_charts_from_yaml(Path::new("/tmp/example.yml"), yaml, &Theme::TokyoNight)
            .expect("chart config should parse");

        super::apply_canvas_dimensions(&mut charts[0], 800.0, 600.0);

        let (width, height) = charts[0].canvas_size();
        assert_eq!(width, 400.0);
        assert_eq!(height, 300.0);
    }
}
