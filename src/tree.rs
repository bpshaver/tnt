extern crate aoc_utils;
use aoc_utils::tree::ArenaTree;

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
}
