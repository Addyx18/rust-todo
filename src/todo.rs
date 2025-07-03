use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{collections::HashMap, io::Read};
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
            title,
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

    let mut content: String = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut task_map: HashMap<String, Vec<Task>> = if content.trim().is_empty() {
        HashMap::new()
    } else {
        serde_json::from_str(&content).unwrap()
    };

    task_map.retain(|_, tasks| {
        tasks.retain(|task| task.id != id);
        !tasks.is_empty()
    });
    let json = serde_json::to_string_pretty(&task_map).unwrap();
    file.set_len(0)?;
    use std::io::Seek;
    use std::io::SeekFrom;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(json.as_bytes()).unwrap();
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

    let mut content: String = String::new();

    file.read_to_string(&mut content).unwrap();

    let mut task_map: HashMap<String, Vec<Task>> = if content.trim().is_empty() {
        HashMap::new()
    } else {
        serde_json::from_str(&content).unwrap()
    };

    for tasks in task_map.values_mut() {
        for task in tasks {
            if task.id == id {
                task.completed = true;
                break;
            }
        }
    }

    let json = serde_json::to_string_pretty(&task_map).unwrap();
    file.set_len(0)?;
    use std::io::Seek;
    use std::io::SeekFrom;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(json.as_bytes()).unwrap();
    println!("Updated todos saved");
    Ok(())
}

pub fn display_todos() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = match std::fs::OpenOptions::new()
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
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let task_map: HashMap<String, Vec<Task>> = if content.trim().is_empty() {
        HashMap::new()
    } else {
        serde_json::from_str(&content).unwrap()
    };
    let now = Utc::now();
    let today_str = now.format("%d-%m-%Y").to_string();
    if !task_map.contains_key(&today_str) {
        println!("No todos today");
    }
    for (i, dates) in task_map.keys().enumerate() {
        if dates == &today_str {
            println!("{}: Today", i + 1);
        } else {
            println!("{}: {}", i + 1, dates);
        }
    }

    let mut user_input: String = String::new();
    print!("Enter your choice: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut user_input).unwrap();
    print!("\n\n");
    let user_input: usize = match user_input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input");
            return Ok(());
        }
    };

    let selected_date = match task_map.keys().nth(user_input - 1) {
        Some(n) => n,
        None => {
            println!("Wrong choice");
            return Ok(());
        }
    };

    if let Some(tasks) = task_map.get(selected_date) {
        for (i, task) in tasks.iter().enumerate() {
            println!("Task: {}", i + 1);
            println!("id: {:?}", task.id);
            println!("title: {:?}", task.title);
            println!("completed: {:?}\n", task.completed);
        }
    } else {
        println!("No tasks for the selected date");
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

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut task_map: HashMap<String, Vec<Task>> = if content.trim().is_empty() {
        HashMap::new()
    } else {
        serde_json::from_str(&content).unwrap()
    };

    let todo = Task::new(title);
    let now = Utc::now();
    task_map
        .entry(now.format("%d-%m-%Y").to_string())
        .or_default()
        .push(todo);

    let json = serde_json::to_string_pretty(&task_map).unwrap();
    file.set_len(0)?;
    use std::io::Seek;
    use std::io::SeekFrom;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(json.as_bytes()).unwrap();
    println!("Todo added successfully!");
    Ok(())
}
