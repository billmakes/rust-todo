use std::io;
use std::io::prelude::*;

mod action;
use action::Action;

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
        let mut action_str = String::new();

        io::stdin()
            .read_line(&mut action_str)
            .expect("Failed to read action");

        let action = Action::new(action_str);
        if action.command.trim().is_empty() {
            help_action();
        } else {
            // let mut action_vec: Vec<String> =
            //     action.split_whitespace().map(|s| s.to_string()).collect();
            // let command = action_vec.remove(0);
            // let action_body = action_vec.join(" ");
            match action.command.trim() {
                "add" | "a" => {
                    todo_list.add_action(action.body);
                }
                "edit" | "e" => {
                    todo_list.edit_action(action.id, action.body);
                }
                "remove" | "rm" => {
                    todo_list.remove_action(action.id);
                }
                "done" | "d" => {
                    todo_list.done_action(action.id, true);
                }
                "undone" | "ud" => {
                    todo_list.done_action(action.id, false);
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
