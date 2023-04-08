use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn read_lines(file: PathBuf) -> impl Iterator<Item = String> {
    let f = File::open(file).unwrap();
    BufReader::new(f)
        .lines()
        .filter(|l| l.is_ok())
        .map(|l| l.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines_fixture() -> PathBuf {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("tests/test_data/test_lines.txt");
        file
    }

    #[test]
    fn test_read_lines() {
        let lines: Vec<String> = read_lines(lines_fixture()).collect();
        assert_eq!(lines, vec!["foo", "bar", "baz"])
    }
}
