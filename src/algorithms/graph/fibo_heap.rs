#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Clone)]
struct Node<T>
where T: Copy {
    pub value: T,
    degree: usize,
    marked: bool,
    parent: Option<Rc<RefCell<Node<T>>>>,
    child: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Copy> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            degree: 0,
            marked: false,
            parent: None,
            child: None,
            prev: None,
            next: None,
        }
    }
}

struct FibonacciHeap<T: Copy> {
    min_node: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}


impl<T: Ord + Copy> FibonacciHeap<T> {
    fn new() -> Self {
        FibonacciHeap {
            min_node: None,
            size: 0,
        }
    }

    fn insert(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        self.min_node = self.merge_nodes(self.min_node.clone(), Some(new_node.clone()));
        self.size += 1;
    }

    fn merge_nodes(
        &mut self,
        mut node1: Option<Rc<RefCell<Node<T>>>>,
        mut node2: Option<Rc<RefCell<Node<T>>>>,
    ) -> Option<Rc<RefCell<Node<T>>>> {
        match (node1.take(), node2.take()) {
            (Some(n1), None) => Some(n1.clone()),
            (None, Some(n2)) => Some(n2.clone()),
            (Some(n1), Some(n2)) => {
                if n1.borrow().value < n2.borrow().value {
                    n1.borrow_mut().next = n2.borrow().next.clone();
                    if let Some(next) = &n1.borrow().next {
                        next.borrow_mut().prev = Some(n1.clone());
                    }
                    n2.borrow_mut().prev = None;
                    n2.borrow_mut().next = Some(n2.clone());
                    n1.borrow_mut().child = self.merge_nodes(n1.borrow().child.clone(), Some(n2));
                    n1.borrow_mut().degree += 1;
                    Some(n1.clone())
                } else {
                    n2.borrow_mut().next = n1.borrow().next.clone();
                    if let Some(next) = &n2.borrow().next {
                        next.borrow_mut().prev = Some(n2.clone());
                    }
                    n1.borrow_mut().prev = None;
                    n1.borrow_mut().next = Some(n1.clone());
                    n2.borrow_mut().child = self.merge_nodes(n2.borrow().child.clone(), Some(n1));
                    n2.borrow_mut().degree += 1;
                    Some(n2.clone())
                }
            }
            (None, None) => None,
        }
    }

fn consolidate(&mut self) {
    let mut degree_table: Vec<Option<Rc<RefCell<Node<T>>>>> = vec![None; self.size];
    let mut nodes = Vec::new();

    while let Some(node) = self.min_node.take() {
        nodes.push(node.clone());
        let mut current = node.borrow_mut().next.clone();
        while let Some(next) = current {
            nodes.push(next.clone());
            current = next.borrow_mut().next.clone();
        }
    }

    for node in nodes {
        let mut current = Some(node.clone());
        let mut degree = current.as_ref().unwrap().borrow().degree;

        while let Some(sibling) = degree_table[degree].clone() {
            if current.as_ref().unwrap().borrow().value > sibling.borrow().value {
                let temp = current.clone();
                current = Some(sibling.clone());
                degree_table[degree] = temp.clone();
            }
            self.link_nodes(sibling.clone(), current.clone());

            degree_table[degree] = None;
            degree += 1;
        }

        degree_table[degree] = current.clone();
    }

    self.min_node = None;

    for node in degree_table {
        if let Some(n) = node {
            if self.min_node.is_none() || n.borrow().value < self.min_node.as_ref().unwrap().borrow().value {
                self.min_node = Some(n.clone());
            }
        }
    }
}
    fn link_nodes(
        &mut self,
        mut child: Rc<RefCell<Node<T>>>,
        mut parent: Option<Rc<RefCell<Node<T>>>>,
    ) {
        child.borrow_mut().prev = None;
        child.borrow_mut().next = None;
        child.borrow_mut().marked = false;

        if parent.is_none() {
            parent = Some(child.clone());
        } else {
            parent.as_mut().unwrap().borrow_mut().next = Some(child.clone());
            child.borrow_mut().prev = parent.clone();

            let mut parent_child = parent.as_ref().unwrap().borrow_mut().child.clone();
            parent_child = self.merge_nodes(parent_child, Some(child.clone()));
            parent.as_mut().unwrap().borrow_mut().child = parent_child;
            parent.as_mut().unwrap().borrow_mut().degree += 1;
        }

        child.borrow_mut().parent = parent;

        if self.min_node.is_none() || child.borrow().value < self.min_node.as_ref().unwrap().borrow().value {
            self.min_node = Some(child);
        }
    }

    fn extract_min(&mut self) -> Option<T> {
        let min_node = self.min_node.clone();

        if let Some(node) = min_node {
            let mut child = node.borrow().child.clone();
            let mut current = child.clone();

            while let Some(c) = current {
                let next = c.borrow().next.clone();
                self.min_node = self.merge_nodes(self.min_node.clone(), Some(c.clone()));
                current = next;
            }

            if let Some(prev) = node.borrow().prev.clone() {
                prev.borrow_mut().next = node.borrow().next.clone();
            }
            if let Some(next) = node.borrow().next.clone() {
                next.borrow_mut().prev = node.borrow().prev.clone();
            }

            if node.as_ref().eq(child.clone().unwrap().as_ref()) {
                self.min_node = None;
            } else {
                self.min_node = child;
                self.consolidate();
            }

            self.size -= 1;
            Some(node.borrow().value)
        } else {
            None
        }
    }

    fn find_min(&self) -> Option<T> {
        self.min_node.as_ref().map(|node| node.borrow().value)
    }

    fn is_empty(&self) -> bool {
        self.min_node.is_none()
    }

    fn size(&self) -> usize {
        self.size
    }
}


#[cfg(test)]
mod fibo_heap_tests {
    use super::*;
    use std::f32::INFINITY;

    #[test]
    fn test_fibo_heap() {
        let mut heap = FibonacciHeap::new();

        heap.insert(4);
        heap.insert(1);
        heap.insert(7);

        assert_eq!(heap.find_min(), Some(1));
        assert_eq!(heap.size(), 3);
        assert!(!heap.is_empty());
    }

    #[test]
    fn test_extract_min() {
        let mut heap = FibonacciHeap::new();

        heap.insert(4);
        heap.insert(1);
        heap.insert(7);

        assert_eq!(heap.extract_min(), Some(1));
        assert_eq!(heap.extract_min(), Some(4));
        assert_eq!(heap.extract_min(), Some(7));
        assert_eq!(heap.extract_min(), None);
        assert!(heap.is_empty());
    }

}
