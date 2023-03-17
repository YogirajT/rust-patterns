#![allow(dead_code)]

use std::{cmp::Ordering, collections::VecDeque};

struct Node<T: Ord> {
    left_kid: Option<Box<Node<T>>>,
    right_kid: Option<Box<Node<T>>>,
    value: T,
}

struct BinaryTree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left_kid: None,
            right_kid: None,
        }
    }

    fn add(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref mut left_kid) = self.left_kid {
                    left_kid.add(value);
                } else {
                    self.left_kid = Some(Box::new(Node::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(ref mut right_kid) = self.right_kid {
                    right_kid.add(value);
                } else {
                    self.right_kid = Some(Box::new(Node::new(value)));
                }
            }
            Ordering::Equal => {}
        }
    }

    fn find(&self, value: &T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref left_kid) = self.left_kid {
                    left_kid.find(value)
                } else {
                    false
                }
            }
            Ordering::Greater => {
                if let Some(ref right_kid) = self.right_kid {
                    right_kid.find(value)
                } else {
                    false
                }
            }
            Ordering::Equal => true,
        }
    }
}

impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, value: T) {
        if let Some(ref mut root) = self.root {
            root.add(value);
        } else {
            self.root = Some(Box::new(Node::new(value)));
        }
    }

    fn contains(&self, value: &T) -> bool {
        match self.root {
            Some(ref root) => root.find(value),
            None => false,
        }
    }
}

fn build_tree() -> BinaryTree<i32> {
    let mut tree = BinaryTree::new();
    tree.insert(1);
    tree.insert(3);
    tree.insert(2);
    tree.insert(4);
    tree.insert(9);
    tree.insert(8);
    tree.insert(7);

    tree
}

impl<T: Ord> IntoIterator for BinaryTreeIterator<T> {
    type Item = T;
    type IntoIter = BinaryTreeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        BinaryTreeIterator {
            stack: VecDeque::from(vec![self]),
        }
    }
}

struct BinaryTreeIterator<T: Ord> {
    stack: VecDeque<BinaryTree<T>>,
}

impl<T: Ord> Iterator for BinaryTree<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop_front() {
            if let Some(right) = node.right {
                self.stack.push_front(*right);
            }
            if let Some(left) = node.left {
                self.stack.push_front(*left);
            }
            Some(node.data)
        } else {
            None
        }
    }
}
#[cfg(test)]
mod iterator_tests {
    use super::build_tree;

    #[test]
    fn test_iterator() {
        let tree = build_tree();
    }
}
