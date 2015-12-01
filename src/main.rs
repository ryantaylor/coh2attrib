extern crate coh2attrib;

use std::path::Path;

//use coh2attrib::node::Node;

fn main() {
    // let path = Path::new("/data/tools/assets/data/attributes/attributes.xml");
    // let document = coh2attrib::parse(&path).unwrap();
    // let values = document.child("enumshj")
    //                      .child("enum.area")
    //                      .value("value_list");
    // println!("{}", values);

    let path = Path::new("/data/tools/assets/data/attributes");
    let document = coh2attrib::walk_dir(&path);
    //println!("{:?}", document);
}