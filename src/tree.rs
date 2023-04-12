#![warn(missing_docs)]

/// Empty struct returns when a requested node does not exist in the tree.
#[derive(Debug)]
pub struct NodeDoesNotExist {}

/// A tree with nodes of types Node<T> all stored in the same vector
///
/// # Examples
#[derive(Debug, Default)]
pub struct ArenaTree<T>
where
    T: std::cmp::PartialEq,
{
    arena: Vec<Node<T>>,
}

/// A node in an ArenaTree<T>
#[derive(Debug)]
pub struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    /// Arbitrary value of type T held by Node<T>
    ///
    /// Must implement PartialEq
    pub value: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, value: T) -> Self {
        Self {
            idx,
            value,
            parent: None,
            children: vec![],
        }
    }

    /// Get the index of the node
    pub fn idx(&self) -> usize {
        self.idx
    }

    /// Get the index of the parent node
    ///
    /// Will return None if there is no parent node
    pub fn parent(&self) -> Option<usize> {
        self.parent
    }

    /// Get a vector if indices of the children nodes
    pub fn children(&self) -> &Vec<usize> {
        &self.children
    }
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    /// Initialize a new tree with zero nodes
    pub fn new() -> ArenaTree<T> {
        ArenaTree { arena: Vec::new() }
    }

    /// Get the number of nodes in the tree
    pub fn len(&self) -> usize {
        self.arena.len()
    }

    /// Get node of index idx.
    ///
    /// Will return error if the node index does not exist in the tree
    pub fn get_node(&self, idx: usize) -> Result<&Node<T>, NodeDoesNotExist> {
        let n_nodes = self.arena.len();

        if idx >= n_nodes {
            return Err(NodeDoesNotExist {});
        }
        Ok(self
            .arena
            .get(idx)
            .expect("Arena has at least this many nodes"))
    }

    /// Add node with value of type T to the tree and get the index back
    pub fn add_node(&mut self, value: T) -> usize {
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, value));
        idx
    }

    /// Register node at child_idx as child of node at parent_idx, and vice versa
    ///
    /// Will return error if either index does not exist in the tree.
    pub fn register_parent_node(
        &mut self,
        child_idx: usize,
        parent_idx: usize,
    ) -> Result<(), NodeDoesNotExist> {
        let n_nodes = self.arena.len();
        if (child_idx >= n_nodes) || (parent_idx >= n_nodes) {
            return Err(NodeDoesNotExist {});
        }
        self.arena[child_idx].parent = Some(parent_idx);
        if !self.arena[parent_idx].children.contains(&child_idx) {
            self.arena[parent_idx].children.push(child_idx)
        }
        Ok(())
    }

    /// Add a child node with value of type T to tree with parent index parent_idx
    ///
    /// Will return an error if parent_idx does not exist.
    pub fn add_child_node(
        &mut self,
        parent_idx: usize,
        value: T,
    ) -> Result<usize, NodeDoesNotExist> {
        let n_nodes = self.arena.len();
        if parent_idx >= n_nodes {
            return Err(NodeDoesNotExist {});
        };
        let child_idx = self.add_node(value);
        self.register_parent_node(child_idx, parent_idx)?;
        Ok(child_idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_arena_tree_of_depth_three() {
        let mut tree: ArenaTree<u32> = ArenaTree::new();
        let idx0 = tree.add_node(10);
        let idx1 = tree.add_node(20);
        let idx2 = tree.add_node(21);
        tree.register_parent_node(idx1, idx0).unwrap();
        tree.register_parent_node(idx2, idx0).unwrap();
        let idx3 = tree.add_child_node(idx1, 30).unwrap();

        assert_eq!((idx0, idx1, idx2, idx3), (0, 1, 2, 3));
        assert_eq!(tree.get_node(0).unwrap().value, 10);
        assert_eq!(tree.get_node(1).unwrap().parent().unwrap(), 0);
        assert_eq!(tree.get_node(3).unwrap().parent().unwrap(), 1);
        assert_eq!(tree.get_node(1).unwrap().children(), &vec![3]);
    }
}
