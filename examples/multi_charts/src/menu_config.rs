

use iced_charts::model::theme_helpers::theme_strings;
use iced_menu::{MenuConfig, MenuItem, MenuMessage};
use std::{path::Path, rc::Rc};
use iced::{Widget, Element};

use crate::{MenuAction, start_up_configs::StartupConfig};

#[derive(Debug, Clone)]
pub struct MenuState {
    pub font_size: f32,
    pub recent_files: Vec<String>,  // full paths
    pub recent_files_display: Vec<String>,  // just filenames for menu display
}

impl Default for MenuState {
    fn default() -> Self {
        let startup_config = StartupConfig::load();
        let size = 14.0;
        let recent_files_display = startup_config.recent_files.iter()
            .map(|path| {
                Path::new(path)
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or(path)
                    .to_string()
            })
            .collect();
        Self {
            font_size: size,
            recent_files: startup_config.recent_files,
            recent_files_display,
        }
    }
}

pub fn define_menu_configs(menu_state: &MenuState) -> Vec<MenuConfig<'static, MenuAction>> {
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
                        items: menu_state.recent_files_display.clone()
                            .into_iter()
                            .map(|display_name| {
                                MenuItem::Item { label: Box::leak(display_name.into_boxed_str()), size }
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
