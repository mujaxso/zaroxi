mod app;
mod bootstrap;
mod commands;
mod ui;

use std::env;

use app::App;
use bootstrap::init;
use commands::Command;
use core_types::workspace::EditorCommand;

fn main() {
    init();
    
    let mut app = App::new();
    
    // Check if a workspace path was provided as an argument
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let path = &args[1];
        app.open_workspace(path.clone());
        
        // Create and execute the command
        let cmd = Command::new(EditorCommand::OpenWorkspace { path: path.clone() });
        cmd.execute();
        
        println!("Workspace opened at: {}", path);
        println!("Current workspace: {:?}", app.current_workspace());
    } else {
        println!("Usage: {} <workspace-path>", args[0]);
        println!("No workspace path provided. Starting with empty state.");
    }
    
    // Keep the application running
    println!("Neote desktop is running (press Ctrl+C to exit)...");
    // In a real GUI, we would run the event loop here
}
