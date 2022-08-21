use crate::handle_print::handle_print;
use serde::{Deserialize, Serialize};

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

pub fn print_list(list: &[Todo]) {
    for item in list.iter() {
        item.print();
    }
}

pub fn remove_action(list: &mut Vec<Todo>, id: usize) {
    handle_print(format!("removing item with id of {id}"));
    list.retain_mut(|i| i.id != id);
}

pub fn edit_action(list: &mut [Todo], id: usize, content: String) {
    for item in list.iter_mut() {
        if item.id == id {
            item.content = content;
            handle_print("edited the following item:".to_string());
            item.print();
            break;
        } else {
            handle_print(format!("item with id: {id} doesn't exist!"));
        }
    }
}

pub fn done_action(list: &mut [Todo], id: usize, done: bool) {
    for item in list.iter_mut() {
        if item.id == id {
            item.done = done;
            let msg = if done { "completed" } else { "undoing" };
            handle_print(format!("{msg} the following item:"));
            item.print();
            break;
        } else {
            handle_print(format!("item with id: {id} doesn't exist!"));
        }
    }
}

pub fn add_action(list: &mut Vec<Todo>, content: String) {
    let id = if list.is_empty() {
        1
    } else {
        list[list.len() - 1].id + 1
    };

    let todo = Todo {
        id,
        content,
        done: false,
    };
    handle_print("added the following item:".to_string());
    todo.print();
    list.push(todo);
}
