use iced::widget::{container, column};
use iced::{Task, Theme, Widget, window};
use iced_charts::{Chart, ChartData, ChartKind, ChartState};
use iced_charts::model::theme_helpers::{theme_strings, str_to_theme};
use iced_charts::config::load_chart_from_yaml;

use iced_menu::{Menu, MenuMessage};

use rfd::AsyncFileDialog;
use native_dialog::{DialogBuilder, MessageLevel};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};


pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .theme(|ex: &Example| ex.theme())
        .run()
}

/// Startup configuration loaded from config.yml
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StartupConfig {
    recent_files: Vec<String>,
    #[serde(default = "default_canvas_width")]
    canvas_width: f32,
    #[serde(default = "default_canvas_height")]
    canvas_height: f32,
    #[serde(default)]
    last_folder: Option<String>,
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
    fn load() -> Self {
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
    fn save(&self) -> std::io::Result<()> {
        let config_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("examples/multi_charts/config.yml");

        let yaml = serde_yaml::to_string(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
        fs::write(config_path, yaml)?;
        Ok(())
    }

    /// Add a file to recent_files, keeping only the last 10 with most recent first
    fn add_recent_file(&mut self, path: String) {
        self.recent_files.retain(|f| f != &path);
        self.recent_files.insert(0, path);
        self.recent_files.truncate(10);
    }

    /// Set the last used folder from a file path
    fn set_last_folder_from_path(&mut self, path: &Path) {
        if let Some(parent) = path.parent() {
            self.last_folder = Some(parent.to_string_lossy().into_owned());
        }
    }

    /// Get the last used folder as a PathBuf, or None if not set
    fn get_last_folder(&self) -> Option<PathBuf> {
        self.last_folder.as_ref().map(PathBuf::from)
    }
}

#[derive(Debug, Clone)]
struct MenuState {
    font_size: f32,
    recent_files: Vec<String>,
}

impl Default for MenuState {
    fn default() -> Self {
        let startup_config = StartupConfig::load();
        let size = 14.0;
        Self {
            font_size: size,
            recent_files: startup_config.recent_files,
        }
    }
}

struct Example {
    grid: ChartState,
    selected_theme: Theme,
    menu: Menu<'static, MenuAction>,
    menu_state: MenuState,
    file_path: PathBuf,
    file_content: String,
    is_loading: bool,
}

impl Default for Example {
    fn default() -> Self {
        let startup_config = StartupConfig::load();
        let menu_state = MenuState::default();

        let configs = define_menu_configs(&menu_state);
        let len = configs.len();
        let mut menu = Menu::default();
        menu.configs = configs;
        menu.is_open = vec![false; len];

        // Try to load the most recent file, or create an empty chart state
        let (mut grid, file_path, file_content) = if let Some(recent_path) = startup_config.recent_files.first() {
            match fs::read_to_string(recent_path) {
                Ok(content) => {
                    match build_grid_from_yaml(Path::new(recent_path), &content, &Theme::TokyoNight) {
                        Ok(grid) => (grid, PathBuf::from(recent_path), content),
                        Err(_) => (create_empty_chart_state(), PathBuf::new(), String::new()),
                    }
                }
                Err(_) => (create_empty_chart_state(), PathBuf::new(), String::new()),
            }
        } else {
            (create_empty_chart_state(), PathBuf::new(), String::new())
        };

        // Apply canvas dimensions from config
        apply_canvas_dimensions(&mut grid, startup_config.canvas_width, startup_config.canvas_height);

        Example {
            grid,
            selected_theme: Theme::TokyoNight,
            menu,
            menu_state,
            file_path,
            file_content,
            is_loading: false,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Menu(MenuMessage<MenuAction>),
    FileLoaded(Result<(PathBuf, String), String>),
    DialogDismissed,
}

#[derive(Debug, Clone)]
enum MenuAction {

}

impl Example {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Menu(mm) => match mm {
                MenuMessage::ItemPressed(label, menubar_index) => {
                    let menubar_name = menubar_index
                        .and_then(|idx| self.menu.configs.get(idx)
                        .map(|c| c.label))
                        .unwrap_or("Unknown");
                    
                    self.menu.clear_timers();

                    if menubar_name == "Theme" {
                        self.selected_theme = str_to_theme(&label);
                        self.grid.request_redraw();
                        Task::none()
                    } else if label == "Open File" {
                        self.is_loading = true;
                        let startup_config = StartupConfig::load();
                        let last_folder = startup_config.get_last_folder();
                        Task::perform(pick_yaml_file(last_folder), Message::FileLoaded)
                    } else if label == "Exit" {
                        window::latest().and_then(window::close)
                    } else {
                        // Check if this is a recent file
                        if self.menu_state.recent_files.contains(&label.to_string()) {
                            self.is_loading = true;
                            let path = label.to_string();
                            Task::perform(
                                async move {
                                    let path_buf = PathBuf::from(path.clone());
                                    fs::read_to_string(&path_buf)
                                        .map(|content| (path_buf, content))
                                        .map_err(|e| e.to_string())
                                },
                                Message::FileLoaded
                            )
                        } else {
                            Task::none()
                        }
                    }
                }
                _ => self.menu.update(mm).map(Message::Menu)
            },
            Message::FileLoaded(result) => {
                self.is_loading = false;
                match result {
                    Ok((path, content)) => {
                        match build_grid_from_yaml(&path, &content, &self.selected_theme) {
                            Ok(mut grid) => {
                                grid.request_redraw();
                                
                                // Load config to get canvas dimensions
                                let startup_config = StartupConfig::load();
                                apply_canvas_dimensions(&mut grid, startup_config.canvas_width, startup_config.canvas_height);
                                
                                self.grid = grid;
                                self.file_path = path.clone();
                                self.file_content = content;
                                
                                // Add to recent files, update last folder, and save
                                let mut updated_config = startup_config.clone();
                                updated_config.add_recent_file(path.to_string_lossy().into_owned());
                                updated_config.set_last_folder_from_path(&path);
                                match updated_config.save() {
                                    Ok(_) => println!("Config saved successfully. Last folder: {:?}", updated_config.last_folder),
                                    Err(e) => eprintln!("Failed to save config: {}", e),
                                }
                                
                                // Update menu state
                                self.menu_state.recent_files = updated_config.recent_files;
                                let configs = define_menu_configs(&self.menu_state);
                                self.menu.configs = configs;
                                
                                Task::none()
                            }
                            Err(err) => {
                                Task::perform(show_error(err), |_| Message::DialogDismissed)
                            }
                        }
                    }
                    Err(err) if err == "cancelled" => Task::none(),
                    Err(err) => Task::perform(show_error(err), |_| Message::DialogDismissed),
                }
            }
            Message::DialogDismissed => Task::none(),
        }
    }

    fn view(&self) -> impl Widget<Message> {

        let menu = self.menu.construct_menu().map(Message::Menu);

        column(vec![
            menu.boxed(),
            container(self.grid.view().map(|_| unreachable!()))
                .boxed()
        ]).padding(20.0).boxed()
    }

    fn theme(&self) -> Theme {
        self.selected_theme.clone()
    }

    fn title(&self) -> String {
        if self.file_path.as_os_str().is_empty() {
            "iced_charts — multi_charts".to_string()
        } else {
            format!("iced_charts — {}", self.file_path.display())
        }
    }
}

use iced_menu::{MenuConfig, MenuItem};
use std::rc::Rc;
use iced::Element;

fn define_menu_configs(menu_state: &MenuState) -> Vec<MenuConfig<'static, MenuAction>> {
    let size = menu_state.font_size;

    vec![        
        MenuConfig {
            label: "File",
            width: 250.0,
            items: vec![
                MenuItem::Item { label: "Open File", size },
                MenuItem::Item { label: "Open Folder", size },
                MenuItem::SubMenu {
                    config: MenuConfig {
                        label: "Open Recent >",
                        width: 240.0,
                        items: menu_state.recent_files.clone()
                            .into_iter()
                            .map(|path| {
                                let display_name = Path::new(&path)
                                    .file_name()
                                    .and_then(|name| name.to_str())
                                    .unwrap_or(&path);
                                MenuItem::Item { label: Box::leak(display_name.to_string().into_boxed_str()), size }
                            })
                            .collect(),
                            ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Item { label: "Exit", size },
            ],
            ..Default::default()
        },
        MenuConfig {
            label: "Theme",
            width: 200.0,
            items: theme_strings()
                .into_iter()
                .map(|theme| MenuItem::Item { label: Box::leak(theme.into_boxed_str()), size })
                .collect(),
            ..Default::default()
        }
        
    ]
}

fn separator() -> impl Fn() -> Element<'static, MenuMessage<MenuAction>> {
    || {
        iced::widget::rule::horizontal(2.0).boxed()
    }
}

/// Pick a YAML config file and read its contents.
async fn pick_yaml_file(last_folder: Option<PathBuf>) -> Result<(PathBuf, String), String> {
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
async fn show_error(message: String) {
    let _ = DialogBuilder::message()
        .set_level(MessageLevel::Error)
        .set_title("Chart load error")
        .set_text(message)
        .alert()
        .spawn()
        .await;
}

/// Parse a YAML config, resolve its data source relative to the config file, and build the chart.
fn build_grid_from_yaml(path: &Path, content: &str, theme: &Theme) -> Result<ChartState, String> {
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

/// Create an empty chart state for startup when no recent files are available
fn create_empty_chart_state() -> ChartState {
    let data = ChartData::default();
    let chart = Chart::new(1, ChartKind::None);
    ChartState::new(chart, data)
}

/// Apply canvas dimensions from config to a ChartState
fn apply_canvas_dimensions(chart_state: &mut ChartState, width: f32, height: f32) {
    chart_state.set_width(width);
    chart_state.set_height(height);
}
