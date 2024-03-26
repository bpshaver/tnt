use clap::{Parser, Subcommand};

/// TNT interactive todo list
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<TntCommand>,
}

#[derive(Subcommand, Debug)]
pub enum TntCommand {
    /// Mark the current task done
    Done,
    /// Show the JSON file used to store tasks
    Which,
    /// Add task
    Add {
        // Name of the task to add
        name: Vec<String>,
        // ID of parent task
        #[arg(short, long)]
        parent: Option<usize>,
        // Switch to the new task
        #[arg(short, long)]
        switch: bool,
    },
    /// Add blocking subtask and switch to it
    First {
        // Name of the task to add
        name: Vec<String>,
    },
    /// Add sibling (non-blocking) task
    Also {
        // Name of the task to add
        name: Vec<String>,
        // Switch to the new task
        #[arg(short, long)]
        switch: bool,
    },
    /// Clear all tasks and subtasks
    Clear {
        /// Delete the local JSON file as well
        #[arg(short, long)]
        delete: bool,
    },
    /// List tasks
    List {
        /// List tasks and subtasks
        #[arg(short, long)]
        all: bool,
    },
    /// View the current task
    View,
    /// Add new tasks from stdin
    Stdin {
        // ID of parent task. Overrrides --current
        #[arg(short, long)]
        parent: Option<usize>,
        // Add tasks from stdin to current task
        #[arg(short, long)]
        current: bool,
    },
    /// List all actionable (non-blocked) subtasks for current root task
    Local,
    /// Switch to task
    Switch {
        // ID of task to switch to
        id: usize,
    },
    /// Init new tnt list in current directory
    Init,
    /// Interactive switch
    Iswitch,
    /// Get current task ID
    Id,
}

impl Args {
    pub fn parse_args() -> Args {
        Args::parse()
    }
}
