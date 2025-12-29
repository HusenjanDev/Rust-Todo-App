mod functions;
use crate::functions::*;

fn main() {
    // Todo file name
    let file_name = String::from("todo.list");

    // Creating file if it doesn't exist.
    match open_file(&file_name) {
        Ok(()) => {
            // File exists, nothing to do
        },
        Err(_) => {
            // File doesn't exist, create it
            if let Err(err) = create_file(&file_name) {
                println!("[!] create_file error: {}", err);
            }
        }
    }

    // Getting arguments for CLI
    let args : Vec<String> = std::env::args().collect();

    // Terminal features
    terminal_cli(&args, &file_name);
}