use serde::{Deserialize, Serialize};
use std::fs::{File};
use std::io::{self, Write};
use std::env;
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

pub fn store_task() -> io::Result<()> {

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