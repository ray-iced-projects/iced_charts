

use std::path::{Path, PathBuf};

use iced::Theme;
use iced_charts::{ChartState, load_chart_from_yaml};
use native_dialog::{DialogBuilder, MessageLevel};
use rfd::AsyncFileDialog;





/// Pick a YAML config file and read its contents.
pub async fn pick_yaml_file(last_folder: Option<PathBuf>) -> Result<(PathBuf, String), String> {
    let mut dialog = AsyncFileDialog::new()
        .add_filter("YAML config", &["yml", "yaml"])
        .set_title("Open chart config");

    // Set initial directory if last folder is known
    if let Some(folder) = last_folder {
        dialog = dialog.set_directory(folder);
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

/// Parse a YAML config, resolve its data source relative to the config file, and build the chart.
pub fn build_grid_from_yaml(path: &Path, content: &str, theme: &Theme) -> Result<ChartState, String> {
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

    let item = config
        .charts
        .first()
        .ok_or_else(|| "Config contains no charts".to_string())?;
    item.build_state(theme)
        .map_err(|e| format!("Failed to load data: {e}"))
}

