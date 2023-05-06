#![allow(dead_code)]
/*
The Composite pattern is a structural design pattern for
composing objects into tree structures to represent part-whole hierarchies.
It lets clients treat individual objects and compositions of objects uniformly.
 */

use std::fmt;

trait Component {
    fn print(&self) -> String;
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

impl Node for File {
    fn add(&mut self, _: Box<dyn Component>) {
        panic!("Cannot add child nodes to a file");
    }
}

trait Node: fmt::Display {
    fn add(&mut self, node: Box<dyn Component>);
}

struct Folder {
    name: String,
    components: Vec<Box<dyn Component>>,
}

impl Node for Folder {
    fn add(&mut self, node: Box<dyn Component>) {
        self.components.push(node);
    }
}


impl fmt::Display for Folder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Box<dyn Component> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

impl Folder {
    fn new(name: String) -> Self {
        Folder { name: name, components: vec![] }
    }

    pub fn add(self: &mut Self, component: Box<dyn Component>) {
        self.components.push(component)
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

impl Component for File {
    fn print(&self) -> String {
        format!("{}", self.name)
    }
}

impl Component for Folder {
    fn print(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("{}\n", self.name));
        for child in &self.components {
            result.push_str(&child.print());
            result.push_str("\n")
        }
        result
    }
}

#[cfg(test)]
mod composite_tests {
    use super::{Folder, File};

    #[test]
    fn test_composite_1() {
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
    }

    #[test]
    fn test_composite2() {
        let mut folder1 = Folder::new("folder1".to_string());
        let file1 = File::new("file1".to_string());

        folder1.add(Box::new(file1));

        assert_eq!(folder1.ls(), "folder1\nfile1\n");
    }
}
