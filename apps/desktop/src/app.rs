#[allow(dead_code)]
pub struct App {
    workspace_path: Option<String>,
}

impl App {
    pub fn new() -> Self {
        App {
            workspace_path: None,
        }
    }

    pub fn open_workspace(&mut self, path: String) {
        self.workspace_path = Some(path);
    }

    pub fn current_workspace(&self) -> Option<&String> {
        self.workspace_path.as_ref()
    }
}
