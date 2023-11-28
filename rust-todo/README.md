# Rust Todo List Application

This is a simple command-line to-do list application written in Rust. It allows users to add tasks, mark them as completed, and view the list of tasks. This application demonstrates basic Rust programming concepts including struct usage, file I/O, and command-line argument parsing with Clap.

## Features

- Add new tasks to the to-do list.
- Mark tasks as completed.
- List all the tasks with their status.
- Save tasks to a file and load them when the application starts.

## Getting Started

### Prerequisites

Ensure you have Rust installed on your system. If not, you can install it from [the official Rust website](https://www.rust-lang.org/learn/get-started).

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/RobWilliamson15/rust-baby.git

2. Navigate to the project directory 
    ```sh
    cd rust_todo

### Usage 

Run the application using Cargo:
    ```sh
    cargo run -- [COMMANDS]

Commands:
- Add a new task:
    ```sh
    cargo run -- --add "Task Description"

- Mark a task as completed:
    ```sh
    cargo run -- --complete INDEX

- List all tasks:
    ```sh
    cargo run
