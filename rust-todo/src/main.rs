use std::fs::File;
use std::io::{self, Write, Read};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}

fn add_task(tasks: &mut Vec<Task>, description: String) {
    let task = Task::new(description);
    tasks.push(task);
}

fn list_tasks(tasks: &Vec<Task>) {
    for (index, task) in tasks.iter().enumerate() {
        println!("{}: {} [{}]", index, task.description, 
                 if task.completed {"done"} else {"not done"});
    }
}

fn complete_task(tasks: &mut Vec<Task>, index: usize){
    if index < tasks.len() {
        tasks[index].complete();
    } else {
        println!("Task index {} is out of bounds.", index);
    }
}

fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let mut file = File::create("tasks.json")
        .expect("Failed to create File");
    let data = serde_json::to_string(tasks)
        .expect("Failed to serialize tasks");
    file.write_all(data.as_bytes())
        .expect("Failed to write to file");
    Ok(())
}

fn load_tasks() -> io::Result<Vec<Task>> {
    let mut file = match File::open("tasks.json") {
        Ok(file) => file,
        Err(_) => return Ok(Vec::new()),
    };

    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Failed to read from file");
    let tasks = serde_json::from_str(&data)
        .expect("Failed to deserialize tasks");
    Ok(tasks)
}

fn main() {
    let mut tasks = match load_tasks() {
        Ok(loaded_tasks) => loaded_tasks,
        Err(_) => Vec::new(), //Starts with an empty vector if error
    };
    
    add_task(&mut tasks, "Learn Rust".to_string());
    add_task(&mut tasks, "Build a Rust project".to_string());
    complete_task(&mut tasks, 0);
    list_tasks(&tasks);
    if let Err(e) = save_tasks(&tasks) {
        println!("Error saving tasks: {}", e);
    }
}
