use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: usize,
    pub value: String,
    parent: Option<usize>,
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
    fn set_active_task(&mut self, id: usize);
    fn print(&self);
    fn print_all(&self);
    fn write(&self) -> Result<()>;
    fn add(&mut self, value: String, parent: Option<usize>, switch: bool);
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

    fn set_active_task(&mut self, id: usize) {
        for task in self.iter_mut() {
            task.active = false;
        }
        if let Some(task) = self.iter_mut().find(|t| t.id == id) {
            if task.children.is_empty() {
                task.active = true;
            }
        } else {
            let leaf_nodes: Vec<usize> = self.get_leaf_descendants(id);
            // TODO: Filter leaf nodes to choose which to make active; could do last touched or first created, etc.
            let active_id = leaf_nodes
                .first()
                .expect("Should be at least one leaf node");
            self.get_mut(*active_id)
                .expect("Index to self is valid")
                .active = true;
        }
    }
    fn print(&self) {
        for task in self.get_root_tasks() {
            task.print(0);
        }
    }

    fn print_all(&self) {
        for task in self.get_root_tasks() {
            tree_print(self, task.id, 0);
        }
    }

    fn write(&self) -> Result<()> {
        // TODO: Error handling
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("/Users/bshaver/.tnt.json")
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
        self.push(task);
        if switch {
            self.set_active_task(id);
        }
    }
}

fn tree_print(tasks: &Vec<Task>, id: usize, indent: usize) {
    if let Some(task) = tasks.get(id) {
        task.print(indent);
        for child in &task.children {
            if !tasks.get(*child).expect("Child ID is valid").done {
                tree_print(tasks, *child, indent + 1);
            }
        }
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
        tasks.set_active_task(0);
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
