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
    let commanders = document.get_child("instances")
                             .get_child("commander");

    print_subtree(commanders, "".to_owned());

    // let values = document.get_child("instances")
    //                      .get_child("commander")
    //                      .get_child("aef")
    //                      .get_child("usf_airborne_company.xml")
    //                      .get_child("instance")
    //                      .get_child("group.commander_bag")
    //                      .get_child("locstring.name")
    //                      .get_value("value");
    // println!("{}", values);
    //println!("{:?}", document);
}

fn print_subtree(node: &Box<Node>, parent: String) {

    let mut new_parent = parent.clone();

    if !node.is_attribute_node() {
        if &parent != "" {
            new_parent.push_str("\\");
        }

        new_parent.push_str(&node.get_name());

        for subnode in &node.get_all_children() {
            print_subtree(subnode, new_parent.clone());
        }
    } else {
        println!("{}", node.get_name());
        let name = node.get_child("group.commander_bag")
                       .get_child("locstring.name")
                       .get_value("value");

        let abilities = node.get_child("group.commander_bag")
                            .get_child("list.commander_abilities")
                            .get_all_children();

        println!("{}", new_parent);
        println!("{}", name);
        for node in &abilities {
            println!("{}", node.get_value("value"));
        }
    }
}