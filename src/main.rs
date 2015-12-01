extern crate coh2attrib;

use std::path::Path;

use coh2attrib::node::Node;

fn main() {
    // let path = Path::new("/data/tools/assets/data/attributes/attributes.xml");
    // let document = coh2attrib::parse(&path).unwrap();
    // let values = document.child("enumshj")
    //                      .child("enum.area")
    //                      .value("value_list");
    // println!("{}", values);

    let path = Path::new("/data/tools/assets/data/attributes");
    println!("parsing attributes...");
    let document = coh2attrib::walk_dir(&path);
    println!("root node: {}", document.get_name());
    //println!("number of children: {}", document.children.len());
    // for (name, node) in &document.get_children() {
    //     println!("child: {} {}", name, node.get_name());
    // }
    let values = document.get_child("instances")
                         .get_child("commander")
                         .get_child("aef")
                         .get_child("usf_airborne_company.xml")
                         .get_child("instance")
                         .get_child("group.commander_bag")
                         .get_child("locstring.name")
                         .get_value("value");
    println!("{}", values);
    //println!("{:?}", document);
}