use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

const TASK_FILE: &str = "dotl_tasks.json";

#[derive(Parser)]
#[command(name = "dotl")]
#[command(about = "Do This Later - a simple CLI todo tracker", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// The task description
        task: String,

        // Mark the task as urgent
        #[arg(short, long)]
        urgent: bool,
    },
    /// List all tasks
    List,
    
    /// Remove a task
    Remove {
        index: usize,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    urgent: bool,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { task , urgent} => {
            let mut tasks = load_tasks();
            tasks.push(Task {
                description: task.to_string(),
                urgent: *urgent,
            });
            save_tasks(&tasks).expect("Failed to save tasks");
            println!("Added: {} {}", task, if *urgent { "[URGENT]" } else { "" })
        }
        Commands::List => {
            let tasks = load_tasks();
            if tasks.is_empty() {
                println!("No tasks yet!");
            } else {
                for (i, task) in tasks.iter().enumerate() {
                    println!("{}: {} {}", i + 1, task.description, if task.urgent { "[URGENT]" } else { "" });
                }
            }
        }
        Commands::Remove { index } => {
            let mut tasks = load_tasks();
            if *index == 0 || *index > tasks.len() {
                println!("Invalid task number");
            }
            else {
                let removed = tasks.remove(index-1);
                save_tasks(&tasks).expect("Failed to save tasks");
                println!("Removed {}", removed.description);
            }
        }
    }
}

fn load_tasks() -> Vec<Task> {
    if !Path::new(TASK_FILE).exists() {
        return Vec::new();
    }

    let data = fs::read_to_string(TASK_FILE).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

fn save_tasks(tasks: &[Task]) -> io::Result<()> {
    let json = serde_json::to_string_pretty(tasks)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(TASK_FILE)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}