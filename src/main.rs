use std::io;
use std::io::prelude::*;

mod todo;
use todo::TodoList;

mod handle_print;
use handle_print::handle_print;

fn main() -> Result<(), std::io::Error> {
    let file_path = "./db/db.json";

    let mut todo_list = TodoList::new(Some(file_path));

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
                    todo_list.add_action(action_body);
                }
                "edit" | "e" => {
                    let id_str = action_vec.remove(0);
                    match id_str.parse::<usize>() {
                        Ok(id) => todo_list.edit_action(id, action_vec.join(" ")),
                        _ => handle_print("invalid id".to_string()),
                    }
                }
                "remove" | "rm" => {
                    let id_str = action_vec.remove(0);
                    match id_str.parse::<usize>() {
                        Ok(id) => todo_list.remove_action(id),
                        _ => handle_print("invalid id".to_string()),
                    }
                }
                "done" | "d" => {
                    let id_str = action_vec.remove(0);
                    match id_str.parse::<usize>() {
                        Ok(id) => todo_list.done_action(id, true),
                        _ => handle_print("invalid id".to_string()),
                    }
                }
                "undone" | "ud" => {
                    let id_str = action_vec.remove(0);
                    match id_str.parse::<usize>() {
                        Ok(id) => todo_list.done_action(id, false),
                        _ => handle_print("invalid id".to_string()),
                    }
                }
                "list" | "ls" => {
                    todo_list.print_list();
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

    std::fs::write(
        file_path,
        serde_json::to_string_pretty(&todo_list.list).unwrap(),
    )?;
    Ok(())
}

fn help_action() {
    handle_print(
        "Available Commmands\n\
        --------------------------------------------------\n\
        (help | h): prints this help menu\n\
        (list | ls): lists all items\n\
        (add | a ) <content>: adds an item to the list\n\
        (edit | e) <item_id> <content>: change content of item\n\
        (done | d) <item_id>: completes item\n\
        (undone | ud) <item_id>: resets item\n\
        (remove | rm) <item_id>: removes an item from the list\n\
        (quit | q): exits the program"
            .to_string(),
    );
}
