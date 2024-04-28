/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T: Copy> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        
        match &mut self.root {
            Some(x) =>{ 
                if value < self.root.as_ref().unwrap().value {
                    match self.root.as_mut().unwrap().left {
                        Some(ref mut y) => y.insert(value),
                        None => self.root.as_mut().unwrap().left = Some(Box::new(TreeNode {
                                                                            value: value,
                                                                            left: None,
                                                                            right: None,
                                                                        }))
                    }
                } else if value > self.root.as_ref().unwrap().value{
                    match self.root.as_mut().unwrap().right {
                        Some(ref mut y) => y.insert(value),
                        None => self.root.as_mut().unwrap().right = Some(Box::new(TreeNode {
                                                                            value: value,
                                                                            left: None,
                                                                            right: None,
                                                                        }))
                    }
                }
            },
            None => self.root = Some(Box::new(TreeNode {
                                        value: value,
                                        left: None,
                                        right: None,
                                    }))
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match & self.root {
            None => false,
            Some(ref x) => x.search(value),
        }
    }
}

impl<T: Copy> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        if value < self.value {
            // self.left.unwrap().insert(T); // panic!
            match &mut self.left {
                Some(ref mut x) => x.insert(value),
                None => self.left = Some(Box::new(TreeNode {
                                                    value : value,
                                                    left : None,
                                                    right : None,
                                                }))
            }
        } else {
            // self.right.unwrap().insert(T);
            match &mut self.right {
                Some(x) => x.insert(value),
                None => self.right = Some(Box::new(TreeNode {
                                                    value: value,
                                                    left: None,
                                                    right: None,
                                                }))
            }
        }

    }
    fn search (&self, value: T) -> bool {
        if (self.value == value) {
            return true;
        } else {
            if value < self.value {
                match &self.left {
                    None => {return false;},
                    Some(x) => x.search(value),
                }
            } else {
                match &self.right {
                    None => {return false;},
                    Some(x) => x.search(value),
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


