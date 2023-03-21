#![allow(dead_code)]
use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
struct Node<'a, T: Ord + Copy> {
    value: &'a T,
    left_kid: Option<Box<Node<'a, T>>>,
    right_kid: Option<Box<Node<'a, T>>>,
}

impl<'a, T: Ord + Copy> Node<'a, T> {
    fn add(&mut self, new_val: &'a T) {
        if self.value == new_val {
            return;
        }
        let target_node = if self.value > new_val {
            &mut self.left_kid
        } else {
            &mut self.right_kid
        };
        match target_node {
            Some(ref mut subnode) => subnode.add(new_val),
            None => {
                let new_node = Node {
                    value: new_val,
                    left_kid: None,
                    right_kid: None,
                };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }

    fn find(&self, value: &T) -> Option<&Node<T>> {
        match self.value.partial_cmp(value) {
            Some(Ordering::Equal) => Some(self),
            Some(Ordering::Less) => self.right_kid.as_ref().and_then(|node| node.find(value)),
            Some(Ordering::Greater) => self.left_kid.as_ref().and_then(|node| node.find(value)),
            None => None,
        }
    }

    fn iter_inorder(&self) -> InOrderIterator<'_, T> {
        InOrderIterator {
            next: Some(self),
            stack: Vec::new(),
        }
    }
}

struct InOrderIterator<'a, T: Ord + Copy> {
    next: Option<&'a Node<'a, T>>,
    stack: Vec<Option<&'a Node<'a, T>>>,
}

impl<'a, T: Ord + Copy + 'a> Iterator for InOrderIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.next {
            self.stack.push(Some(node));
            self.next = node.left_kid.as_deref();
        }

        let next_node = self.stack.pop()?.unwrap();
        self.next = next_node.right_kid.as_deref();

        Some(next_node.value)
    }
}

#[cfg(test)]
mod iterator_tests {
    use rand::seq::SliceRandom;

    use super::Node;

    fn get_inputs() -> (Vec<i32>, Vec<i32>) {
        let sorted_answers = &vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut clone = sorted_answers.clone();

        clone.shuffle(&mut rand::thread_rng());
        (sorted_answers.to_vec(), clone)
    }

    fn generate_tree(clone: &Vec<i32>) -> Node<i32> {
        let mut tree = Node {
            value: &5,
            left_kid: None,
            right_kid: None,
        };

        for value in clone {
            tree.add(value);
        }
        tree
    }

    #[test]
    fn test_iterator() {
        let (sorted_answers, clone) = get_inputs();

        let tree = generate_tree(&clone);

        for (count, value) in tree.iter_inorder().enumerate() {
            assert_eq!(*value, sorted_answers[count]);
        }
    }

    #[test]
    fn test_find() {
        let (_sorted_answers, clone) = get_inputs();

        let tree = generate_tree(&clone);

        assert_eq!(None, Some(tree.find(&11)).unwrap());

        let node = tree.find(&2).unwrap();

        assert_eq!(&2, node.value);
    }
}
