use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: usize,
    pub value: String,
    pub parent: Option<usize>,
    children: Vec<usize>,
    pub active: bool,
    pub done: bool,
}

impl Task {
    pub fn print(&self, indent: usize) {
        let ws = (0..indent).map(|_| "  ").collect::<String>();
        println!("{}{} {}", ws, self.id, self)
    }
}

pub trait TaskTree {
    fn get_root_tasks(&self) -> Vec<&Task>;
    fn get_root(&self, id: usize) -> Result<usize>;
    fn get_leaf_tasks(&self) -> Vec<&Task>;
    fn get_leaf_descendants(&self, idx: usize) -> Vec<usize>;
    fn get_active_task(&self) -> Option<&Task>;
    fn get_mut_active_task(&mut self) -> Option<&mut Task>;
    fn set_active_task(&mut self, id: Option<usize>);
    fn print(&self);
    fn print_all(&self);
    fn write(&self) -> Result<()>;
    fn add(&mut self, value: String, parent: Option<usize>, switch: bool);
    fn done(&mut self);
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TaskTree for Vec<Task> {
    fn get_root_tasks(&self) -> Vec<&Task> {
        self.iter()
            .filter(|t| t.parent.is_none() && !t.done)
            .collect()
    }

    fn get_root(&self, id: usize) -> Result<usize> {
        let task = self
            .get(id)
            .context("Cannot find root task for task id not in task list")?;
        match task.parent {
            None => Ok(task.id),
            Some(id) => self.get_root(id),
        }
    }

    fn get_leaf_tasks(&self) -> Vec<&Task> {
        self.iter()
            .filter(|t| t.children.is_empty() && !t.done)
            .collect()
    }
    fn get_leaf_descendants(&self, id: usize) -> Vec<usize> {
        if let Some(task) = self.get(id) {
            if task.done {
                return vec![];
            }
            if task.children.is_empty() {
                return vec![id];
            }
            task.children
                .iter()
                .flat_map(|task_id| self.get_leaf_descendants(*task_id))
                .collect()
        } else {
            vec![]
        }
    }

    fn get_active_task(&self) -> Option<&Task> {
        self.iter().find(|t| t.active)
    }
    fn get_mut_active_task(&mut self) -> Option<&mut Task> {
        self.iter_mut().find(|t| t.active)
    }

    fn set_active_task(&mut self, id: Option<usize>) {
        for task in self.iter_mut() {
            task.active = false;
        }
        let new_task_id = recursive_get_new_active_task(self, id);
        dbg!(new_task_id);
        if let Some(task_id) = new_task_id {
            self.get_mut(task_id).expect("ID is valid").active = true;
        }
    }

    fn print(&self) {
        for task in self.get_root_tasks() {
            task.print(0);
        }
    }

    fn print_all(&self) {
        for task in self.get_root_tasks() {
            recursive_print(self, task.id, 0);
        }
    }

    fn write(&self) -> Result<()> {
        // TODO: Error handling
        let path_var = "TNT_PATH";
        let path_str = match env::var(path_var) {
            Ok(value) => value,
            Err(_) => "Users/bshaver/.tnt.json".to_string(),
        };

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path_str)
            .unwrap();
        // The following truncates the file before writing to it
        file.set_len(0)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, self).unwrap();
        writer.flush().unwrap();
        Ok(())
    }

    fn add(&mut self, value: String, parent: Option<usize>, switch: bool) {
        let id = self.len();
        let task = Task {
            id,
            value,
            parent,
            children: vec![],
            done: false,
            active: false,
        };
        if let Some(parent) = parent {
            // TODO to fix this
            let parent = self.get_mut(parent).expect("Parent ID is valid");
            parent.children.push(id);
        }
        self.push(task);
        if switch {
            self.set_active_task(Some(id));
        }
    }

    fn done(&mut self) {
        let mut parent_id = None;
        if let Some(task) = self.get_mut_active_task() {
            task.done = true;
            parent_id = task.parent;
        }
        self.set_active_task(parent_id)
    }
}

fn recursive_print(tasks: &Vec<Task>, id: usize, indent: usize) {
    if let Some(task) = tasks.get(id) {
        task.print(indent);
        for child in &task.children {
            if !tasks.get(*child).expect("Child ID is valid").done {
                recursive_print(tasks, *child, indent + 1);
            }
        }
    }
}

fn recursive_get_new_active_task(tasks: &Vec<Task>, id: Option<usize>) -> Option<usize> {
    match id {
        Some(id) => {
            let task = tasks.get(id).expect("ID is valid");
            match task
                .children
                .iter()
                .find(|id| !tasks.get(**id).expect("ID is valid").done)
            {
                None => Some(id),
                Some(child_id) => recursive_get_new_active_task(tasks, Some(*child_id)),
            }
        }

        None => match tasks.get_root_tasks().first() {
            None => None,
            Some(task) => recursive_get_new_active_task(tasks, Some(task.id)),
        },
    }
}

pub fn read_task_list_from_file(file: &PathBuf) -> Result<Vec<Task>> {
    let f = File::open(file)?;
    let reader = BufReader::new(f);
    let task_list: Vec<Task> = serde_json::from_reader(reader)?;
    Ok(task_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tasks_fixture() -> Vec<Task> {
        vec![
            Task {
                id: 0,
                value: "foo".to_string(),
                parent: None,
                children: vec![2, 3],
                active: false,
                done: false,
            },
            Task {
                id: 1,
                value: "bar".to_string(),
                parent: None,
                children: vec![],
                active: false,
                done: true,
            },
            Task {
                id: 2,
                value: "baz".to_string(),
                parent: Some(0),
                children: vec![],
                active: true,
                done: false,
            },
            Task {
                id: 3,
                value: "d".to_string(),
                parent: Some(0),
                children: vec![4, 5],
                active: false,
                done: false,
            },
            Task {
                id: 4,
                value: "e".to_string(),
                parent: Some(3),
                children: vec![],
                active: false,
                done: false,
            },
            Task {
                id: 5,
                value: "f".to_string(),
                parent: Some(3),
                children: vec![],
                active: false,
                done: false,
            },
        ]
    }
    fn task_list_fixture() -> PathBuf {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("tests/test_data/test_tasks.json");
        file
    }

    #[test]
    fn get_root_tasks() {
        assert_eq!(tasks_fixture().get_root_tasks()[0].id, 0);
        assert_eq!(tasks_fixture().get_root_tasks().len(), 1);
    }
    #[test]
    fn get_root() {
        assert_eq!(tasks_fixture().get_root(5).unwrap(), 0)
    }

    #[test]
    fn get_leaf_tasks() {
        assert_eq!(
            tasks_fixture()
                .get_leaf_tasks()
                .iter()
                .map(|t| t.id)
                .collect::<Vec<usize>>(),
            vec![2, 4, 5]
        )
    }

    #[test]
    fn get_leaf_descendants() {
        let tasks = tasks_fixture();
        assert_eq!(tasks.get_leaf_descendants(3), vec![4, 5]);
        assert_eq!(tasks.get_leaf_descendants(0), vec![2, 4, 5]);
    }

    #[test]
    fn get_active_task() {
        assert_eq!(tasks_fixture().get_active_task().unwrap().id, 2);
    }

    #[test]
    fn set_active_task() {
        let mut tasks = vec![Task {
            id: 0,
            value: "foo".to_string(),
            parent: None,
            children: vec![],
            done: false,
            active: false,
        }];
        tasks.set_active_task(Some(0));
        assert!(tasks[0].active)
    }

    #[test]
    fn deserialize_simple_task_string() {
        let task_json = r#"
            {
                "id": 5,
                "value": "do my taxes",
                "parent": null,
                "children": [6],
                "active": false,
                "done": false
            }"#;
        let task: Task = serde_json::from_str(task_json).unwrap();
        assert_eq!(task.id, 5);
        assert_eq!(task.value, "do my taxes");
        assert_eq!(task.parent, None);
        assert_eq!(task.children, vec![6]);
        assert!(!task.active);
        assert!(!task.done);
    }

    #[test]
    fn read_lines() {
        let tasks: Vec<Task> = read_task_list_from_file(&task_list_fixture()).unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].value, "do my taxes");
        assert_eq!(tasks[1].value, "get w2");
    }
}
