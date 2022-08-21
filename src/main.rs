use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct Todo {
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

fn main() -> Result<(), std::io::Error> {
    let file_path = "./db/db.json";
    let file = File::open(file_path).expect("File not found");
    let mut todo_list: Vec<Todo> = serde_json::from_reader(file).expect("error while reading");
    loop {
        println!("---------------------------------------");
        print!("Enter command: ");
        let _ = io::stdout().flush();
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read action");

        let mut action_vec: Vec<String> =
            action.split_whitespace().map(|s| s.to_string()).collect();
        let command = action_vec.remove(0);
        let action_body = action_vec.join(" ");

        match command.trim() {
            "add" | "a" => {
                add_action(&mut todo_list, action_body);
            }
            "edit" | "e" => {
                let id_str = action_vec.remove(0);
                match id_str.parse::<usize>() {
                    Ok(id) => edit_action(&mut todo_list, id, action_vec.join(" ")),
                    _ => println!("invalid id"),
                }
            }
            "remove" | "rm" => {
                let id_str = action_vec.remove(0);
                match id_str.parse::<usize>() {
                    Ok(id) => remove_action(&mut todo_list, id),
                    _ => println!("invalid id"),
                }
            }
            "done" | "d" => {
                let id_str = action_vec.remove(0);
                match id_str.parse::<usize>() {
                    Ok(id) => done_action(&mut todo_list, id, true),
                    _ => println!("invalid id"),
                }
            }
            "undone" | "ud" => {
                let id_str = action_vec.remove(0);
                match id_str.parse::<usize>() {
                    Ok(id) => done_action(&mut todo_list, id, false),
                    _ => println!("invalid id"),
                }
            }
            "list" | "ls" => {
                print_list(&todo_list);
            }
            "quit" | "q" => break,
            "help" | "h" => help_action(),
            _ => {
                println!("invalid command");
                help_action();
            }
        };
    }
    std::fs::write(file_path, serde_json::to_string_pretty(&todo_list).unwrap())?;
    Ok(())
}

fn help_action() {
    println!("Available Commmands");
    println!("--------------------------------------------------");
    println!("(help | h): prints this help menu");
    println!("(list | ls): lists all items");
    println!("(add | a ) <content>: adds an item to the list");
    println!("(edit | e) <item_id> <content>: change content of item");
    println!("(done | d) <item_id>: completes item");
    println!("(undone | ud) <item_id>: resets item");
    println!("(remove | rm) <item_id>: removes an item from the list");
    println!("(quit | q): exits the program");
}

fn print_list(list: &[Todo]) {
    for item in list.iter() {
        item.print();
    }
}

fn remove_action(list: &mut Vec<Todo>, id: usize) {
    println!("removing item with id of {}", id);
    list.retain_mut(|i| i.id != id);
}

fn edit_action(list: &mut [Todo], id: usize, content: String) {
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

fn done_action(list: &mut [Todo], id: usize, done: bool) {
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

fn add_action(list: &mut Vec<Todo>, content: String) {
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
