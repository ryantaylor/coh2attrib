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
    println!("root node: {}", document.name);
    println!("number of children: {}", document.children.len());
    for (name, node) in &document.children {
        println!("child: {} {}", name, node.name);
    }
    let values = document.child("instances")
                         .child("commander")
                         .child("aef")
                         .child("usf_airborne_company.xml")
                         .child("instance")
                         .child("group.commander_bag")
                         .child("locstring.name")
                         .value("value");
    println!("{}", values);
    //println!("{:?}", document);
}