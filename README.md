# TNT

`tnt` stands for The Next Thing.

## Installation

Install `cargo`
```bash
$ cargo install --git https://github.com/bpshaver/tnt
```

## Usage

```trycmd
$ tnt --help
TNT interactive todo list

Usage: tnt [COMMAND]

Commands:
  done    Mark the current task done
  which   Show the JSON file used to store tasks
  add     Add task
  first   Add blocking subtask and switch to it
  also    Add sibling (non-blocking) task
  clear   Clear all tasks and subtasks
  list    List tasks
  view    View the current task
  stdin   Add new tasks from stdin
  local   List all actionable (non-blocked) subtasks for current root task
  switch  Switch to task
  init    Init new tnt list in current directory
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```
