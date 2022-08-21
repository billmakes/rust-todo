use std::fs::File;
use std::io;
use std::io::prelude::*;

mod todo;
use todo::add_action;
use todo::done_action;
use todo::edit_action;
use todo::print_list;
use todo::remove_action;
use todo::Todo;

fn main() -> Result<(), std::io::Error> {
    let file_path = "./db/db.json";

    let mut todo_list: Vec<Todo> = match File::open(file_path) {
        Ok(file) => serde_json::from_reader(file).expect("error while reading"),
        _ => Vec::new(),
    };

    loop {
        println!("---------------------------------------");
        print!("Enter command: ");
        let _ = io::stdout().flush();
        let mut action = String::new();

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read action");

        if action.trim().is_empty() {
            help_action();
        } else {
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
