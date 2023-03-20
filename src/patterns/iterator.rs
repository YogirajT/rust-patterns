#![allow(dead_code)]

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
        let target_node = if new_val < self.value {
            &mut self.left_kid
        } else {
            &mut self.right_kid
        };
        match target_node {
            &mut Some(ref mut subnode) => subnode.add(new_val),
            &mut None => {
                let new_node = Node {
                    value: new_val,
                    left_kid: None,
                    right_kid: None,
                };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
            Some(_) => todo!(),
        }
    }
}

impl<'a, T: Ord + Copy> Iterator for Node<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // traverse left subtree first, if it exists
        if let Some(left) = &mut self.left_kid {
            if let Some(value) = left.next() {
                return Some(value);
            }
        }

        // traverse right subtree next, if it exists
        if let Some(right) = &mut self.right_kid {
            if let Some(value) = right.next() {
                return Some(value);
            }
        }

        // no more values to iterate over
        None
    }
}

#[cfg(test)]
mod iterator_tests {
    use super::Node;

    #[test]
    fn test_iterator() {
        let mut tree = Node {
            value: &5,
            left_kid: None,
            right_kid: None,
        };
        tree.add(&1);
        tree.add(&3);
        tree.add(&2);
        tree.add(&4);
        tree.add(&9);
        tree.add(&8);
        tree.add(&7);

        println!("{tree:#?}");

        for value in tree {
            println!("{value}");
        }
    }
}
