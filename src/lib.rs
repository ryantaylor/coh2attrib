#[macro_use]
extern crate nom;
extern crate xml;
extern crate walkdir;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::iter::Peekable;
use std::ops::Deref;
use std::path::Path;
use std::str::{FromStr, from_utf8_unchecked};

use nom::{digit, eof, not_line_ending, space, tab};
use nom::IResult::*;
use walkdir::{Iter, WalkDir};
use xml::reader::{EventReader, XmlEvent};

pub mod node;

use node::{DirNode, Node, XmlNode};

pub fn walk_dir(path: &Path) -> DirNode {
    let walker = WalkDir::new(&path);
    let mut walker = walker.into_iter().peekable();
    if let Some(node) = parse_dir(&mut walker, 0) {
        println!("got stuff!");
        node
    } else {
        println!("stuff go bye bye");
        DirNode::new()
    }
}

fn parse(path: &Path) -> XmlNode {
    let file = File::open(&path).unwrap();
    let file = BufReader::new(file);

    let mut parser = EventReader::new(file);
    if let Some(doc) = parse_node(&mut parser) {
        doc
    } else {
        println!("bad stuff");
        XmlNode::new()
    }
}

fn parse_node<R: Read>(parser: &mut EventReader<R>) -> Option<XmlNode> {
    let mut node = XmlNode::new();
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                let mut node_name = name.local_name.clone();
                node.set_name(&node_name);

                for attribute in &attributes {
                    if &attribute.name.local_name == "name" {
                        node_name.push_str(".");
                        node_name.push_str(&attribute.value);
                        node.set_name(&node_name);
                    }
                    node.add_data(&attribute.name.local_name, &attribute.value);
                }

                while let Some(child_node) = parse_node(parser) {
                    node.add_child(Box::new(child_node));
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

fn parse_dir(walker: &mut Peekable<Iter>, depth: usize) -> Option<DirNode> {
    let mut node = DirNode::new();
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
                        if let Some(name) = entry.path().file_stem() {
                            let name = name.to_string_lossy();
                            node.set_name(name.deref());
                        } else {
                            let name = entry.file_name().to_string_lossy();
                            node.set_name(name.deref());
                        }

                        if entry.file_type().is_dir() {
                            while let Some(child_node) = parse_dir(walker, entry.depth() + 1) {
                                node.add_child(Box::new(child_node));
                            }

                            return Some(node);
                        } else {
                            let child_node = parse(&entry.path());
                            node.add_child(Box::new(child_node));
                            println!("---returning node {}", node.get_name());
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
                println!("returning node {}", node.get_name());
                return Some(node);
            }
        }
    }
}

pub fn parse_locale(path: &Path) -> HashMap<i32, String> {
    println!("opening file...");
    let mut file = File::open(&path).unwrap();
    //let mut file_string = String::new();
    let mut file_vec = Vec::new();
    println!("reading to string...");
    //file.read_to_string(&mut file_string).unwrap();
    file.read_to_end(&mut file_vec).unwrap();

    println!("parsing...");

    named!(eol, alt!(tag!("\r\n") | tag!("\n") | tag!("\u{2028}") | tag!("\u{2029}")));

    named!(parse_entry <&[u8], (i32, String)>,
        chain!(
                id: map!(call!(digit), buf_to_i32)          ~
                    tab?                                    ~
               val: map!(call!(not_line_ending), to_string) ~
                    alt!(eof | eol),
                || {
                    //println!("{} {}", id, val);
                    (id, val.to_owned())
                }
            ));

    named!(parse_loc_strings <&[u8], (Vec<(i32, String)>)>, many0!(parse_entry));

    //let locale_entries = match parse_loc_strings(&file_string.as_bytes()[..]) {
    let locale_entries = match parse_loc_strings(&file_vec[..]) {
        Done(_, entries) => entries,
        _ => panic!("shit went down yo")
    };

    let mut locale_strings: HashMap<i32, String> = HashMap::new();

    for (id, value) in locale_entries {
        if id >= 0 {
            locale_strings.insert(id, value);
        }
    }

    locale_strings
}

fn to_string(s: &[u8]) -> &str {
    unsafe { from_utf8_unchecked(s) }
}

fn to_i32(s: &str) -> i32 {
    //println!("{}", s);
    match FromStr::from_str(s) {
        Ok(val) => val,
        _ => -1
    }
}

fn buf_to_i32(s: &[u8]) -> i32 {
    to_i32(to_string(s))
}