use anyhow::{anyhow, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
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
    pub fn new() -> Result<Todos, anyhow::Error> {
        let mut new_todos = Todos {
            list: Vec::new(),
            done: Vec::new(),
        };

        let mut saved = File::open("todo_db")?;
        let mut contents = String::new();
        saved.read_to_string(&mut contents)?;
        if let Ok(saved_todos) = serde_json::from_str::<Todos>(&contents) {
            new_todos.list = saved_todos.list;
            new_todos.done = saved_todos.done;
        };
        Ok(new_todos)
    }
    pub fn add(&mut self, content: String) -> Result<()> {
        let todo = Todo {
            content,
            done: false,
        };
        self.list.push(todo);
        self.save()?;
        Ok(())
    }

    pub fn remove(&mut self, index: u16) -> Result<()> {
        if index as usize >= self.list.len() {
            return Err(anyhow!("Provided index doesn't exist"));
        }
        self.list.remove(index.into());
        self.save()?;
        Ok(())
    }

    pub fn mark_done(&mut self, index: u16) -> Result<()> {
        if index as usize >= self.list.len() {
            return Err(anyhow!("Provided index doesn't exist"));
        }

        let mut todo = self.list.remove(index.into());
        todo.done = true;
        self.done.push(todo);
        self.save()?;
        Ok(())
    }

    pub fn print(&self, all: bool) {
        let todos_fmt = self.format_tasks(&self.list);
        println!("{}", "Todos".bold());
        println!("{}", todos_fmt);
        println!("");
        if all {
            let done_fmt = self.format_tasks(&self.done);
            println!("{}", "Done".bold());
            println!("{}", done_fmt);
        }
    }

    fn save(&mut self) -> Result<()> {
        let saved_serde = serde_json::to_string(&self)?;
        let mut file = File::create("todo_db")?;
        file.write_all(saved_serde.as_bytes())?;
        Ok(())
    }

    fn format_tasks(&self, tasks: &Vec<Todo>) -> String {
        tasks
            .iter()
            .enumerate()
            .map(|(index, todo)| format!("{} {}", index.to_string().bold(), todo.content))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
