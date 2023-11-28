use clap::{App, Arg, SubCommand};
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
    let matches = App::new("Rust Todo List")
        .version("1.0")
        .author("Robert Williamson")
        .about("Manages a to-do list")
        .subcommand(SubCommand::with_name("add")
            .about("Adds a task to the list")
            .arg(Arg::with_name("DESCRIPTION")
                .help("The description of the task")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("coimplete")
            .about("Marks a task as completed")
            .arg(Arg::with_name("INDEX")
                .help("The index of the task to complete")
                .required(true)
                .index(1)))
        .get_matches();

    let mut tasks = match load_tasks() {
        Ok(loaded_tasks) => loaded_tasks,
        Err(_) => Vec::new(), //Starts with an empty vector if error
    };
    
    if let Some((command, sub_matches)) = matches.subcommand(){
        match command {
            "add" => {
                if let Some(description) = sub_matches.value_of("DESCRIPTION"){
                    add_task(&mut tasks, description.to_string());
                }                    
            },
            "complete" => {
                if let Some(index_str) = sub_matches.value_of("INDEX") {
                    let index = index_str.parse::<usize>()
                        .expect("Please input a valid index");
                    complete_task(&mut tasks, index);
                }
            },
            _ => list_tasks(&tasks),
        }
    } else {
        list_tasks(&tasks);
    }
    
    if let Err(e) = save_tasks(&tasks) {
        println!("Error saving tasks: {}", e);
    }
}
