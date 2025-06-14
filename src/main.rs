mod todo;
use crate::todo::{add_todo, delete_todo, display_todos, mark_completed};
use std::io::Write;

enum ClientOption {
    ShowTodos,
    AddNew,
    DeleteTodo,
    MarkCompleted,
}

fn main() {
    print!(
        "1- Show todos\
        \n2- Add new\
        \n3- Delete todo\
        \n4- Mark Completed\n\n"
    );

    print!("Enter your choice(1, 2, 3 or 4): ");
    std::io::stdout().flush().unwrap();
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let user_choice = match input.trim().parse::<u8>() {
        Ok(choice) => choice,
        Err(e) => {
            eprintln!("Parsing Error: {}", e);
            return;
        }
    };

    let option: ClientOption = match user_choice {
        1 => ClientOption::ShowTodos,
        2 => ClientOption::AddNew,
        3 => ClientOption::DeleteTodo,
        4 => ClientOption::MarkCompleted,
        _ => {
            println!("Invalid Choice");
            return;
        }
    };

    let _result = match option {
        ClientOption::ShowTodos => display_todos(),
        ClientOption::AddNew => add_todo(),
        ClientOption::DeleteTodo => delete_todo(),
        ClientOption::MarkCompleted => mark_completed(),
    };
}
