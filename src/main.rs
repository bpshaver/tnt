mod cli;
mod task;

use crate::cli::{Args, TntCommand};
use crate::task::TaskTree;
use anyhow::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    let path = PathBuf::from("/Users/bshaver/.tnt.json");
    let mut tasks = task::read_task_list_from_file(&path).expect("~/.tnt.json exists");
    let args = Args::parse_args();
    match args.command {
        None => println!("No subcommand provided, showing current task..."),
        Some(command) => match command {
            TntCommand::Add {
                name,
                parent,
                switch,
            } => {
                tasks.add(name.join(" "), parent, switch);
                tasks.write()?
            }
            TntCommand::View => match tasks.get_active_task() {
                None => println!("No active task"),
                Some(task) => println!("{}", task),
            },
            TntCommand::Done => todo!(),
            TntCommand::First { name } => {
                if let Some(task) = tasks.get_active_task() {
                    tasks.add(name.join(" "), Some(task.id), true);
                    tasks.write()?;
                }
            }
            #[allow(unused)]
            TntCommand::Also { name, switch } => todo!(),
            TntCommand::Clear => todo!(),
            TntCommand::List { all } => {
                if all {
                    tasks.print_all()
                } else {
                    tasks.print()
                }
            }
            TntCommand::Switch { id } => {
                tasks.set_active_task(id);
                tasks.write()?;
            }
            #[allow(unused)]
            TntCommand::Stdin { parent, current } => todo!(),
            TntCommand::Local => {
                if let Some(task) = tasks.get_active_task() {
                    let root_task = tasks
                        .get_root(task.id)
                        .expect("Valid task is guaranteed to have root");
                    let leaf_nodes = tasks.get_leaf_descendants(root_task);
                    for task_id in leaf_nodes {
                        let task = tasks.get(task_id).expect("Task id valid");
                        if !task.done {
                            println!("{} {}", task_id, task)
                        }
                    }
                }
            }
        },
    }
    Ok(())
}
