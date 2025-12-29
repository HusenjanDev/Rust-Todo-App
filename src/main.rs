use core::panic;
use std::{io::{Read, Write}};

/*
 * create_file() : creating file if the file doesn't exist.
*/
fn create_file(file_name : &String) {
    let _todo_file = std::fs::File::open(file_name);

    let _todo_file = match _todo_file {
        Ok(file) => file,
        Err(err) => {
            match err.kind() {
                std::io::ErrorKind::NotFound => match std::fs::File::create(file_name) {
                    Ok(fc) => fc, 
                    Err(_e) => {
                        panic!("[#] An issue occurred while creating the file.");
                    }
                }
                _ => {
                    panic!("[#] An issue occurred while opening the file.");
                }
            }
        }
    };
}

/*
 * open_file() : checking if we can open the file, return true or false.
*/
fn open_file(file_name : &str) -> bool {
    match std::fs::File::open(file_name) {
        Ok(_f) => return true,
        Err(_e) => return false
    }
}

/*
 * read_todo_file() : reads from the todo file and prints it to terminal.
*/
fn read_todo_file(file_name: &String) {
    let mut file = std::fs::File::open(file_name).expect("[#] An issue occured while opening the file");
    let mut content = String::new();
    let _ = file.read_to_string(&mut content).expect("[#] An issue occurred while reading the content of the file.");

    for line in content.lines() {
        println!("{line}");
    }
}

/*
 * add_task() : adds task into file.
*/
fn add_task(file_name: &String, task: String) {
    let mut file = std::fs::OpenOptions::new().append(true).open(file_name).expect("[#] An issue occured while opening the file.");
    let mtask = String::from("• [ ] ".to_string() + &task + "\n");
    let _ = file.write(mtask.as_bytes()).expect("[#] An issue occurred while adding a task.");
}

/*
 * delete_task() : deleting a task from file.
*/
fn delete_task(file_name : &str, task_number : usize) {
    let mut content = String::new();
    let mut vector = std::vec::Vec::new();

    match &mut std::fs::File::open(file_name) {
        Ok(file) => {
            let _ = file.read_to_string(&mut content);

            for (index, line) in content.lines().enumerate() {
                if index == (task_number - 1) {
                    continue; 
                }

                vector.push(line);
            }
        }
        Err(_err) => {
            panic!("[!] An issue occurred at delete_task() where read file action is performed.");
        }
    };

    // Deleting content of the file.
    std::fs::OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(file_name)
                        .expect("[!] An issue occurred at delete_task() where trunncate is performed on a file.");

    // Writing data inside the vector to file.
    match &mut std::fs::File::open(file_name) {
        Ok(file) => {
            for item in vector {
                let line = item.to_string() + "\n";
                let _ = file.write(line.as_bytes());
                println!("{item}");
            }
        }
        Err(_err) => {
            panic!("[!] An error occurred at complete_task while adding data to file.")
        }
    }
}

/*
 * get_total_length_of_file() : getting the total length of the todo.txt file.
*/
fn get_total_length_of_file(file_name : &String) -> usize {
    let mut content = String::new();
    
    match &mut std::fs::File::open(file_name) {
        Ok(file) => {
            let _ = file.read_to_string(&mut content);
            return content.lines().count() + 1
        }
        Err(_err) => {
            panic!("[!] An issue occurred at get_total_length_of_file while opening file.");
        }
    }
}

/*
 * task_completed() : adds or removes `- [x]` task from file.  
*/
fn complete_task(file_name : &String, task: usize) {
    let mut content = String::new();
    let mut vector = std::vec::Vec::new();

    // Opening file and reading data into content variable.
    match &mut std::fs::File::open(file_name) {
        Ok(file) => {
            let _ = file.read_to_string(&mut content);
        }
        Err(_err) => {
            panic!("[!] An issue occurred while completing a task!");
        }
    }

    // Adding `• []` to the completed task.
    for (index, line) in content.lines().enumerate() {
        if index == (task - 1) && line.contains("• [ ] ") {
            vector.push(line.to_string().replace("• [ ] ", "• [x] "));
        }
        else if index == (task - 1) && line.contains("• [x] ") {
            vector.push(line.to_string().replace("• [x] ", "• [ ] "));
        }
        else {
            vector.push(line.to_string());
        }
    }

    // Deleting file.
    std::fs::OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(file_name)
                        .expect("[!] An issue occurred at task_complete() where truncate is performed.");
    
    // Appending modified data to the cleaned file.
    match &mut std::fs::OpenOptions::new().write(true).append(true).open(file_name) {
        Ok(file) => {
            for item in vector.iter_mut() {
                let line = item.to_string() + "\n";
                let _ = file.write(line.as_bytes());
                println!("{item}");
            }
        }
        Err(_err) => {
            panic!("[!] An issue occurred at task_complete() where writing to file is performed.");
        }
    }
}

/*
 * empty_file() : Deletes all content inside the file.
*/
fn empty_file(file_name : &str) -> bool {
    match std::fs::OpenOptions::new()
    .write(true)
    .truncate(true)
    .open(file_name) {
        Ok(_f) => return true,
        Err(_e) => return false
    }
}

fn main() {
    // Todo file name
    let file_name = String::from("todo.txt");

    // Creating file if it doesn't exist.
    if open_file(&file_name) == false {
        create_file(&file_name);
    }

    // Getting arguments for CLI
    let args : Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        read_todo_file(&file_name);
    }
    else if args.len() == 3 && args[1].contains("delete") {
        let task_number : usize = args[2].parse().unwrap();

        if task_number <= get_total_length_of_file(&file_name) {
            delete_task(&file_name, task_number);
        }
        else {
            println!("Enter an valid task number to perform this action.");
        }
    }
    else if args.len() == 3 && args[1].contains("complete") {
        let task_number : usize = args[2].parse().unwrap();

        if task_number <= get_total_length_of_file(&file_name) {
            complete_task(&file_name, task_number);
        }
        else {
            println!("Enter an valid task number to perform this action.");
        }
    }
    else if args.len() == 2 && args[1].contains("read") {
        read_todo_file(&file_name);
    }
    else if args.len() == 3 && args[1].contains("add") {
        add_task(&file_name, args[2].to_string());
    }
    else if args.len() == 2  && args[1].contains("clean") {
        empty_file(&file_name);
    }
    else if args.len() == 2 && args[1].contains("--help") {
        println!(".\\TotoApp.exe add \"Clean dishes\" ");
        println!(".\\TotoApp.exe complete `[TASK-NUMBER]`");
        println!(".\\TodoApp.exe delete `[TASK-NUMBER]`");
    }
}