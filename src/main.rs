mod cli;
mod task;

use crate::cli::{Args, TntCommand};
use crate::task::TaskTree;
use anyhow::Result;
use lets_find_up::find_up;
use std::path::PathBuf;

fn get_path() -> PathBuf {
    match find_up(".tnt.json").expect("find_up succeeds") {
        None => {
            let path = PathBuf::from("/Users/bshaver/.tnt.json");
            Vec::new().write(&path).unwrap();
            path
        }
        Some(path) => path,
    }
}

fn main() -> Result<()> {
    let args = Args::parse_args();
    let path = get_path();
    let mut tasks = task::read_task_list_from_file(&path).expect("Path should be valid");
    match args.command {
        None => match tasks.get_active_task() {
            None => {
                println!("No active task")
            }
            Some(task) => {
                println!("{}", task)
            }
        },

        Some(command) => match command {
            TntCommand::Init => {
                Vec::new().write(&PathBuf::from(".tnt.json")).unwrap();
            }
            TntCommand::Add {
                name,
                parent,
                switch,
            } => {
                tasks
                    .add(name.join(" "), parent, switch)
                    .write(&path)
                    .expect("Write works");
            }
            TntCommand::View => match tasks.get_active_task() {
                None => println!("No active task"),
                Some(task) => println!("{}", task),
            },
            TntCommand::Done => {
                tasks.done().write(&path)?;
            }
            TntCommand::First { name } => {
                if let Some(task) = tasks.get_active_task() {
                    tasks
                        .add(name.join(" "), Some(task.id), true)
                        .write(&path)?;
                }
            }
            TntCommand::Also { name, switch } => {
                let parent = tasks.get_active_task().map(|task| task.id);
                tasks.add(name.join(" "), parent, switch).write(&path)?;
            }
            TntCommand::Clear => {
                Vec::new().write(&path).unwrap();
            }
            TntCommand::List { all } => {
                if all {
                    tasks.print_all()
                } else {
                    tasks.print()
                }
            }
            TntCommand::Switch { id } => {
                tasks.set_active_task(Some(id)).write(&path)?;
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
