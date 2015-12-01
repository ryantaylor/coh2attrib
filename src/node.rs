use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub children: HashMap<String, Node>,
    pub data: HashMap<String, String>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            name: String::new(),
            children: HashMap::new(),
            data: HashMap::new(),
        }
    }

    pub fn child(&self, name: &str) -> Node {
        if let Some(node) = self.children.get(name) {
            println!("node {} has {} children", name, node.children.len());
            node.clone()
        } else {
            println!("couldn't find node {}", name);
            Node::new()
        }
    }

    pub fn value(&self, name: &str) -> String {
        if let Some(value) = self.data.get(name) {
            value.to_owned()
        } else {
            String::new()
        }
    }
}