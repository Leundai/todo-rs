use log::{debug, error};
use std::fmt;

struct Todo {
    content: String,
    done: bool,
}

pub struct Todos {
    list: Vec<Todo>,
    done: Vec<Todo>,
}

impl Todos {
    pub fn new() -> Todos {
        Todos {
            list: Vec::new(),
            done: Vec::new(),
        }
    }
    pub fn add(&mut self, content: String) {
        let todo = Todo {
            content,
            done: false,
        };
        self.list.push(todo);
    }

    pub fn remove(&mut self, index: u16) {
        self.list.remove(index.into());
    }

    pub fn mark_done(&mut self, index: u16) {
        if index as usize >= self.list.len() {
            error!("Given index is larger than todo list");
            return;
        }

        let mut todo = self.list.remove(index.into());
        todo.done = true;
        self.done.push(todo);
    }
}

impl fmt::Display for Todos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let todos_fmt = self
            .list
            .iter()
            .enumerate()
            .map(|(index, todo)| format!("{} {}", index, todo.content))
            .collect::<Vec<String>>()
            .join("\n");
        let done_fmt = self
            .done
            .iter()
            .enumerate()
            .map(|(index, todo)| format!("{} {} ✅ \n", index, todo.content))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}\n\n{}", todos_fmt, done_fmt)
    }
}
