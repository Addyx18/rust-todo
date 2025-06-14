use serde::{Deserialize, Serialize};
use std::io::{BufReader, BufWriter, Write};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: String,
    title: String,
    completed: bool,
}
fn assign_id() -> String {
    Uuid::new_v4().to_string()
}

impl Task {
    fn new(title: String) -> Self {
        Task {
            id: assign_id(),
            title: title,
            completed: false,
        }
    }
}

pub fn delete_todo() -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter todo id: ");
    std::io::stdout().flush().unwrap();
    let mut id = String::new();

    std::io::stdin()
        .read_line(&mut id)
        .expect("Failed to read input");
    id = id.trim().to_string();
    let mut file = match std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("todos.json")
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error in opening file: {}", e);
            return Err(Box::new(e));
        }
    };

    let reader = BufReader::new(&file);

    let mut todos: Vec<Task> = match serde_json::from_reader(reader) {
        Ok(f) => f,
        Err(_) => Vec::new(),
    };

    todos.retain(|todo| todo.id != id);
    file.set_len(0)?;
    use std::io::Seek;
    use std::io::SeekFrom;
    file.seek(SeekFrom::Start(0))?;
    let writer = BufWriter::new(&file);
    serde_json::to_writer_pretty(writer, &todos)?;
    println!("Deleted Successfully");
    Ok(())
}

pub fn mark_completed() -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter todo id: ");
    std::io::stdout().flush().unwrap();
    let mut id = String::new();

    std::io::stdin()
        .read_line(&mut id)
        .expect("Failed to read input");
    id = id.trim().to_string();
    let mut file = match std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("todos.json")
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error in opening file: {}", e);
            return Err(Box::new(e));
        }
    };

    let reader = BufReader::new(&file);

    let mut todos: Vec<Task> = match serde_json::from_reader(reader) {
        Ok(f) => f,
        Err(_) => Vec::new(),
    };

    match todos.iter_mut().find(|task| task.id == id) {
        Some(task) => {
            task.completed = true;
            println!("Marked task completed");
        }

        None => {
            println!("No task found with id {}", id);
            return Ok(());
        }
    }
    file.set_len(0)?;
    use std::io::Seek;
    use std::io::SeekFrom;
    file.seek(SeekFrom::Start(0))?;
    let writer = BufWriter::new(&file);
    serde_json::to_writer_pretty(writer, &todos)?;
    println!("Updated todos saved");
    Ok(())
}

pub fn display_todos() -> Result<(), Box<dyn std::error::Error>> {
    let file = match std::fs::OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open("todos.json")
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Hello {}", e);
            return Err(Box::new(e));
        }
    };
    let reader = BufReader::new(&file);

    let todos: Vec<Task> = match serde_json::from_reader(reader) {
        Ok(t) => t,
        Err(_) => Vec::new(),
    };
    for todo in todos {
        let content: String = serde_json::to_string_pretty(&todo)?;
        println!("{}", content);
    }
    Ok(())
}

pub fn add_todo() -> Result<(), Box<dyn std::error::Error>> {
    let mut title: String = String::new();
    print!("Todo title: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut title)
        .expect("Failed to read");
    title = title.trim().to_string();

    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("todos.json")?;

    let reader = BufReader::new(&file);

    let mut todos: Vec<Task> = match serde_json::from_reader(reader) {
        Ok(t) => t,
        Err(_) => Vec::new(),
    };

    let todo = Task::new(title);

    todos.push(todo);
    file.set_len(0)?;
    use std::io::Seek;
    use std::io::SeekFrom;
    file.seek(SeekFrom::Start(0))?;
    let writer = BufWriter::new(&file);
    serde_json::to_writer_pretty(writer, &todos)?;

    println!("Todo added successfully!");
    Ok(())
}
