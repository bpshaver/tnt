mod cli;
mod task;

use crate::cli::{Args, TntCommand};
use crate::task::TaskTree;
use anyhow::Result;
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    let path_var = "TNT_PATH";
    let path = match env::var(path_var) {
        Ok(value) => PathBuf::from(value),
        Err(_) => PathBuf::from("/Users/bshaver/.tnt.json"),
    };
    let mut tasks = task::read_task_list_from_file(&path).expect("~/.tnt.json exists");
    let args = Args::parse_args();
    match args.command {
        None => match tasks.get_active_task() {
            None => println!("No active task"),
            Some(task) => println!("{}", task),
        },
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
            TntCommand::Done => {
                tasks.done();
                tasks.write()?
            }
            TntCommand::First { name } => {
                if let Some(task) = tasks.get_active_task() {
                    tasks.add(name.join(" "), Some(task.id), true);
                    tasks.write()?;
                }
            }
            TntCommand::Also { name, switch } => {
                let parent = tasks.get_active_task().and_then(|t| t.parent);
                tasks.add(name.join(" "), parent, switch);
                tasks.write()?
            }
            TntCommand::Clear => todo!(),
            TntCommand::List { all } => {
                if all {
                    tasks.print_all()
                } else {
                    tasks.print()
                }
            }
            TntCommand::Switch { id } => {
                tasks.set_active_task(Some(id));
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
