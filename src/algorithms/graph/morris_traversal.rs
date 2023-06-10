use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

pub fn morris_traversal<T>(root: Option<Rc<RefCell<Node<T>>>>)
where
    T: std::fmt::Debug + std::cmp::PartialEq,
{
    let mut current = root;

    while let Some(node) = current {
        if node.borrow().left.is_none() {
            println!("{:?}", node.borrow().value);
            current = node.borrow().right.clone();
        } else {
            let mut predecessor = node.borrow().left.clone().unwrap();
            while predecessor.clone().borrow().right.is_some()
                && predecessor.as_ref().borrow().right.as_ref().unwrap().borrow().value != node.borrow().value
            {
                predecessor = predecessor.clone().borrow().right.clone().unwrap();
            }

            if predecessor.borrow().right.is_none() {
                predecessor.borrow_mut().right = Some(node.clone());
                current = node.borrow().left.clone();
            } else {
                println!("{:?}", node.borrow().value);
                predecessor.borrow_mut().right = None;
                current = node.borrow().right.clone();
            }
        }
    }
}

#[cfg(test)]
mod morris_tests {
    use std::{cell::RefCell, rc::Rc};

    use super::{Node, morris_traversal};

    #[test]
    fn morris_test() {
        // Create a sample binary tree
        let root = Rc::new(RefCell::new(Node::new(1)));
        let node2 = Rc::new(RefCell::new(Node::new(2)));
        let node3 = Rc::new(RefCell::new(Node::new(3)));
        let node4 = Rc::new(RefCell::new(Node::new(4)));
        let node5 = Rc::new(RefCell::new(Node::new(5)));

        root.borrow_mut().left = Some(node2.clone());
        root.borrow_mut().right = Some(node3);
        node2.borrow_mut().left = Some(node4);
        node2.borrow_mut().right = Some(node5);

        // Perform Morris Traversal
        morris_traversal(Some(root));
    }
}
