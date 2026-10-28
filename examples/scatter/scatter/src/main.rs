use iced::widget::{container, column};
use iced::{Task, Theme, Widget, window};
use iced_charts::model::features::{ChartText, TextKind};
use iced_charts::{ChartState, Chart, ChartData, ChartKind, SeriesData};
use iced_charts::resolve::series_color;
use iced_charts::model::style::ChartTheme;
use iced_charts::model::series_colors::{theme_strings, str_to_theme};

use iced_menu::{Menu, MenuMessage};


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
}

impl Default for Example {
    fn default() -> Self {

        let menu_state = MenuState::default();

        let configs = define_menu_configs(&menu_state);
        let len = configs.len();
        let mut menu = Menu::default();
        menu.configs = configs;
        menu.is_open = vec![false; len];

        let mut title = ChartText::new(TextKind::Title);
        title.content = "Four Quadrant Analysis - Centered at Origin (0, 0)".into();

        let mut x_title = ChartText::new(TextKind::XTitle);
        x_title.content = "X Axis (-10 to 10)".into();
        
        let mut y_title = ChartText::new(TextKind::YTitle);
        y_title.content = "Y Axis (-10 to 10)".into();
        
        let chart = 
            Chart::new(
                1,
                ChartKind::Scatter,
            )
            .title(title)
            .x_title(x_title)
            .y_title(y_title);
        
        let theme = ChartTheme::default();

        let data = ChartData {
            categories: vec![],
            series: vec![
                // Quadrant I (Top Right): X > 0, Y > 0
                SeriesData {
                    name: "Quadrant I (+, +)".into(),
                    x: Some(vec![Some(5.0), Some(3.0), Some(7.0), Some(5.0), Some(6.0), Some(4.0)]),
                    values: vec![Some(5.0), Some(7.0), Some(2.0), Some(7.0), Some(4.0), Some(8.0)],
                    kind: ChartKind::Scatter,
                    color: series_color(&theme, 0),
                    ..Default::default()
                },
                // Quadrant II (Top Left): X < 0, Y > 0
                SeriesData {
                    name: "Quadrant II (-, +)".into(),
                    x: Some(vec![Some(-5.0), Some(-4.0), Some(-7.0), Some(-5.0), Some(-4.0), Some(-7.0)]),
                    values: vec![Some(5.0), Some(3.0), Some(6.0), Some(4.0), Some(7.0), Some(5.0)],
                    kind: ChartKind::Scatter,
                    color: series_color(&theme, 1),
                    ..Default::default()
                },
                // Quadrant III (Bottom Left): X < 0, Y < 0
                SeriesData {
                    name: "Quadrant III (-, -)".into(),
                    x: Some(vec![Some(-5.0), Some(-3.0), Some(-6.0), Some(-5.0), Some(-3.0), Some(-6.0)]),
                    values: vec![Some(-5.0), Some(-6.0), Some(-2.0), Some(-4.0), Some(-7.0), Some(-3.0)],
                    kind: ChartKind::Scatter,
                    color: series_color(&theme, 2),
                    ..Default::default()
                },
                // Quadrant IV (Bottom Right): X > 0, Y < 0
                SeriesData {
                    name: "Quadrant IV (+, -)".into(),
                    x: Some(vec![Some(5.0), Some(4.0), Some(6.0), Some(5.0), Some(4.0), Some(6.0)]),
                    values: vec![Some(-5.0), Some(-4.0), Some(-7.0), Some(-6.0), Some(-3.0), Some(-5.0)],
                    kind: ChartKind::Scatter,
                    color: series_color(&theme, 3),
                    ..Default::default()
                },
            ],
            palette: vec![],
        };
        Example { 
            grid: ChartState::new(chart, data),
            selected_theme: Theme::TokyoNight,
            menu,
            menu_state,
         }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Menu(MenuMessage<MenuAction>),
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
            }
            
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
use std::rc::Rc;
use iced::Element;

fn define_menu_configs(state: &MenuState) -> Vec<MenuConfig<'static, MenuAction>> {
    let size = 14.0;
    
    vec![        
        MenuConfig {
            label: "File",
            width: 250.0,
            items: vec![
                MenuItem::Item {
                    label: "New File", size},
                MenuItem::Item {
                    label:"New Window", size},
                MenuItem::SubMenu {
                    config: MenuConfig {
                        label: "New Window with Profile >",
                        width: 200.0,
                        items: vec![
                            MenuItem::Item { label: "Profile 1", size },
                            MenuItem::Item { label: "Profile 2", size },
                            MenuItem::SubMenu {
                                config: MenuConfig {
                                    label: "Additional Profiles >",
                                    width: 100.0,
                                    items: vec![
                                        MenuItem::Item {
                                            label: "Profile 3",
                                            size },
                                        MenuItem::Item {
                                            label: "Profile 4",
                                            size },
                                        MenuItem::Item {
                                            label: "Profile 5",
                                            size },
                                    ],
                                    ..Default::default()
                                },
                            },
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Item { label: "Open File", size },
                MenuItem::Item { label: "Open Folder", size },
                MenuItem::Item { label: "Open Wokspace from File", size },
                MenuItem::SubMenu {
                    config: MenuConfig {
                        label: "Open Recent >",
                        width: 240.0,
                        items: vec![
                            MenuItem::Item {
                                label: "~/document/iced_menu", size },
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Item { label: "Save", size },
                MenuItem::Item { label: "Save As...", size },
                MenuItem::Item { label: "Save All", size },
                MenuItem::Item { label: "Revert File", size },
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
