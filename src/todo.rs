use crate::handle_print::handle_print;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize)]
pub struct Todo {
    id: usize,
    content: String,
    done: bool,
}

impl Todo {
    fn print(&self) {
        handle_print(format!(
            "---------------------------------------\n\
            id: {}, done: {}\n\
            {}",
            self.id, self.done, self.content
        ))
    }
}

pub struct TodoList {
    pub list: Vec<Todo>,
}

impl TodoList {
    pub fn new(file_path: Option<&str>) -> Self {
        match file_path {
            Some(path) => match File::open(path) {
                Ok(file) => Self {
                    list: serde_json::from_reader(file).expect("error while reading"),
                },
                _ => Self { list: vec![] },
            },
            None => Self { list: vec![] },
        }
    }

    pub fn print_list(&self) {
        for item in self.list.iter() {
            item.print();
        }
    }

    pub fn add_action(&mut self, content: String) {
        let id = if self.list.is_empty() {
            1
        } else {
            self.list[self.list.len() - 1].id + 1
        };

        let todo = Todo {
            id,
            content,
            done: false,
        };
        handle_print("added the following item:".to_string());
        todo.print();
        self.list.push(todo);
    }

    pub fn remove_action(&mut self, id: usize) {
        handle_print(format!("removing item with id of {id}"));
        self.list.retain_mut(|i| i.id != id);
    }

    pub fn edit_action(&mut self, id: usize, content: String) {
        if let Some(item) = self.list.iter_mut().find(|i| i.id == id) {
            item.content = content;
            handle_print("edited the following item: ".to_string());
            item.print();
        } else {
            handle_print(format!("item with id: {id} doesn't exist!"));
        }
    }

    pub fn done_action(&mut self, id: usize, done: bool) {
        if let Some(item) = self.list.iter_mut().find(|i| i.id == id) {
            item.done = done;
            handle_print("edited the following item: ".to_string());
            item.print();
        } else {
            handle_print(format!("item with id: {id} doesn't exist!"));
        }
    }
}
