
# dotl — Do This Later

A simple CLI todo tracker written in Rust.

## Features

* Add tasks quickly from the terminal
    * Mark as urgent
    * Add a due date
* View your current list of tasks
* Remove tasks by their number
* Persistent storage of tasks in a local JSON file

## Installation

Install the crate
```bash
cargo install dotl
```

## Usage

```bash
# Add new tasks
dotl add "Drink water"
dotl add "Watch Rust tutorial"

# Mark task as urgent
dotl add "Finish report" -u
dotl add "Respond to email" --urgent

# Add a due date
dotl add "Submit assignment" --due "2025-5-18 15:30"

# List all tasks
dotl list

# Remove a task by its number
dotl remove 1
```

## Contributing

Contributions are welcome! Please feel free to open issues for bugs or feature requests,
or submit pull requests to improve the project.
