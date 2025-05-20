![Crates.io Version](https://img.shields.io/crates/v/dotl?style=for-the-badge&labelColor=%23000000)
![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/dotl?style=for-the-badge&labelColor=%23000000)
![Crates.io License](https://img.shields.io/crates/l/dotl?style=for-the-badge&labelColor=%23000000)

# dotl — Do This Later

A simple CLI todo tracker written in Rust.

## Features

* Add tasks quickly from the terminal
    * Mark as urgent
    * Add a due date
* View your current list of tasks
* Remove tasks by their number
* Export your tasks
* Persistent storage of tasks across directories

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

# Export your tasks
dotl export mytasks.csv

# Clear all of your tasks
dotl clear
```

## Contributing

Contributions are welcome! Please feel free to open issues for bugs or feature requests,
or submit pull requests to improve the project.
