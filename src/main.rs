mod cli;
mod task;
mod utils;

use crate::cli::{Args, TntCommand};
use crate::task::TaskTree;
use crate::utils::find_matching_task;
use anyhow::Result;
use file_lookup::home_find_file;
use home::home_dir;
use inquire::Select;
use log::trace;
use std::fs;
use std::io;
use std::path::PathBuf;

fn get_path() -> PathBuf {
    trace!("Looking for the TNT JSON path");
    match home_find_file(".tnt.json").expect("home_find_file succeeds") {
        None => {
            trace!("No JSON file found. Creating one...");
            let mut path: PathBuf = home_dir().expect("Can find home_dir");
            path.push(".tnt.json");
            Vec::new().write(&path).unwrap();
            path
        }
        Some(path) => {
            trace!("Using JSON file {:?}", &path);
            path
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();
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
                println!("{}", path.display());
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
            TntCommand::Id => match tasks.get_active_task() {
                Some(task) => println!("{}", task.id),
                _ => {}
            },
            TntCommand::Done => {
                tasks.done().write(&path)?;
            }
            TntCommand::First { name } => {
                let parent = tasks.get_active_task().map(|task| task.id);
                tasks.add(name.join(" "), parent, true).write(&path)?;
            }
            TntCommand::Also { name, switch } => {
                let parent = match tasks.get_active_task() {
                    None => None,
                    Some(task) => task.parent,
                };
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
            TntCommand::Iswitch => {
                if let Ok(task) = Select::new("Select?", tasks.iter().filter(|t| !t.done).collect())
                    .with_vim_mode(true)
                    .without_help_message()
                    .prompt()
                {
                    tasks.set_active_task(Some(task.id)).write(&path)?
                };
            }
            TntCommand::Do { name } => {
                let name = name.join(" ");
                let task = find_matching_task(&name, &tasks);
                if let Some(task) = task {
                    tasks.set_active_task(Some(task.id)).write(&path)?
                }
            }
        },
    }
    Ok(())
}
