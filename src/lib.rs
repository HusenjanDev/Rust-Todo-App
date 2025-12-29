mod functions;

#[cfg(test)]
mod tests {
    use core::task;

    use super::*;
    
    #[test]
    fn open_file() {
        let file = functions::open_file("todo.list");
    }
}