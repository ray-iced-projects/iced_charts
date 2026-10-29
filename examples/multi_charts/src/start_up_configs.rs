
use std::{fs, path::{Path, PathBuf}};
use serde::{Deserialize, Serialize};




/// Startup configuration loaded from config.yml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupConfig {
    pub(crate) recent_files: Vec<String>,
    #[serde(default = "default_canvas_width")]
    pub(crate) canvas_width: f32,
    #[serde(default = "default_canvas_height")]
    pub(crate) canvas_height: f32,
    #[serde(default)]
    pub(crate) last_folder: Option<String>,
}

fn default_canvas_width() -> f32 {
    800.0
}

fn default_canvas_height() -> f32 {
    800.0
}

impl Default for StartupConfig {
    fn default() -> Self {
        Self {
            recent_files: vec![],
            canvas_width: 800.0,
            canvas_height: 800.0,
            last_folder: None,
        }
    }
}

impl StartupConfig {
    /// Load config from the example's config.yml file
    pub fn load() -> Self {
        let config_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("examples/multi_charts/config.yml");

        if config_path.exists() {
            if let Ok(content) = fs::read_to_string(&config_path) {
                if let Ok(config) = serde_yaml::from_str::<StartupConfig>(&content) {
                    return config;
                }
            }
        }
        StartupConfig::default()
    }

    /// Save config to config.yml
    pub fn save(&self) -> std::io::Result<()> {
        let config_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("examples/multi_charts/config.yml");

        let yaml = serde_yaml::to_string(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
        fs::write(config_path, yaml)?;
        Ok(())
    }

    /// Add a file to recent_files, keeping only the last 10 with most recent first
    pub fn add_recent_file(&mut self, path: String) {
        self.recent_files.retain(|f| f != &path);
        self.recent_files.insert(0, path);
        self.recent_files.truncate(10);
    }

    /// Set the last used folder from a file path
    pub fn set_last_folder_from_path(&mut self, path: &Path) {
        if let Some(parent) = path.parent() {
            self.last_folder = Some(parent.to_string_lossy().into_owned());
        }
    }

    /// Get the last used folder as a PathBuf, or None if not set
    pub fn get_last_folder(&self) -> Option<PathBuf> {
        self.last_folder.as_ref().map(PathBuf::from)
    }
}
