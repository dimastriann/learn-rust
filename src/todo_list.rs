use serde::{Deserialize, Serialize};
use std::fs::{File};
use std::io::{self, Write};
use std::env;

use inquire::Select;

// use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Task {
    id: i32,
    description: String,
    completed: bool,
}

#[derive(Serialize, Deserialize)]
struct TodoList {
    tasks: Vec<Task>,
}

fn store_task() -> io::Result<()> {

    let current_dir = env::current_dir()?;
    println!("Current directory: {}", current_dir.display());

    let file_name = "src\\files\\todo.json";
    let file_path = current_dir.join(file_name);
    println!("File Path: {}", file_path.display());

    // Create a sample to-do list
    let todo_list = TodoList {
        tasks: vec![
            Task {
                id: 1,
                description: String::from("Buy groceries"),
                completed: false,
            },
            Task {
                id: 2,
                description: String::from("Finish Rust project"),
                completed: true,
            },
            Task {
                id: 3,
                description: String::from("Call Alice"),
                completed: false,
            },
            Task {
                id: 4,
                description: String::from("New Task Updated"),
                completed: false,
            },
            Task {
                id: 5,
                description: String::from("New Task 2 Updated"),
                completed: false,
            }
        ],
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&todo_list).expect("Failed to serialize to JSON");
    // Write JSON to File
    let mut file = File::create(&file_path)?;
    file.write_all(json.as_bytes())?;
    println!("To-do list saved to todo.json");
    // if Path::exists(&file_path) {
    //     // let mut file = File::open(&file_path)?;
    //     // file.write_all(json.as_bytes())?;
    //     println!("To-do list updated to todo.json");
    // } else {
    //     // Write JSON to File
    //     let mut file = File::create(&file_path)?;
    //     file.write_all(json.as_bytes())?;
    //     println!("To-do list saved to todo.json");
    // }

    Ok(())
}

pub fn open_todo_options() {
    let options_todo = ["Add Task", "Delete Task"];
    // Open Options Todo-List
    println!("Choose todo list command ?");
    for (index, option) in options_todo.iter().enumerate() {
        println!("{}. {}", index + 1, option);
    }

    print!("Enter choice [1-{}]: ", options_todo.len());
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    match buf.trim().parse::<usize>() {
        Ok(n) if n >= 1 && n <= options_todo.len() => {
            match options_todo[n - 1] {
                "Add Task" => println!("Add task in progress..."),
                "Delete Task" => println!("Delete task in progress..."),
                _ => {}
            }
        }
        _ => println!("Invalid Command !")
    }
}

pub fn open_todo_options_with_inquire() {
    let options_todo: Vec<&str> = vec!["Add Task", "Delete Task"];
    let choice = Select::new("Choose todo list command ?", options_todo)
        .with_help_message("Use ↑/↓ and Enter")
        .prompt();
    match choice {
        Ok("Add Task") => println!("Add task in progress..."),
        Ok("Delete Task") => println!("Delete task in progress..."),
        Ok(_) => {},
        Err(e) => println!("Canceled or error: {}", e)
    }
}
