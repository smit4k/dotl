use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
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

        // Due date in format YYYY-MM-DD
        #[arg(short, long)]
        due: Option<String>,
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
    due_date: Option<NaiveDateTime>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { task , urgent, due} => {
            let due_date = due.as_ref().and_then(|d| {
                NaiveDateTime::parse_from_str(d, "%Y-%m-%d %H:%M").ok()
            });

            let mut tasks = load_tasks();
            tasks.push(Task {
                description: task.to_string(),
                urgent: *urgent,
                due_date,
            });

            save_tasks(&tasks).expect("Failed to save tasks");

            let due_str = due_date.map_or(String::new(), |d| 
                format!(" [Due: {}]", d.format("%Y-%m-%d @ %H:%M"))
            );

            println!("Added: {} {} {}", task, if *urgent { "[URGENT]" } else { "" }, due_str)
        }
        Commands::List => {
            let tasks = load_tasks();
            if tasks.is_empty() {
                println!("No tasks yet!");
            } else {
                for (i, task) in tasks.iter().enumerate() {

                    let due_str = task.due_date.map_or(String::new(), |d| 
                        format!(" [Due: {}]", d.format("%Y-%m-%d @ %H:%M"))
                    );              

                    println!("{}: {} {}{}", i + 1, task.description, if task.urgent { "[URGENT]" } else { "" }, due_str);
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