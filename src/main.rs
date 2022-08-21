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

mod handle_print;
use handle_print::handle_print;

fn main() -> Result<(), std::io::Error> {
    let file_path = "./db/db.json";

    let mut todo_list: Vec<Todo> = match File::open(file_path) {
        Ok(file) => serde_json::from_reader(file).expect("error while reading"),
        _ => Vec::new(),
    };

    loop {
        handle_print("---------------------------------------".to_string());
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
                        _ => handle_print("invalid id".to_string()),
                    }
                }
                "remove" | "rm" => {
                    let id_str = action_vec.remove(0);
                    match id_str.parse::<usize>() {
                        Ok(id) => remove_action(&mut todo_list, id),
                        _ => handle_print("invalid id".to_string()),
                    }
                }
                "done" | "d" => {
                    let id_str = action_vec.remove(0);
                    match id_str.parse::<usize>() {
                        Ok(id) => done_action(&mut todo_list, id, true),
                        _ => handle_print("invalid id".to_string()),
                    }
                }
                "undone" | "ud" => {
                    let id_str = action_vec.remove(0);
                    match id_str.parse::<usize>() {
                        Ok(id) => done_action(&mut todo_list, id, false),
                        _ => handle_print("invalid id".to_string()),
                    }
                }
                "list" | "ls" => {
                    print_list(&todo_list);
                }
                "quit" | "q" => break,
                "help" | "h" => help_action(),
                _ => {
                    handle_print("invalid command".to_string());
                    help_action();
                }
            };
        }
    }

    std::fs::write(file_path, serde_json::to_string_pretty(&todo_list).unwrap())?;
    Ok(())
}

fn help_action() {
    handle_print("Available Commmands".to_string());
    handle_print("--------------------------------------------------".to_string());
    handle_print("(help | h): prints this help menu".to_string());
    handle_print("(list | ls): lists all items".to_string());
    handle_print("(add | a ) <content>: adds an item to the list".to_string());
    handle_print("(edit | e) <item_id> <content>: change content of item".to_string());
    handle_print("(done | d) <item_id>: completes item".to_string());
    handle_print("(undone | ud) <item_id>: resets item".to_string());
    handle_print("(remove | rm) <item_id>: removes an item from the list".to_string());
    handle_print("(quit | q): exits the program".to_string());
}
