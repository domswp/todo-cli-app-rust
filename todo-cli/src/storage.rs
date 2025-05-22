use crate::task::Task;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader};

const FILE_NAME: &str = "todos.txt";

pub fn load_todos() -> Vec<Task> {
    let file = OpenOptions::new().read(true).open(FILE_NAME);
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            reader
                .lines()
                .filter_map(Result::ok)
                .filter_map(|line| Task::from_string(&line))
                .collect()
        }
        Err(_) => Vec::new(),
    }
}

pub fn save_todos(todos: &[Task]) {
    let data: String = todos.iter().map(|t| t.to_string()).collect::<Vec<_>>().join("\n");
    fs::write(FILE_NAME, data).expect("Gagal menyimpan file");
}

