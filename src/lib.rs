mod functions;

#[cfg(test)]
mod tests {
    use core::task;

    use super::*;
    
    #[test]
    fn create_file() {
        let file = functions::create_file("todo.list");
        assert!(file.is_ok());
    }

    #[test]
    fn adding_tasks() {
        let file_name = "todo.list";
        let task_one = "Learn basics of Rust";
        let task_two = "Learn basics of C++";
        let task_three = "Touch some grass!";

        assert!(functions::add_task(file_name, task_one).is_ok());
        assert!(functions::add_task(file_name, task_two).is_ok());
        assert!(functions::add_task(file_name, task_three).is_ok());
    }

    #[test]
    fn delete_task() {
        let file_name = "todo.list";
        let task_number = 1;

        assert!(functions::delete_task(file_name, task_number).is_ok());
    }

    #[test]
    fn empty_file() {
        let file_name = "todo.list";
        assert!(functions::empty_file(file_name).is_ok());
    }
}