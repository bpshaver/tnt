use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::task::Task;

#[allow(dead_code, unused)]
pub fn find_matching_task<'a>(pattern: &str, choices: &'a Vec<Task>) -> Option<&'a Task> {
    let matcher = SkimMatcherV2::default();
    let mut res = None;
    let mut best_score = 0;
    for choice in choices {
        let score = matcher.fuzzy_match(&choice.value, pattern);
        if let Some(score) = score {
            if score > best_score {
                res = Some(choice);
                best_score = score;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_max_matching_string() {
        let pattern = "foobar";
        let choices = vec![
            Task::new(0, "foobaz".to_string(), None),
            Task::new(1, "foobooz".to_string(), None),
            Task::new(2, "foobar".to_string(), None),
            Task::new(3, "foobarzz".to_string(), None),
            Task::new(4, "qwerty".to_string(), None),
        ];
        assert_eq!(find_matching_task(pattern, &choices).unwrap(), &choices[2]);
    }
}
