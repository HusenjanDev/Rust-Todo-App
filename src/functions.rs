use std::{io::{Read, Write}, vec};
use colored_text::Colorize;

/*
 * create_file() : creating file if the file doesn't exist.
*/
#[allow(dead_code)]
pub fn create_file(file_name : &str) -> std::io::Result<()> {
    std::fs::File::create(file_name)?;
    Ok(())
}

/*
 * open_file() : checking if we can open the file, return true or false.
*/
#[allow(dead_code)]
pub fn open_file(file_name : &str) -> std::io::Result<()>{
    match std::fs::File::open(file_name) {
        Ok(_f) => Ok(()),
        Err(error) => Err(error)
    }
}

/*
 * read_todo_file() : reads from the todo file and prints it to terminal.
*/
#[allow(dead_code)]
pub fn read_file(file_name: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::open(file_name)?;
    let mut content = String::new();
    
    file.read_to_string(&mut content)?;

    println!("{}", "TODOS".magenta().bold());

    for line in content.lines() {
        if line.starts_with("• [x]") {
            println!("{}", line.green().strikethrough())
        } 
        else {
            println!("{}", line);
        }
    }

    Ok(())
}

/*
 * add_task() : adds task into file.
*/
#[allow(dead_code)]
pub fn add_task(file_name: &str, task: &str) -> std::io::Result<()> {
    let mut file = std::fs::OpenOptions::new().write(true).append(true).open(file_name)?;
    let modified_task = "• [ ] ".to_string() + &task + "\n";
    file.write(modified_task.as_bytes())?;
    read_file(file_name)?;
    Ok(())
}

/*
 * delete_task() : deleting a task from file.
*/
#[allow(dead_code)]
pub fn delete_task(file_name : &str, task_number : usize) -> std::io::Result<()> {
    let mut file = std::fs::OpenOptions::new().append(true).read(true).write(true).open(file_name)?;
    let mut content = String::new();
    let mut vector : Vec<&str>  = vec![];
    let task_number = task_number - 1;

    file.read_to_string(&mut content)?;

    if content.lines().count() < get_total_length_of_file(file_name) {
        println!("Enter an valid task number to perform this action.");
        return Ok(())
    }

    for (index, line) in content.lines().enumerate() {
        if index == task_number {
            continue;
        }
        vector.push(line);
    }


    // Deleting content of the file.
    std::fs::OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(file_name)?;

     // Appending modified data to the cleaned file.
    for item in vector {
        let line = item.to_string() + "\n";
        file.write(line.as_bytes())?;
    }

    read_file(file_name)?;

    Ok(())
}

/*
 * get_total_length_of_file() : getting the total length of the todo.txt file.
*/
#[allow(dead_code)]
pub fn get_total_length_of_file(file_name : &str) -> usize {    
    match &mut std::fs::File::open(file_name) {
        Ok(file) => {
            let mut content = String::new();
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
#[allow(dead_code)]
pub fn complete_task(file_name : &str, task_number: usize) -> std::io::Result<()> {
    let mut file = std::fs::OpenOptions::new().append(true).write(true).read(true).open(file_name)?;
    let mut content = String::new();
    let mut vector = std::vec::Vec::new();
    let task_number = task_number - 1;

    file.read_to_string(&mut content)?;


    // Adding `• []` to the completed task.
    for (index, line) in content.lines().enumerate() {
        if index == task_number && line.contains("• [ ] ") {
            vector.push(line.to_string().replace("• [ ] ", "• [x] "));
        }
        else if index == task_number && line.contains("• [x] ") {
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
    for item in vector {
        let line =  item.to_string() + "\n";
        file.write(line.as_bytes())?;
    }

    read_file(file_name)?;

    Ok(())
}

/*
 * empty_file() : Deletes all content inside the file.
*/
#[allow(dead_code)]
pub fn empty_file(file_name : &str) -> std::io::Result<()> {
    match std::fs::OpenOptions::new()
    .write(true)
    .truncate(true)
    .open(file_name) {
        Ok(_f) => Ok(()),
        Err(error) => Err(error)
    }
}

/*
 * terminal_cli(): Allows us to interact with the app.
*/
#[allow(dead_code)]
pub fn terminal_cli(args : &Vec<String>, file_name : &str) {
    if args.len() < 2 {
        if let Err(e) = read_file(&file_name) {
            println!("[!] read_file() error: {}", e)
        }
    }
    else if args.len() == 3 && args[1].contains("delete") {
        let task_number : usize = args[2].parse().unwrap();

        if task_number <= get_total_length_of_file(&file_name) {
            if let Err(e) = delete_task(&file_name, task_number) {
                println!("[!] delete_task() error: {}", e);
            }
        }
        else {
            println!("Enter an valid task number to perform this action.");
        }
    }
    else if args.len() == 3 && args[1].contains("complete") {
        let task_number : usize = args[2].parse().unwrap();

        if task_number <= get_total_length_of_file(&file_name) {
            if let Err(e) = complete_task(&file_name, task_number) {
                println!("[!] complete_task() error: {}", e);
            }
        }
        else {
            println!("Enter an valid task number to perform this action.");
        }
    }
    else if args.len() == 2 && args[1].contains("read") {
        if let Err(e) = read_file(&file_name) {
            println!("[!] read_file() error: {}", e);
        }
    }
    else if args.len() == 3 && args[1].contains("add") {
        if let Err(e) =  add_task(&file_name, &args[2]) {
            println!("[!] add_task() error: {}", e);
        }
    }
    else if args.len() == 2  && args[1].contains("clean") {
        if let Err(e) = empty_file(&file_name) {
            println!("[!] empty_file() error: {}", e);
        }
    }
}