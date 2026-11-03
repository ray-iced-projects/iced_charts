use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub enum DemoEvent {
    Tick,
}

#[derive(Debug, Clone)]
pub struct DemoController {
    files: Vec<PathBuf>,
    index: usize,
    running: bool,
}

impl Default for DemoController {
    fn default() -> Self {
        Self::new()
    }
}

impl DemoController {
    pub fn new() -> Self {
        let files = demo_files();
        Self {
            files,
            index: 0,
            running: false,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn start(&mut self) -> Option<PathBuf> {
        if self.files.is_empty() {
            self.running = false;
            self.index = 0;
            return None;
        }

        self.index = 0;
        self.running = true;
        Some(self.files[0].clone())
    }

    pub fn advance(&mut self) -> Option<PathBuf> {
        if !self.running || self.files.is_empty() {
            return None;
        }

        self.index += 1;

        if self.index >= self.files.len() {
            self.running = false;
            self.index = self.files.len().saturating_sub(1);
            return None;
        }

        Some(self.files[self.index].clone())
    }
}

fn demo_files() -> Vec<PathBuf> {
    let data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("selectable_charts")
        .join("data");

    let mut files = fs::read_dir(data_dir)
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("yml") {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    files.sort();
    files
}