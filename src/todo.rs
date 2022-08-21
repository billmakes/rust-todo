use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    id: usize,
    content: String,
    done: bool,
}

impl Todo {
    fn print(&self) {
        println!("---------------------------------------");
        println!("id: {}, done: {}", self.id, self.done);
        println!("{}", self.content);
    }
}

pub fn print_list(list: &[Todo]) {
    for item in list.iter() {
        item.print();
    }
}

pub fn remove_action(list: &mut Vec<Todo>, id: usize) {
    println!("removing item with id of {}", id);
    list.retain_mut(|i| i.id != id);
}

pub fn edit_action(list: &mut [Todo], id: usize, content: String) {
    for item in list.iter_mut() {
        if item.id == id {
            item.content = content;
            println!("edited the following item:");
            item.print();
            break;
        } else {
            println!("item with id: {} doesn't exist!", id);
        }
    }
}

pub fn done_action(list: &mut [Todo], id: usize, done: bool) {
    for item in list.iter_mut() {
        if item.id == id {
            item.done = done;
            let msg = if done { "completed" } else { "undoing" };
            println!("{msg} the following item:");
            item.print();
            break;
        } else {
            println!("item with id: {} doesn't exist!", id);
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
    println!("added the following item:");
    todo.print();
    list.push(todo);
}
