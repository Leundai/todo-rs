use colored::Colorize;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::{
    fs::File,
    io::{Read, Write},
};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    content: String,
    done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todos {
    list: Vec<Todo>,
    done: Vec<Todo>,
}

impl Todos {
    pub fn new() -> Todos {
        let mut new_todos = Todos {
            list: Vec::new(),
            done: Vec::new(),
        };

        let saved = File::open("todo_db");
        let mut contents = String::new();
        let _ = saved.unwrap().read_to_string(&mut contents);
        if let Ok(saved_todos) = serde_json::from_str::<Todos>(&contents) {
            new_todos.list = saved_todos.list;
            new_todos.done = saved_todos.done;
        };

        new_todos
    }
    pub fn add(&mut self, content: String) {
        let todo = Todo {
            content,
            done: false,
        };
        self.list.push(todo);
        self.save();
    }

    pub fn remove(&mut self, index: u16) {
        self.list.remove(index.into());
        self.save();
    }

    pub fn mark_done(&mut self, index: u16) {
        if index as usize >= self.list.len() {
            error!("Given index is larger than todo list");
            return;
        }

        let mut todo = self.list.remove(index.into());
        todo.done = true;
        self.done.push(todo);
        self.save();
    }

    fn save(&mut self) {
        let saved_serde = serde_json::to_string(&self).unwrap();
        let file = File::create("todo_db");
        let _ = file.unwrap().write_all(saved_serde.as_bytes());
    }
}

impl fmt::Display for Todos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let todos_fmt = self
            .list
            .iter()
            .enumerate()
            .map(|(index, todo)| format!("{} {}", index.to_string().bold(), todo.content))
            .collect::<Vec<String>>()
            .join("\n");
        let done_fmt = self
            .done
            .iter()
            .enumerate()
            .map(|(index, todo)| format!("{} {} ✅", index.to_string().bold(), todo.content))
            .collect::<Vec<String>>()
            .join("\n");

        let result = format!(
            "\
            {}\n\
            {}\n\
            \n\n\
            {}\n\
            {}",
            "Pending".bold(),
            todos_fmt,
            "Done".bold(),
            done_fmt
        );
        write!(f, "{}", result)
    }
}
