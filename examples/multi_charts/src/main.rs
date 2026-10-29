use iced::widget::{container, column};
use iced::{Task, Theme, Widget, window};
use iced_charts::ChartState;
use iced_charts::model::theme_helpers::{theme_strings, str_to_theme};
use iced_charts::config::load_chart_from_yaml;

use iced_menu::{Menu, MenuMessage};

use rfd::AsyncFileDialog;
use native_dialog::DialogBuilder;

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .theme(|ex: &Example| ex.theme())
        .run()
}

#[derive(Debug, Clone, Default)]
struct MenuState {
    is_dark_mode: bool,
}

struct Example {
    grid: ChartState,
    selected_theme: Theme,
    menu: Menu<'static, MenuAction>,
    menu_state: MenuState,
    file_path: PathBuf,
    file_content: String,
    is_loading: bool
}

impl Default for Example {
    fn default() -> Self {
        let menu_state = MenuState::default();

        let configs = define_menu_configs(&menu_state);
        let len = configs.len();
        let mut menu = Menu::default();
        menu.configs = configs;
        menu.is_open = vec![false; len];

        // Load chart configuration from YAML using library function
        let config_content = include_str!("../config_csv.yml");
        let config = load_chart_from_yaml(config_content).expect("Failed to load config_csv.yml");

        // Build a ready-to-render chart state directly from the config
        let grid = config.charts[0]
            .build_state(&Theme::TokyoNight)
            .expect("Failed to build chart state");

        Example {
            grid,
            selected_theme: Theme::TokyoNight,
            menu,
            menu_state,
            file_path: PathBuf::new(),
            file_content: "".to_string(),
            is_loading: false,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Menu(MenuMessage<MenuAction>),
    LoadFile(PathBuf),
    FileLoaded(String, String),
}

#[derive(Debug, Clone)]
enum MenuAction {
    SetDarkMode(bool),

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
                    if menubar_name == "Theme" {
                        self.selected_theme = str_to_theme(&label);
                        self.grid.request_redraw();
                    }
                    println!("{} > {}", menubar_name, &label);
                    self.menu.clear_timers();
                    if label == "Exit" {
                        window::latest().and_then(window::close)
                    } else {
                        Task::none()
                    }
                }
                MenuMessage::Custom(action) => match action {
                    MenuAction::SetDarkMode(checked) => {
                        self.menu_state.is_dark_mode = checked;
                        // Rebuild configs so widgets reflect the new state
                        self.menu.configs = define_menu_configs(&self.menu_state);
                        println!("Dark mode: {}", self.menu_state.is_dark_mode);
                        Task::none()
                    }
                }
                _ => self.menu.update(mm).map(Message::Menu)
            },
            Message::LoadFile(path) => {
                let task = Task::perform(
                    load_file(path.clone()),
                    move |result| {
                        match result {
                            Ok(content) => {
                                let file_name = path.file_name().map(|f| f.to_string_lossy().to_string());
                                let file_content = content.to_string();
                                Message::FileSystemWindow(
                                    Message::FileLoaded(file_name, Some(file_content)),
                                )
                            },
                            Err(e) => {
                                eprintln!("[ERROR] Failed to load file: {:?}", e);
                                Message::FileSystemWindow(id, Message::FileLoaded(None, None))
                            }
                        }
                    },
                );
                
                task
            
            },
            Message::FileLoaded(file_name, file_content) => {
                
                self.file_path = file_name;
                self.file_content = file_content;
                self.is_loading = false;
                    
                Task::none()
            },
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
}

use iced_menu::{MenuConfig, MenuItem};
use std::path::PathBuf;
use std::rc::Rc;
use iced::Element;

fn define_menu_configs(state: &MenuState) -> Vec<MenuConfig<'static, MenuAction>> {
    let size = 14.0;
    
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
                        items: vec![
                            MenuItem::Item {
                                label: "", size },
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Item { label: "Exit", size },
            ],
            ..Default::default()
        },
        MenuConfig {
            label: "Edit",
            width: 200.0,
            gap: None,
            items: vec![
                MenuItem::Item { label: "Undo", size },
                MenuItem::Item { label: "Redo", size },
                MenuItem::Item { label: "Cut", size },
                MenuItem::Item { label: "Copy", size },
                MenuItem::Item { label: "Paste", size },
                MenuItem::SubMenu {
                    config: MenuConfig {
                        label: "Find >",
                        width: 150.0,
                        items: vec![
                            MenuItem::Item { label: "Find", size },
                            MenuItem::Item { label: "Find Next", size },
                            MenuItem::Item { label: "Find Previous", size },
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Item { label: "Replace", size },
                MenuItem::Item { label: "Find in Files", size },
                MenuItem::Item { label: "Replace in Files", size },
                MenuItem::Element(Rc::new(dark_mode_toggle(state.is_dark_mode))),
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


fn dark_mode_toggle(checked: bool) -> impl Fn() -> Element<'static, MenuMessage<MenuAction>> {
    move || {
        iced::widget::checkbox(checked)
            .label("Dark Mode")
            .on_toggle(|b| MenuMessage::Custom(MenuAction::SetDarkMode(b)))
            .boxed()
    }
}

