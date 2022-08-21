use std::io;
use std::io::prelude::*;

struct Todo {
    id: i32,
    content: String,
    done: bool,
}

impl Todo {
    fn print(&self) {
        println!("{}", self.id);
        println!("{}", self.content);
        println!("{}", self.done);
    }
}

fn main() {
    let mut todo_list: Vec<Todo> = Vec::new();

    loop {
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
        // println!("{}", command);
        // println!("{:?}", action_body);

        match command.trim() {
            "add" => {
                add_action(&mut todo_list, action_body);
            }
            "done" => {
                done_action(&mut todo_list, 1, true);
            }
            "undone" => {
                done_action(&mut todo_list, 1, false);
            }
            "list" => {
                print_list(&todo_list);
            }
            "help" => help_action(),
            _ => {
                println!("invalid command");
                help_action();
            }
        };
    }
}

fn print_list(list: &[Todo]) {
    for item in list.iter() {
        item.print();
    }
}

fn done_action(list: &mut [Todo], id: i32, done: bool) {
    for item in list.iter_mut() {
        if item.id == id {
            item.done = done;
            let msg = if done { "completed" } else { "undoing" };
            println!("{msg} the following item:");
            item.print();
        }
    }
}

fn add_action(list: &mut Vec<Todo>, content: String) {
    let todo = Todo {
        id: 1,
        content,
        done: false,
    };
    println!("added the following item:");
    todo.print();
    list.push(todo);
}

fn help_action() {
    println!("Available Commmands");
    println!("--------------------------------------------------");
    println!("list: lists all items");
    println!("add <content>: adds an item to the list");
    println!("edit <item_id> <content>: change content of item");
    println!("done <item_id>: completes item");
    println!("undone <item_id>: resets item");
    println!("remove <item_id>: removes an item from the list");
}
