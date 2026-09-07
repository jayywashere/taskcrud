# taskcrud

> a small Rust CLI task manager because yeah. task. manager. cli. crud. ok.

## About

so, i made this as a small Rust project to practice working with CRUD operations, command-line input, task management, and sorting

the program lets me add, remove, update, and complete tasks, with priority sorting for keeping track of what's more important.. (food.)

it also has a way to dispatch the highest-priority task because apparently regular task lists weren't enough

## Features

- add tasks
- remove tasks
- update task names and priorities
- mark tasks as complete
- list tasks by ID or priority
- ascending and descending sorting
- dispatch the highest-priority task
- input validation

## Requirements

- Rust
- Cargo

## Running

run the program with:

```bash
cargo run
```

## How It Works

tasks can be managed directly through the CLI using the available commands.

each task has a name, priority, and completion status. tasks can be listed by their ID or sorted by priority in either ascending or descending order (i'm either ascending to heaven or descending to hell PLS SAVE ME call 911 immediately)

the highest-priority task can also be dispatched when needed.. why would you do the lowest-priority task..... BYE i'm crying and dying and flying

## Current Scope

- CLI-based task management
- task creation and removal
- task updates
- task completion
- priority sorting
- input validation
- highest-priority task dispatching

more features may be added later

## License

See [LICENSE](LICENSE).
