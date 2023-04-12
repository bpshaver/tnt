use aoc_utils::tree::ArenaTree;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::default::Default;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: usize,
    pub value: String,
    parent: Option<usize>,
    children: Vec<usize>,
    active: bool,
    done: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_empty_arena_tree_and_add_one_node() {
        let mut tree: ArenaTree<isize> = ArenaTree::new();
        assert_eq!(tree.len(), 0);
        tree.add_node(42);
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn test_can_init_default_empty_task_list() {
        let tasks: Vec<Task> = Vec::default();
        assert_eq!(tasks.len(), 0)
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
        assert_eq!(task.active, false);
        assert_eq!(task.done, false);
    }
}
