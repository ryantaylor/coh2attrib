extern crate xml;
extern crate walkdir;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::iter::Peekable;
use std::ops::Deref;
use std::path::Path;

use walkdir::{Iter, WalkDir};
use xml::reader::{EventReader, XmlEvent};

pub mod node;

use node::Node;

pub fn walk_dir(path: &Path) -> Node {
    let walker = WalkDir::new(&path);
    let mut walker = walker.into_iter().peekable();
    if let Some(node) = parse_dir(&mut walker, 0) {
        println!("got stuff!");
        node
    } else {
        println!("stuff go bye bye");
        Node::new()
    }
}

fn parse(path: &Path) -> Node {
    let file = File::open(&path).unwrap();
    let file = BufReader::new(file);

    let mut parser = EventReader::new(file);
    if let Some(doc) = parse_node(&mut parser) {
        doc
    } else {
        println!("bad stuff");
        Node::new()
    }
}

fn parse_node<R: Read>(parser: &mut EventReader<R>) -> Option<Node> {
    let mut node = Node::new();
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                node.name = name.local_name.clone();
                for attribute in &attributes {
                    if &attribute.name.local_name == "name" {
                        node.name.push_str(".");
                        node.name.push_str(&attribute.value);
                    }
                    node.data.insert(attribute.name.local_name.clone(), attribute.value.clone());
                }

                while let Some(child_node) = parse_node(parser) {
                    node.children.insert(child_node.name.clone(), child_node);
                }

                return Some(node);
            },
            Ok(XmlEvent::EndElement { .. }) => {
                return None;
            }
            Err(err) => {
                println!("error: {}", err);
                return None;
            },
            _ => {}
        }
    }
}

fn parse_dir(walker: &mut Peekable<Iter>, depth: usize) -> Option<Node> {
    let mut node = Node::new();
    if let Some(next) = walker.peek() {
        match next {
            &Ok(ref val) => {
                if val.depth() != depth {
                    return None;
                }
            },
            &Err(ref err) => {
                println!("parse_dir error {}", err);
                return None;
            }
        }
    } else {
        return None;
    }
    loop {
        match walker.next() {
            Some(result) => {
                match result {
                    Ok(entry) => {
                        //println!("{}", entry.depth());
                        let name = entry.file_name().to_string_lossy();
                        node.name = String::from(name.deref());

                        if entry.file_type().is_dir() {
                            //println!("in directory {}", entry.path().display());
                            //let depth = entry.depth();

                            while let Some(child_node) = parse_dir(walker, entry.depth() + 1) {
                                node.children.insert(child_node.name.clone(), child_node);
                            }

                            return Some(node);
                        } else {
                            //println!("in file {}", entry.path().display());
                            let child_node = parse(&entry.path());
                            node.children.insert(child_node.name.clone(), child_node);
                            println!("---returning node {}", node.name);
                            return Some(node);
                        }
                    },
                    Err(err) => {
                        println!("error {}", err);
                        return None;
                    }
                }
            },
            _ => {
                //println!("finished");
                println!("returning node {}", node.name);
                return Some(node);
            }
        }
    }
}

// fn parse_dir(walker: &mut Iter) -> Node {
//     let mut node = Node::new();
//     walker.peekable();
//     match walker.next() {
//         Some(result) => {
//             match result {
//                 Ok(entry) => {
//                     let name = entry.file_name.to_string_lossy();
//                     node.name = String::new(name.deref());
//                     let depth = entry.depth();

//                     loop {
//                         if let Some(next) = walker.peek() {
//                             if next.file_type.is_dir() {
//                                 let child_node = parse_dir(walker);
//                                 node.children.insert(child_node.name.clone(), child_node);
//                             } else {
//                                 if next.depth() == depth {

//                                 }
//                             }
//                         } else {
//                             Node::new()
//                         }
//                     }

//                     return Some(node);
//                 },
//                 Err(err) => {
//                     println!("error {}", err);
//                     Node::new()
//                 }
//             }
//         },
//         _ => { node }
//     }
// }