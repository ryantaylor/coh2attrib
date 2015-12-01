use std::collections::HashMap;
use std::ops::Deref;

pub trait Node {
    fn is_attribute_node(&self) -> bool;
    fn get_name(&self) -> String;
    fn set_name(&mut self, name: &str);
    fn add_data(&mut self, name: &str, value: &str);
    fn add_child(&mut self, node: Box<Node>);
    fn get_child(&self, name: &str) -> &Box<Node>;
    fn get_children(&self, name: &str) -> Vec<&Box<Node>>;
    fn get_all_children(&self) -> Vec<&Box<Node>>;
    fn get_value(&self, name: &str) -> String;
}

pub struct DirNode {
    name: String,
    children: HashMap<String, Box<Node>>,
    data: HashMap<String, String>,
}

impl DirNode {
    pub fn new() -> DirNode {
        DirNode {
            name: String::new(),
            children: HashMap::new(),
            data: HashMap::new(),
        }
    }
}

impl Node for DirNode {
    fn is_attribute_node(&self) -> bool {
        false
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    fn add_data(&mut self, name: &str, value: &str) {
        self.data.insert(String::from(name), String::from(value));
    }

    fn add_child(&mut self, node: Box<Node>) {
        self.children.insert(node.get_name(), node);
    }

    fn get_child(&self, name: &str) -> &Box<Node> {
        self.children.get(name).unwrap()
    }

    fn get_children(&self, name: &str) -> Vec<&Box<Node>> {
        let mut children = Vec::new();
        for (node_name, node) in &self.children {
            if node_name == name {
                children.push(node);
            }
        }

        children
    }

    fn get_all_children(&self) -> Vec<&Box<Node>> {
        let mut children = Vec::new();
        for (_, node) in &self.children {
            children.push(node);
        }

        children
    }

    fn get_value(&self, name: &str) -> String {
        if let Some(value) = self.data.get(name) {
            value.to_owned()
        } else {
            String::new()
        }
    }
}

pub struct XmlNode {
    name: String,
    children: Vec<Box<Node>>,
    data: HashMap<String, String>,
}

impl XmlNode {
    pub fn new() -> XmlNode {
        XmlNode {
            name: String::new(),
            children: Vec::new(),
            data: HashMap::new(),
        }
    }
}

impl Node for XmlNode {
    fn is_attribute_node(&self) -> bool {
        true
    }
    
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    fn add_data(&mut self, name: &str, value: &str) {
        self.data.insert(String::from(name), String::from(value));
    }

    fn add_child(&mut self, node: Box<Node>) {
        self.children.push(node);
    }

    fn get_child(&self, name: &str) -> &Box<Node> {
        for node in &self.children {
            if &node.deref().get_name() == name {
                return node;
            }
        }

        panic!("couldn't find child {}", name)
    }

    fn get_children(&self, name: &str) -> Vec<&Box<Node>> {
        let mut children = Vec::new();
        for node in &self.children {
            if &node.deref().get_name() == name {
                children.push(node);
            }
        }

        children
    }

    fn get_all_children(&self) -> Vec<&Box<Node>> {
        let mut children = Vec::new();
        for node in &self.children {
            children.push(node);
        }

        children
    }

    fn get_value(&self, name: &str) -> String {
        if let Some(value) = self.data.get(name) {
            value.to_owned()
        } else {
            String::new()
        }
    }
}