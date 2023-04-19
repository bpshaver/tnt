use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: usize,
    pub value: String,
    parent: Option<usize>,
    children: Vec<usize>,
    active: bool,
    done: bool,
}

pub trait TaskTree {
    fn get_root_tasks(&self) -> Vec<&Task>;
    fn get_leaf_tasks(&self) -> Vec<&Task>;
    fn get_active_task(&self) -> Option<&Task>;
    fn set_active_task(&mut self, id: usize);
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
    fn get_leaf_tasks(&self) -> Vec<&Task> {
        self.iter()
            .filter(|t| t.children.is_empty() && !t.done)
            .collect()
    }
    fn get_active_task(&self) -> Option<&Task> {
        self.iter().find(|t| t.active)
    }
    fn set_active_task(&mut self, id: usize) {
        self.iter_mut().for_each(|task| task.active = task.id == id);
    }
}

pub fn read_task_list_from_file(file: PathBuf) -> Result<Vec<Task>> {
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
                children: vec![2],
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
        ]
    }
    fn task_list_fixture() -> PathBuf {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("tests/test_data/test_tasks.json");
        file
    }

    #[test]
    fn test_get_root_tasks() {
        assert_eq!(tasks_fixture().get_root_tasks()[0].id, 0);
        assert_eq!(tasks_fixture().get_root_tasks().len(), 1);
    }

    #[test]
    fn test_get_leaf_tasks() {
        assert_eq!(tasks_fixture().get_leaf_tasks()[0].id, 2);
        assert_eq!(tasks_fixture().get_leaf_tasks().len(), 1);
    }
    #[test]
    fn test_get_active_task() {
        assert_eq!(tasks_fixture().get_active_task().unwrap().id, 2);
    }

    #[test]
    fn test_set_active_task() {
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
    fn test_deserialize_simple_task_string() {
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
    fn test_read_lines() {
        let tasks: Vec<Task> = read_task_list_from_file(task_list_fixture()).unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].value, "do my taxes");
        assert_eq!(tasks[1].value, "get w2");
    }
}
