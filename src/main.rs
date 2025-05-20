use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{PathBuf};
use xdg::BaseDirectories;

#[derive(Parser)]
#[command(name = "dotl")]
#[command(about = "Do This Later - a simple CLI todo tracker \nv0.1.5", long_about = None)]
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

        // Due date in format YYYY-MM-DD HH:MM
        #[arg(short, long)]
        due: Option<String>,
    },
    /// List all tasks
    List,

    /// Remove a task
    Remove {
        index: usize,
    },

    /// Export tasks to a file
    Export {
        file_path: String,
    },

    /// Clear all tasks
    Clear,

    /// Backup current list of tasks
    Backup,

    /// Restore tasks from a backup file
    Restore {
        backup_file_path: String,
    }
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
        Commands::Add { task, urgent, due } => {
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

            let due_str = due_date.map_or(String::new(), |d| {
                format!(" [Due: {}]", d.format("%Y-%m-%d @ %H:%M"))
            });

            println!(
                "Added: {} {} {}",
                task,
                if *urgent { "[URGENT]" } else { "" },
                due_str
            );
        }
        Commands::List => {
            let tasks = load_tasks();
            if tasks.is_empty() {
                println!("No tasks yet!");
            } else {
                for (i, task) in tasks.iter().enumerate() {
                    let due_str = task.due_date.map_or(String::new(), |d| {
                        format!(" [Due: {}]", d.format("%Y-%m-%d @ %H:%M"))
                    });

                    println!(
                        "{}: {} {}{}",
                        i + 1,
                        task.description,
                        if task.urgent { "[URGENT]" } else { "" },
                        due_str
                    );
                }
            }
        }
        Commands::Remove { index } => {
            let mut tasks = load_tasks();
            if *index == 0 || *index > tasks.len() {
                println!("Invalid task number");
            } else {
                let removed = tasks.remove(index - 1);
                save_tasks(&tasks).expect("Failed to save tasks");
                println!("Removed {}", removed.description);
            }
        }
        Commands::Export { file_path } => {
            let tasks = load_tasks();
            let mut wtr = csv::Writer::from_path(file_path).expect("Failed to create a CSV writer");

            for task in tasks {
                let due_str = task
                    .due_date
                    .map_or(String::new(), |d| d.format("%Y-%m-%d %H:%M").to_string());
                wtr.write_record(&[
                    &task.description,
                    &task.urgent.to_string(),
                    &due_str,
                ])
                .expect("Failed to write task to CSV");
            }

            wtr.flush().expect("Failed to flush CSV writer");
            println!("Tasks exported to {}", file_path);
        }
        Commands::Clear => {
            println!("Are you sure you want to clear all tasks? (y/n)");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            if input.trim().eq_ignore_ascii_case("y") {
                save_tasks(&Vec::new()).expect("Failed to clear tasks");
                println!("All tasks cleared!");
            } else {
                println!("Clear operation canceled");
            }
        }

        Commands::Backup => {
            let backup_path = create_backup().expect("Failed to create backup");
            println!("Backup created at: {}", backup_path.display());
        }

        Commands::Restore { backup_file_path } => {
            restore_from_backup(backup_file_path).expect("Failed to restore from a backup file");
            println!("Tasks restored from backup: {}", backup_file_path);
        }
    }
}

fn get_task_file_path() -> PathBuf {
    let xdg_dirs = BaseDirectories::with_prefix("dotl").expect("Cannot access XDG directories");
    xdg_dirs
        .place_config_file("dotl_tasks.json")
        .expect("Cannot create config file path")
}

fn load_tasks() -> Vec<Task> {
    let path = get_task_file_path();
    if !path.exists() {
        return Vec::new();
    }

    let data = fs::read_to_string(&path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

fn save_tasks(tasks: &[Task]) -> io::Result<()> {
    let path = get_task_file_path();
    fs::create_dir_all(path.parent().unwrap())?;
    let json = serde_json::to_string_pretty(tasks)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn create_backup() -> io::Result<PathBuf> {
    let task_file_path = get_task_file_path();
    if !task_file_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,"Task file does not exist",
        ));
    }

    let backup_file_name = format!(
        "dotl_tasks_backup{}.json", 
        chrono::Local::now().format("%Y%m%d%H%M%S"));

    let backup_file_path = task_file_path.with_file_name(backup_file_name);
    fs::copy(&task_file_path, &backup_file_path)?;
    Ok(backup_file_path)
}

fn restore_from_backup(backup_file_path: &str) -> io::Result<()> {
    let backup_path = PathBuf::from(backup_file_path);
    if !backup_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Backup file does not exist",
        ));
    }

        let task_file_path = get_task_file_path();
        fs::copy(&backup_path, &task_file_path)?;
        Ok(())
    }