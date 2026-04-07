mod files;
mod git;
mod tasks;
mod terminal;

fn main() {
    println!("Workspace daemon started");
    // For now, just print a message
    // In a real implementation, we would set up an RPC server
    println!("TODO: Implement RPC server to handle filesystem operations");
}
