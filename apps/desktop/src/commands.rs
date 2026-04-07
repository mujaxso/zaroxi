use core_types::workspace::EditorCommand;

pub struct Command {
    pub command: EditorCommand,
}

impl Command {
    pub fn new(command: EditorCommand) -> Self {
        Self { command }
    }

    pub fn execute(&self) {
        match &self.command {
            EditorCommand::OpenWorkspace { path } => {
                println!("Opening workspace at: {}", path);
                // TODO: Actually open the workspace
            }
            EditorCommand::OpenFile { path } => {
                println!("Opening file: {}", path);
                // TODO: Actually open the file
            }
            EditorCommand::SaveActiveFile => {
                println!("Saving active file");
                // TODO: Actually save the file
            }
        }
    }
}
