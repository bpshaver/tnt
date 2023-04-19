mod cli;
mod task;

use crate::cli::{Args, TntCommand};
use crate::task::TaskTree;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("/Users/bshaver/.tnt.json");
    let tasks = task::read_task_list_from_file(path).expect("~/.tnt.json exists");
    let args = Args::parse_args();
    match args.command {
        None => println!("No subcommand provided, showing current task..."),
        Some(command) => match command {
            TntCommand::Add {
                name,
                parent,
                switch,
            } => {
                println!(
                    "Name: {:#?}, parent: {:#?}, switch: {}",
                    name, parent, switch
                )
            }
            TntCommand::View => match tasks.get_active_task() {
                None => println!("No active task"),
                Some(task) => println!("{}", task),
            },
            _ => todo!("Command not supported yet!"),
        },
    }
}
