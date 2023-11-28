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

fn main() {
    let mut tasks = Vec::new();

    add_task(&mut tasks, "Learn Rust".to_string());
    add_task(&mut tasks, "Build a Rust project".to_string());

    list_tasks(&tasks);
}
