mod cli;
mod task;

use crate::cli::{Args, TntCommand};
use crate::task::TaskTree;
use anyhow::Result;
use lets_find_up::find_up;
use std::fs;
use std::io;
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
            TntCommand::Which => {
                println!("{:?}", &path.to_str().expect("Path can go to str"));
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
            TntCommand::Clear { delete } => {
                if delete {
                    fs::remove_file(path).unwrap();
                } else {
                    Vec::new().write(&path).unwrap();
                }
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
            TntCommand::Stdin { parent, current } => {
                let mut id = None;
                if current {
                    if let Some(parent) = tasks.get_active_task() {
                        id = Some(parent.id)
                    }
                } else if parent.is_some() {
                    id = parent
                }
                for line in io::stdin().lines() {
                    let line = line
                        .expect("Expected no non-UTF-8 chars")
                        .trim()
                        .to_string();
                    if !line.is_empty() {
                        tasks.add(line, id, false);
                    }
                }
                tasks.write(&path)?;
            }
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
