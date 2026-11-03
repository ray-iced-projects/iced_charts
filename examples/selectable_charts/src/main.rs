use iced::Length::Fill;
use iced::widget::{container, column, row};
use iced::{Subscription, Task, Theme, Widget, window};
use iced_charts::{Chart, ChartData, ChartKind, ChartMessage, ChartState};
use iced_charts::model::theme_helpers::str_to_theme;

use iced_menu::{Menu, MenuMessage};

use std::fs;
use std::path::{Path, PathBuf};

use crate::menu_config::{MenuIntent, MenuState, define_menu_configs, resolve_menu_intent};
use crate::start_up_configs::StartupConfig;
use crate::demo::{DemoController, DemoEvent};
use crate::yml_loader::{build_charts_from_yaml, pick_yaml_file, process_loaded_chart_file, show_error};

mod demo;
mod menu_config;
mod start_up_configs;
mod yml_loader;


pub fn main() -> iced::Result {
    let startup_config = StartupConfig::load();
    
    iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .theme(|ex: &Example| ex.theme())
        .window_size((startup_config.window_width as f32, startup_config.window_height as f32))
        .centered()
        .subscription(|state| state.subscription())
        .run()
}


struct Example {
    charts: Vec<ChartState>,
    selected_theme: Theme,
    menu: Menu<'static, MenuAction>,
    menu_state: MenuState,
    demo: DemoController,
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

        // Try to load the most recent file, or create an empty chart state.
        let (mut charts, file_path, file_content) = if let Some(recent_path) = startup_config.recent_files.first() {
            match fs::read_to_string(recent_path) {
                Ok(content) => {
                    match build_charts_from_yaml(Path::new(recent_path), &content, &Theme::TokyoNight) {
                        Ok(charts) => (charts, PathBuf::from(recent_path), content),
                        Err(_) => (create_empty_charts(), PathBuf::new(), String::new()),
                    }
                }
                Err(_) => (create_empty_charts(), PathBuf::new(), String::new()),
            }
        } else {
            (create_empty_charts(), PathBuf::new(), String::new())
        };

        // Apply canvas dimensions from config
        for chart in &mut charts {
            apply_canvas_dimensions(chart, startup_config.canvas_width, startup_config.canvas_height);
        }

        Example {
            charts,
            selected_theme: Theme::TokyoNight,
            menu,
            menu_state,
            demo: DemoController::new(),
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
    Demo(DemoEvent),
    DialogDismissed,
    Chart { index: usize, msg: ChartMessage },
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

                    match resolve_menu_intent(&label, Some(menubar_name), &self.menu_state) {
                        MenuIntent::Theme(theme_name) => {
                            self.selected_theme = str_to_theme(&theme_name);
                            if !self.file_path.as_os_str().is_empty() && !self.file_content.is_empty() {
                                match build_charts_from_yaml(&self.file_path, &self.file_content, &self.selected_theme) {
                                    Ok(mut charts) => {
                                        let startup_config = StartupConfig::load();
                                        for chart in &mut charts {
                                            apply_canvas_dimensions(chart, startup_config.canvas_width, startup_config.canvas_height);
                                            chart.request_redraw();
                                        }
                                        self.charts = charts;
                                        Task::none()
                                    }
                                    Err(err) => Task::perform(show_error(err), |_| Message::DialogDismissed),
                                }
                            } else {
                                for chart in &mut self.charts {
                                    chart.request_redraw();
                                }
                                Task::none()
                            }
                        }
                        MenuIntent::OpenFile => {
                            self.is_loading = true;
                            let startup_config = StartupConfig::load();
                            let last_folder = startup_config.get_last_folder();
                            Task::perform(pick_yaml_file(last_folder), Message::FileLoaded)
                        }
                        MenuIntent::OpenChart(chart_name) => {
                            self.is_loading = true;
                            let chart_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                                .join("examples")
                                .join("selectable_charts")
                                .join("data")
                                .join(format!("{chart_name}.yml"));
                            let path_buf = chart_path.clone();
                            Task::perform(
                                async move {
                                    fs::read_to_string(&path_buf)
                                        .map(|content| (path_buf, content))
                                        .map_err(|e| e.to_string())
                                },
                                Message::FileLoaded
                            )
                        }
                        MenuIntent::DemoStart => {
                            if let Some(path) = self.demo.start() {
                                self.is_loading = true;
                                let path_buf = path.clone();
                                return Task::perform(
                                    async move {
                                        fs::read_to_string(&path_buf)
                                            .map(|content| (path_buf, content))
                                            .map_err(|e| e.to_string())
                                    },
                                    Message::FileLoaded,
                                );
                            }

                            Task::none()
                        }
                        MenuIntent::RecentFile(path) => {
                            self.is_loading = true;
                            let path_buf = path.clone();
                            Task::perform(
                                async move {
                                    fs::read_to_string(&path_buf)
                                        .map(|content| (path_buf, content))
                                        .map_err(|e| e.to_string())
                                },
                                Message::FileLoaded
                            )
                        }
                        MenuIntent::Exit => window::latest().and_then(window::close),
                        MenuIntent::NoOp => Task::none(),
                    }
                }
                _ => self.menu.update(mm).map(Message::Menu)
            },
            Message::Demo(DemoEvent::Tick) => {
                if let Some(next_path) = self.demo.advance() {
                    self.is_loading = true;
                    let path_buf = next_path.clone();
                    return Task::perform(
                        async move {
                            fs::read_to_string(&path_buf)
                                .map(|content| (path_buf, content))
                                .map_err(|e| e.to_string())
                        },
                        Message::FileLoaded,
                    );
                }

                Task::none()
            }
            Message::FileLoaded(result) => {
                self.is_loading = false;
                match result {
                    Ok((path, content)) => {
                        match process_loaded_chart_file(path, content, &self.selected_theme) {
                            Ok(loaded) => {
                                let startup_config = StartupConfig::load();
                                self.charts = loaded.charts;
                                for chart in &mut self.charts {
                                    apply_canvas_dimensions(chart, startup_config.canvas_width, startup_config.canvas_height);
                                    chart.request_redraw();
                                }
                                self.file_path = loaded.file_path;
                                self.file_content = loaded.file_content;
                                self.menu_state.recent_files = loaded.recent_files;
                                self.menu_state.recent_files_display = loaded.recent_files_display;
                                self.menu.configs = define_menu_configs(&self.menu_state);
                                Task::none()
                            }
                            Err(err) => Task::perform(show_error(err), |_| Message::DialogDismissed),
                        }
                    }
                    Err(err) if err == "cancelled" => Task::none(),
                    Err(err) => Task::perform(show_error(err), |_| Message::DialogDismissed),
                }
            }
            Message::DialogDismissed => Task::none(),
            Message::Chart { index, msg } => {
                if let Some(chart) = self.charts.get_mut(index) {
                    chart.apply(msg);
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> impl Widget<Message> {
        let menu = self.menu.construct_menu().map(Message::Menu);

        let charts = self.charts.iter().enumerate().map(|(index, chart)| {
            container(chart.view().map(move |msg| Message::Chart { index, msg }))
                .padding(10.0)
                .boxed()
        }).collect::<Vec<_>>();

        container(
            column(vec![
                menu.boxed(),
                row(charts).spacing(20.0).boxed(),
            ]).padding(20.0).boxed()
        ).center(Fill).boxed()
    }

    fn theme(&self) -> Theme {
        self.selected_theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.demo.is_running() {
            iced::time::every(std::time::Duration::from_secs(1))
                .map(|_| Message::Demo(DemoEvent::Tick))
        } else {
            Subscription::none()
        }
    }

    fn title(&self) -> String {
        if self.file_path.as_os_str().is_empty() {
            "iced_charts — selectable_charts".to_string()
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

fn create_empty_charts() -> Vec<ChartState> {
    vec![create_empty_chart_state()]
}

/// Apply canvas dimensions from config to a ChartState
fn apply_canvas_dimensions(chart_state: &mut ChartState, width: f32, height: f32) {
    chart_state.apply_default_canvas_size(width, height);
}
