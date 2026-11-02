
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

pub struct LoadedChartFile {
    pub grid: ChartState,
    pub file_path: PathBuf,
    pub file_content: String,
    pub recent_files: Vec<String>,
    pub recent_files_display: Vec<String>,
}

fn apply_canvas_dimensions(chart_state: &mut ChartState, width: f32, height: f32) {
    chart_state.set_width(width);
    chart_state.set_height(height);
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
    let mut grid = build_grid_from_yaml(&path, &content, theme)?;

    let mut startup_config = StartupConfig::load();
    apply_canvas_dimensions(&mut grid, startup_config.canvas_width, startup_config.canvas_height);
    grid.request_redraw();

    startup_config.add_recent_file(path.to_string_lossy().into_owned());
    startup_config.set_last_folder_from_path(&path);

    if let Err(e) = startup_config.save() {
        eprintln!("Failed to save config: {}", e);
    }

    let recent_files = startup_config.recent_files.clone();
    let recent_files_display = display_names(&recent_files);

    Ok(LoadedChartFile {
        grid,
        file_path: path,
        file_content: content,
        recent_files,
        recent_files_display,
    })
}
