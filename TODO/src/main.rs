use std::fs;
use std::io;
use std::io::Write;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool, // New field
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks();
    loop {
        println!("1. Add task");
        println!("2. View tasks");
        println!("3. Edit task");
        println!("4. Delete task");
        println!("5. Save and exit");
        print!("Enter your choice: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                let mut description = String::new();
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut description).expect("Failed to read line");
                let task = Task {
                    id: tasks.len() + 1,
                    description: description.trim().to_string(),
                    completed: false, // New task is not completed by default
                };
                tasks.push(task);
            }
            2 => {
                for task in &tasks {
                    println!("{}: {} [{}]", task.id, task.description, if task.completed { "Completed" } else { "Not Completed" });
                }
            }
            3 => {
                let mut task_id = String::new();
                print!("Enter task ID to edit: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut task_id).expect("Failed to read line");
                let task_id: usize = match task_id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
                    let mut status = String::new();
                    print!("Enter new status (done/not): ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut status).expect("Failed to read line");
                    task.completed = match status.trim().to_lowercase().as_str() {
                        "done" => true,
                        "not " => false,
                        _ => {
                            println!("Invalid status");
                            continue;
                        }
                    };
                } else {
                    println!("Task not found");
                }
            }
            4 => {
                let mut task_id = String::new();
                print!("Enter task ID to delete: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut task_id).expect("Failed to read line");
                let task_id: usize = match task_id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                tasks.retain(|t| t.id != task_id);
            }
            5 => {
                save_tasks(&tasks);
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let serialized = serde_json::to_string(&tasks).unwrap();
    fs::write("tasks.json", serialized).expect("Failed to write file");
}

fn load_tasks() -> Vec<Task> {
    if let Ok(data) = fs::read_to_string("tasks.json") {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}