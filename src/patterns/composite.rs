#![allow(dead_code)]
/*
The Composite pattern is a structural design pattern for
composing objects into tree structures to represent part-whole hierarchies.
It lets clients treat individual objects and compositions of objects uniformly.
 */

use std::fmt;

trait Component {
    fn display(&self);
}

pub struct File {
    name: String,
}

impl File {
    fn new(name: String) -> Self {
        File { name }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Component for File {

    fn display(&self) {
        println!("File: {}", self.name);
    }
}

impl Node for File {
    fn add(&mut self, _: Box<dyn Node>) {
        panic!("Cannot add child nodes to a file");
    }
}


trait Node: fmt::Display {
    fn add(&mut self, node: Box<dyn Node>);
}

struct Folder {
    name: String,
    components: Vec<Box<dyn Component>>,
}

impl Node for Folder {
    fn add(&mut self, node: Box<dyn Node>) {
        self.components.push(node);
    }
}


impl fmt::Display for Folder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Folder {
    fn new(name: String) -> Self {
        Folder { name: name, components: vec![] }
    }

    pub fn add(self: &Self, component: Box<dyn Component>) {
        self.components.append(component)
    }

    fn ls(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("{}\n", self.name));

        for child in &self.components {
            let child_output = child.to_string();

            if let Some('\n') = child_output.chars().next_back() {
                output.push_str(&format!("{}{}", child_output, ""));
            } else {
                output.push_str(&format!("{}\n", child_output));
            }
        }

        output
    }
}


impl Component for Folder {
    fn display(&self) {
        println!("Folder: {}", self.name);
        for component in self.components.iter() {
            component.display();
        }
    }
}

#[cfg(test)]
mod composite_tests {
    use crate::patterns::composite::{Folder, File};

    #[test]
    fn test_composite() {
        let mut root = Folder::new("root".to_string());
        let mut folder1 = Folder::new("folder1".to_string());
        let mut folder2 = Folder::new("folder2".to_string());
        let file1 = File::new("file1".to_string());
        let file2 = File::new("file2".to_string());
        let file3 = File::new("file3".to_string());

        folder1.add(Box::new(file1));
        folder2.add(Box::new(file2));
        folder2.add(Box::new(file3));

        root.add(Box::new(folder1));
        root.add(Box::new(folder2));

        assert_eq!(root.ls(), "root\nfolder1\nfile1\nfolder2\nfile2\nfile3\n");
        assert_eq!(folder1.ls(), "folder1\nfile1\n");
        assert_eq!(folder2.ls(), "folder2\nfile2\nfile3\n");
    }
}
