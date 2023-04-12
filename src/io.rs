use crate::task::Task;
use anyhow::Result;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

fn read_task_list_from_file(file: PathBuf) -> Result<Vec<Task>> {
    let f = File::open(file)?;
    let reader = BufReader::new(f);
    let task_list: Vec<Task> = serde_json::from_reader(reader)?;
    Ok(task_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task_list_fixture() -> PathBuf {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("tests/test_data/test_tasks.json");
        file
    }

    #[test]
    fn test_read_lines() {
        let tasks: Vec<Task> = read_task_list_from_file(task_list_fixture()).unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].value, "do my taxes");
        assert_eq!(tasks[1].value, "get w2");
    }
}
