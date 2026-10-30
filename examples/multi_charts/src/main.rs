use iced::Length::Fill;
use iced::widget::{container, column};
use iced::{Task, Theme, Widget, window};
use iced_charts::{Chart, ChartData, ChartKind, ChartState};
use iced_charts::model::theme_helpers::str_to_theme;

use iced_menu::{Menu, MenuMessage};



use std::fs;
use std::path::{Path, PathBuf};

use crate::menu_config::{MenuState, define_menu_configs};
use crate::start_up_configs::StartupConfig;
use crate::yml_loader::{build_grid_from_yaml, pick_yaml_file, show_error};

mod menu_config;
mod start_up_configs;
mod yml_loader;


pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .theme(|ex: &Example| ex.theme())
        .run()
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
                        // Check if this is a recent file by finding it in the display names
                        if let Some(index) = self.menu_state.recent_files_display.iter().position(|name| name == &label) {
                            if let Some(path_str) = self.menu_state.recent_files.get(index) {
                                self.is_loading = true;
                                let path = path_str.clone();
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
                                
                                // Update menu state with both full paths and display names
                                self.menu_state.recent_files = updated_config.recent_files.clone();
                                self.menu_state.recent_files_display = updated_config.recent_files.iter()
                                    .map(|path| {
                                        std::path::Path::new(path)
                                            .file_name()
                                            .and_then(|name| name.to_str())
                                            .unwrap_or(path)
                                            .to_string()
                                    })
                                    .collect();
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

        container(
            column(vec![
                menu.boxed(),
                container(self.grid.view().map(|_| unreachable!()))
                    .boxed()
            ]).padding(20.0).boxed()
        ).center(Fill).boxed()
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
