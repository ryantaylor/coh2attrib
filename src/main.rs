extern crate coh2attrib;
extern crate postgres;

use std::collections::HashMap;
use std::path::Path;

use postgres::{Connection, SslMode};

use coh2attrib::node::Node;

fn main() {
    let path = Path::new("/data/coh2/CoH2/Locale/English/locale.ucs");
    let strings = coh2attrib::parse_locale(&path);

    let conn = Connection::connect("postgresql://ryan@localhost/cohdb", &SslMode::None).unwrap();

    let path = Path::new("/data/tools/assets/data/attributes");
    println!("parsing attributes...");
    let document = coh2attrib::walk_dir(&path);
    println!("root node: {}", document.get_name());

    let abilities = document.get_child("instances")
                            .get_child("abilities");
    load_abilities(abilities, "".to_owned(), &conn, &strings);

    let ebps = document.get_child("instances")
                       .get_child("ebps")
                       .get_child("races");
    load_ebps(ebps, "ebps".to_owned(), &conn, &strings);

    let sbps = document.get_child("instances")
                       .get_child("sbps");
    load_sbps(sbps, "".to_owned(), &conn, &strings);

    let upgrade = document.get_child("instances")
                          .get_child("upgrade");
    load_upgrade(upgrade, "".to_owned(), &conn, &strings);

    let commander = document.get_child("instances")
                            .get_child("commander");
    load_commander(commander, "".to_owned(), &conn, &strings);

    let intel_bulletin = document.get_child("instances")
                                 .get_child("intel_bulletin");
    load_intel_bulletin(intel_bulletin, "".to_owned(), &conn, &strings);

    let skin_pack = document.get_child("instances")
                            .get_child("skin_pack");
    load_skin_pack(skin_pack, "".to_owned(), &conn, &strings);

    let faceplate = document.get_child("instances")
                            .get_child("faceplate");
    load_faceplate(faceplate, "".to_owned(), &conn, &strings);

    let vehicle_decal = document.get_child("instances")
                                .get_child("vehicle_decal");
    load_vehicle_decal(vehicle_decal, "".to_owned(), &conn, &strings);

    let fatality = document.get_child("instances")
                           .get_child("fatality");
    load_fatality(fatality, "".to_owned(), &conn, &strings);
}

fn load_ebps(node: &Box<Node>, parent: String, conn: &Connection, locale: &HashMap<i32, String>) {
    let mut new_parent = parent.clone();

    if !node.is_attribute_node() {
        if &parent != "" {
            new_parent.push_str("\\");
        }

        new_parent.push_str(&node.get_name());

        for subnode in &node.get_all_children() {
            load_ebps(subnode, new_parent.clone(), conn, locale);
        }
    } else {
        //println!("{}", node.get_name());
        let pbgid = node.get_child("uniqueid.pbgid")
                        .get_value("value")
                        .parse::<i32>().unwrap();

        let references = node.get_child("list.extensions")
                             .get_all_children();

        let mut screen_name = String::new();
        let mut icon_name = String::new();
        let mut help_text = String::new();
        let mut extra_text = String::new();

        for node in &references {
            if &node.get_value("value") == "ebpextensions\\ui_ext" {
                //println!("{}", node.get_name());
                screen_name = node.get_child("locstring.screen_name")
                                  .get_value("value");
                icon_name = node.get_child("icon.icon_name")
                                .get_value("value");
                help_text = node.get_child("locstring.help_text")
                                .get_value("value");
                extra_text = node.get_child("locstring.extra_text")
                                 .get_value("value");

                screen_name = get_locale_text(&screen_name, locale);
                help_text = get_locale_text(&help_text, locale);
                extra_text = get_locale_text(&extra_text, locale);
            }
        }

        let stmt = conn.prepare("INSERT INTO ebps (pbgid, attribute_id, screen_name, icon_name, help_text, extra_text) VALUES ($1, $2, $3, $4, $5, $6)").unwrap();
        match stmt.execute(&[&pbgid, &new_parent, &screen_name, &icon_name, &help_text, &extra_text]) {
            Ok(_) => {},
            Err(err) => {
                println!("error: {}", err);
                println!("{}", new_parent);
                println!("{}", pbgid);
            }
        }

        // println!("pbgid: {}", pbgid);
        // println!("attribute_id: {}", new_parent);
        // println!("screen_name: {}", screen_name);
        // println!("icon_name: {}", icon_name);
        // println!("help_text: {}", help_text);
        // println!("extra_text: {}", extra_text);
    }
}

fn load_sbps(node: &Box<Node>, parent: String, conn: &Connection, locale: &HashMap<i32, String>) {
    let mut new_parent = parent.clone();

    if !node.is_attribute_node() {
        if &parent != "" {
            new_parent.push_str("\\");
        }

        new_parent.push_str(&node.get_name());

        for subnode in &node.get_all_children() {
            load_sbps(subnode, new_parent.clone(), conn, locale);
        }
    } else {
        //println!("{}", node.get_name());
        let pbgid = node.get_child("uniqueid.pbgid")
                        .get_value("value")
                        .parse::<i32>().unwrap();

        let references = node.get_child("list.extensions")
                             .get_children("template_reference.squadexts");

        let mut screen_name = String::new();
        let mut icon_name = String::new();
        let mut help_text = String::new();
        let mut extra_text = String::new();

        for node in &references {
            if &node.get_value("value") == "sbpextensions\\squad_ui_ext" {
                //println!("{}", node.get_name());
                let child_node = node.get_child("list.race_list")
                                     .get_child("group.race_data")
                                     .get_child("group.info");


                screen_name = child_node.get_child("locstring.screen_name")
                                        .get_value("value");
                icon_name = child_node.get_child("icon.icon_name")
                                      .get_value("value");
                help_text = child_node.get_child("locstring.help_text")
                                      .get_value("value");
                extra_text = child_node.get_child("locstring.extra_text")
                                       .get_value("value");

                screen_name = get_locale_text(&screen_name, locale);
                help_text = get_locale_text(&help_text, locale);
                extra_text = get_locale_text(&extra_text, locale);
            }
        }

        let stmt = conn.prepare("INSERT INTO sbps (pbgid, attribute_id, screen_name, icon_name, help_text, extra_text) VALUES ($1, $2, $3, $4, $5, $6)").unwrap();
        match stmt.execute(&[&pbgid, &new_parent, &screen_name, &icon_name, &help_text, &extra_text]) {
            Ok(_) => {},
            Err(err) => {
                println!("error: {}", err);
                println!("{}", new_parent);
                println!("{}", pbgid);
            }
        }

        // println!("pbgid: {}", pbgid);
        // println!("attribute_id: {}", new_parent);
        // println!("screen_name: {}", screen_name);
        // println!("icon_name: {}", icon_name);
        // println!("help_text: {}", help_text);
        // println!("extra_text: {}", extra_text);
    }
}

fn load_upgrade(node: &Box<Node>, parent: String, conn: &Connection, locale: &HashMap<i32, String>) {
    let mut new_parent = parent.clone();

    if !node.is_attribute_node() {
        if &parent != "" {
            new_parent.push_str("\\");
        }

        new_parent.push_str(&node.get_name());

        for subnode in &node.get_all_children() {
            load_upgrade(subnode, new_parent.clone(), conn, locale);
        }
    } else {
        //println!("{}", node.get_name());
        let pbgid = node.get_child("uniqueid.pbgid")
                        .get_value("value")
                        .parse::<i32>().unwrap();

        let mut screen_name = node.get_child("group.upgrade_bag")
                              .get_child("group.ui_info")
                              .get_child("locstring.screen_name")
                              .get_value("value");
        let icon_name = node.get_child("group.upgrade_bag")
                            .get_child("group.ui_info")
                            .get_child("icon.icon_name")
                            .get_value("value");
        let mut help_text = node.get_child("group.upgrade_bag")
                            .get_child("group.ui_info")
                            .get_child("locstring.help_text")
                            .get_value("value");
        let mut extra_text = node.get_child("group.upgrade_bag")
                             .get_child("group.ui_info")
                             .get_child("locstring.extra_text")
                             .get_value("value");

        screen_name = get_locale_text(&screen_name, locale);
        help_text = get_locale_text(&help_text, locale);
        extra_text = get_locale_text(&extra_text, locale);

        let stmt = conn.prepare("INSERT INTO upgrade (pbgid, attribute_id, screen_name, icon_name, help_text, extra_text) VALUES ($1, $2, $3, $4, $5, $6)").unwrap();
        match stmt.execute(&[&pbgid, &new_parent, &screen_name, &icon_name, &help_text, &extra_text]) {
            Ok(_) => {},
            Err(err) => {
                println!("error: {}", err);
                println!("{}", new_parent);
                println!("{}", pbgid);
            }
        }

        // println!("pbgid: {}", pbgid);
        // println!("attribute_id: {}", new_parent);
        // println!("screen_name: {}", screen_name);
        // println!("icon_name: {}", icon_name);
        // println!("help_text: {}", help_text);
        // println!("extra_text: {}", extra_text);
    }
}

fn load_abilities(node: &Box<Node>, parent: String, conn: &Connection, locale: &HashMap<i32, String>) {
    let mut new_parent = parent.clone();

    if !node.is_attribute_node() {
        if &parent != "" {
            new_parent.push_str("\\");
        }

        new_parent.push_str(&node.get_name());

        for subnode in &node.get_all_children() {
            load_abilities(subnode, new_parent.clone(), conn, locale);
        }
    } else {
        //println!("{}", node.get_name());
        let pbgid = node.get_child("uniqueid.pbgid")
                        .get_value("value")
                        .parse::<i32>().unwrap();

        let mut screen_name = node.get_child("group.ability_bag")
                              .get_child("group.ui_info")
                              .get_child("locstring.screen_name")
                              .get_value("value");
        let icon_name = node.get_child("group.ability_bag")
                            .get_child("group.ui_info")
                            .get_child("icon.icon_name")
                            .get_value("value");
        let mut help_text = node.get_child("group.ability_bag")
                            .get_child("group.ui_info")
                            .get_child("locstring.help_text")
                            .get_value("value");
        let mut extra_text = node.get_child("group.ability_bag")
                             .get_child("group.ui_info")
                             .get_child("locstring.extra_text")
                             .get_value("value");

        screen_name = get_locale_text(&screen_name, locale);
        help_text = get_locale_text(&help_text, locale);
        extra_text = get_locale_text(&extra_text, locale);

        let stmt = conn.prepare("INSERT INTO abilities (pbgid, attribute_id, screen_name, icon_name, help_text, extra_text) VALUES ($1, $2, $3, $4, $5, $6)").unwrap();
        match stmt.execute(&[&pbgid, &new_parent, &screen_name, &icon_name, &help_text, &extra_text]) {
            Ok(_) => {},
            Err(err) => {
                println!("error: {}", err);
                println!("{}", new_parent);
                println!("{}", pbgid);
            }
        }

        // println!("pbgid: {}", pbgid);
        // println!("attribute_id: {}", new_parent);
        // println!("screen_name: {}", screen_name);
        // println!("icon_name: {}", icon_name);
        // println!("help_text: {}", help_text);
        // println!("extra_text: {}", extra_text);
    }
}

fn load_commander(node: &Box<Node>, parent: String, conn: &Connection, locale: &HashMap<i32, String>) {
    let mut new_parent = parent.clone();

    if !node.is_attribute_node() {
        if &parent != "" {
            new_parent.push_str("\\");
        }

        new_parent.push_str(&node.get_name());

        for subnode in &node.get_all_children() {
            load_commander(subnode, new_parent.clone(), conn, locale);
        }
    } else {
        //println!("{}", node.get_name());
        let pbgid = node.get_child("uniqueid.pbgid")
                        .get_value("value")
                        .parse::<i32>().unwrap();

        let server_id = node.get_child("group.commander_bag")
                            .get_child("template_reference.server_item")
                            .get_child("uniqueid.server_id")
                            .get_value("value")
                            .parse::<i32>().unwrap();

        let mut name = node.get_child("group.commander_bag")
                       .get_child("locstring.name")
                       .get_value("value");

        let mut description = node.get_child("group.commander_bag")
                              .get_child("locstring.description")
                              .get_value("value");

        let icon = node.get_child("group.commander_bag")
                       .get_child("icon.icon")
                       .get_value("value");

        let icon_secondary = node.get_child("group.commander_bag")
                                 .get_child("icon.icon_secondary")
                                 .get_value("value");

        let rarity = node.get_child("group.commander_bag")
                         .get_child("template_reference.server_item")
                         .get_child("enum.rarity")
                         .get_value("value");

        name = get_locale_text(&name, locale);
        description = get_locale_text(&description, locale);

        let stmt = conn.prepare("INSERT INTO commander (server_id, pbgid, attribute_id, name, description, icon, icon_secondary, rarity) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)").unwrap();
        match stmt.execute(&[&server_id, &pbgid, &new_parent, &name, &description, &icon, &icon_secondary, &rarity]) {
            Ok(_) => {},
            Err(err) => {
                println!("error: {}", err);
                println!("{}", new_parent);
                println!("{}", server_id);
            }
        }

        // println!("pbgid: {}", pbgid);
        // println!("attribute_id: {}", new_parent);
        // println!("screen_name: {}", screen_name);
        // println!("icon_name: {}", icon_name);
        // println!("help_text: {}", help_text);
        // println!("extra_text: {}", extra_text);
    }
}

fn load_intel_bulletin(node: &Box<Node>, parent: String, conn: &Connection, locale: &HashMap<i32, String>) {
    let mut new_parent = parent.clone();

    if !node.is_attribute_node() {
        if &parent != "" {
            new_parent.push_str("\\");
        }

        new_parent.push_str(&node.get_name());

        for subnode in &node.get_all_children() {
            load_intel_bulletin(subnode, new_parent.clone(), conn, locale);
        }
    } else {
        //println!("{}", new_parent);
        //println!("{}", node.get_name());
        let pbgid = node.get_child("uniqueid.pbgid")
                        .get_value("value")
                        .parse::<i32>().unwrap();

        let server_id = node.get_child("group.intel_bag")
                            .get_child("template_reference.server_item")
                            .get_child("uniqueid.server_id")
                            .get_value("value")
                            .parse::<i32>().unwrap();

        let mut name = node.get_child("group.intel_bag")
                       .get_child("locstring.name")
                       .get_value("value");

        let mut description = node.get_child("group.intel_bag")
                              .get_child("locstring.description")
                              .get_value("value");

        let icon = node.get_child("group.intel_bag")
                       .get_child("icon.icon")
                       .get_value("value");

        let rarity = node.get_child("group.intel_bag")
                         .get_child("template_reference.server_item")
                         .get_child("enum.rarity")
                         .get_value("value");

        name = get_locale_text(&name, locale);
        description = get_locale_text(&description, locale);

        let stmt = conn.prepare("INSERT INTO intel_bulletin (server_id, pbgid, attribute_id, name, description, icon, rarity) VALUES ($1, $2, $3, $4, $5, $6, $7)").unwrap();
        match stmt.execute(&[&server_id, &pbgid, &new_parent, &name, &description, &icon, &rarity]) {
            Ok(_) => {},
            Err(err) => {
                println!("error: {}", err);
                println!("{}", new_parent);
                println!("{}", server_id);
            }
        }

        // println!("pbgid: {}", pbgid);
        // println!("attribute_id: {}", new_parent);
        // println!("screen_name: {}", screen_name);
        // println!("icon_name: {}", icon_name);
        // println!("help_text: {}", help_text);
        // println!("extra_text: {}", extra_text);
    }
}

fn load_skin_pack(node: &Box<Node>, parent: String, conn: &Connection, locale: &HashMap<i32, String>) {
    let mut new_parent = parent.clone();

    if !node.is_attribute_node() {
        if &parent != "" {
            new_parent.push_str("\\");
        }

        new_parent.push_str(&node.get_name());

        for subnode in &node.get_all_children() {
            load_skin_pack(subnode, new_parent.clone(), conn, locale);
        }
    } else {
        //println!("{}", node.get_name());
        let pbgid = node.get_child("uniqueid.pbgid")
                        .get_value("value")
                        .parse::<i32>().unwrap();

        let server_id = node.get_child("group.skin_pack_bag")
                            .get_child("template_reference.server_item")
                            .get_child("uniqueid.server_id")
                            .get_value("value")
                            .parse::<i32>().unwrap();

        let skin_name = node.get_child("group.skin_pack_bag")
                            .get_child("string.skin_name")
                            .get_value("value");

        let season = node.get_child("group.skin_pack_bag")
                         .get_child("enum.season")
                         .get_value("value");

        let mut name = node.get_child("group.skin_pack_bag")
                       .get_child("locstring.name")
                       .get_value("value");

        let mut description = node.get_child("group.skin_pack_bag")
                              .get_child("locstring.description")
                              .get_value("value");

        let icon = node.get_child("group.skin_pack_bag")
                       .get_child("icon.icon")
                       .get_value("value");

        let icon_secondary = node.get_child("group.skin_pack_bag")
                                 .get_child("icon.icon_secondary")
                                 .get_value("value");

        let rarity = node.get_child("group.skin_pack_bag")
                         .get_child("template_reference.server_item")
                         .get_child("enum.rarity")
                         .get_value("value");

        name = get_locale_text(&name, locale);
        description = get_locale_text(&description, locale);

        let stmt = conn.prepare("INSERT INTO skin_pack (server_id, pbgid, attribute_id, skin_name, name, description, icon, icon_secondary, season, rarity) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)").unwrap();
        match stmt.execute(&[&server_id, &pbgid, &new_parent, &skin_name, &name, &description, &icon, &icon_secondary, &season, &rarity]) {
            Ok(_) => {},
            Err(err) => {
                println!("error: {}", err);
                println!("{}", new_parent);
                println!("{}", server_id);
            }
        }

        // println!("pbgid: {}", pbgid);
        // println!("attribute_id: {}", new_parent);
        // println!("screen_name: {}", screen_name);
        // println!("icon_name: {}", icon_name);
        // println!("help_text: {}", help_text);
        // println!("extra_text: {}", extra_text);
    }
}

fn load_faceplate(node: &Box<Node>, parent: String, conn: &Connection, locale: &HashMap<i32, String>) {
    let mut new_parent = parent.clone();

    if !node.is_attribute_node() {
        if &parent != "" {
            new_parent.push_str("\\");
        }

        new_parent.push_str(&node.get_name());

        for subnode in &node.get_all_children() {
            load_faceplate(subnode, new_parent.clone(), conn, locale);
        }
    } else {
        //println!("{}", node.get_name());
        let pbgid = node.get_child("uniqueid.pbgid")
                        .get_value("value")
                        .parse::<i32>().unwrap();

        let server_id = node.get_child("group.faceplate_bag")
                            .get_child("template_reference.server_item")
                            .get_child("uniqueid.server_id")
                            .get_value("value")
                            .parse::<i32>().unwrap();

        let mut name = node.get_child("group.faceplate_bag")
                       .get_child("locstring.name")
                       .get_value("value");

        let icon = node.get_child("group.faceplate_bag")
                       .get_child("icon.icon")
                       .get_value("value");

        let rarity = node.get_child("group.faceplate_bag")
                         .get_child("template_reference.server_item")
                         .get_child("enum.rarity")
                         .get_value("value");

        name = get_locale_text(&name, locale);

        let stmt = conn.prepare("INSERT INTO faceplate (server_id, pbgid, attribute_id, name, icon, rarity) VALUES ($1, $2, $3, $4, $5, $6)").unwrap();
        match stmt.execute(&[&server_id, &pbgid, &new_parent, &name, &icon, &rarity]) {
            Ok(_) => {},
            Err(err) => {
                println!("error: {}", err);
                println!("{}", new_parent);
                println!("{}", server_id);
            }
        }

        // println!("pbgid: {}", pbgid);
        // println!("attribute_id: {}", new_parent);
        // println!("screen_name: {}", screen_name);
        // println!("icon_name: {}", icon_name);
        // println!("help_text: {}", help_text);
        // println!("extra_text: {}", extra_text);
    }
}

fn load_fatality(node: &Box<Node>, parent: String, conn: &Connection, locale: &HashMap<i32, String>) {
    let mut new_parent = parent.clone();

    if !node.is_attribute_node() {
        if &parent != "" {
            new_parent.push_str("\\");
        }

        new_parent.push_str(&node.get_name());

        for subnode in &node.get_all_children() {
            load_fatality(subnode, new_parent.clone(), conn, locale);
        }
    } else {
        //println!("{}", node.get_name());
        let pbgid = node.get_child("uniqueid.pbgid")
                        .get_value("value")
                        .parse::<i32>().unwrap();

        let server_id = node.get_child("group.fatality_bag")
                            .get_child("template_reference.server_item")
                            .get_child("uniqueid.server_id")
                            .get_value("value")
                            .parse::<i32>().unwrap();

        let mut name = node.get_child("group.fatality_bag")
                       .get_child("locstring.name")
                       .get_value("value");

        let mut description = node.get_child("group.fatality_bag")
                              .get_child("locstring.description")
                              .get_value("value");

        let icon = node.get_child("group.fatality_bag")
                       .get_child("icon.icon")
                       .get_value("value");

        let rarity = node.get_child("group.fatality_bag")
                         .get_child("template_reference.server_item")
                         .get_child("enum.rarity")
                         .get_value("value");

        name = get_locale_text(&name, locale);
        description = get_locale_text(&description, locale);

        let stmt = conn.prepare("INSERT INTO fatality (server_id, pbgid, attribute_id, name, description, icon, rarity) VALUES ($1, $2, $3, $4, $5, $6, $7)").unwrap();
        match stmt.execute(&[&server_id, &pbgid, &new_parent, &name, &description, &icon, &rarity]) {
            Ok(_) => {},
            Err(err) => {
                println!("error: {}", err);
                println!("{}", new_parent);
                println!("{}", server_id);
            }
        }

        // println!("pbgid: {}", pbgid);
        // println!("attribute_id: {}", new_parent);
        // println!("screen_name: {}", screen_name);
        // println!("icon_name: {}", icon_name);
        // println!("help_text: {}", help_text);
        // println!("extra_text: {}", extra_text);
    }
}

fn load_vehicle_decal(node: &Box<Node>, parent: String, conn: &Connection, locale: &HashMap<i32, String>) {
    let mut new_parent = parent.clone();

    if !node.is_attribute_node() {
        if &parent != "" {
            new_parent.push_str("\\");
        }

        new_parent.push_str(&node.get_name());

        for subnode in &node.get_all_children() {
            load_vehicle_decal(subnode, new_parent.clone(), conn, locale);
        }
    } else {
        //println!("{}", node.get_name());
        let pbgid = node.get_child("uniqueid.pbgid")
                        .get_value("value")
                        .parse::<i32>().unwrap();

        let server_id = node.get_child("group.vehicle_decal_bag")
                            .get_child("template_reference.server_item")
                            .get_child("uniqueid.server_id")
                            .get_value("value")
                            .parse::<i32>().unwrap();

        let decal_name = node.get_child("group.vehicle_decal_bag")
                             .get_child("string.decal_name")
                             .get_value("value");

        let mut name = node.get_child("group.vehicle_decal_bag")
                       .get_child("locstring.name")
                       .get_value("value");

        let mut description = node.get_child("group.vehicle_decal_bag")
                              .get_child("locstring.description")
                              .get_value("value");

        let icon = node.get_child("group.vehicle_decal_bag")
                       .get_child("icon.icon")
                       .get_value("value");

        let rarity = node.get_child("group.vehicle_decal_bag")
                         .get_child("template_reference.server_item")
                         .get_child("enum.rarity")
                         .get_value("value");

        name = get_locale_text(&name, locale);
        description = get_locale_text(&description, locale);

        let stmt = conn.prepare("INSERT INTO vehicle_decal (server_id, pbgid, attribute_id, decal_name, name, description, icon, rarity) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)").unwrap();
        match stmt.execute(&[&server_id, &pbgid, &new_parent, &decal_name, &name, &description, &icon, &rarity]) {
            Ok(_) => {},
            Err(err) => {
                println!("error: {}", err);
                println!("{}", new_parent);
                println!("{}", server_id);
            }
        }

        // println!("pbgid: {}", pbgid);
        // println!("attribute_id: {}", new_parent);
        // println!("screen_name: {}", screen_name);
        // println!("icon_name: {}", icon_name);
        // println!("help_text: {}", help_text);
        // println!("extra_text: {}", extra_text);
    }
}

fn get_locale_text(id: &str, locale: &HashMap<i32, String>) -> String {
    let id_i32 = id.parse::<i32>().unwrap();
    if let Some(locale_string) = locale.get(&id_i32) {
        locale_string.clone()
    } else {
        id.to_owned()
    }
}

// fn print_subtree(node: &Box<Node>, parent: String) {

//     let mut new_parent = parent.clone();

//     if !node.is_attribute_node() {
//         if &parent != "" {
//             new_parent.push_str("\\");
//         }

//         new_parent.push_str(&node.get_name());

//         for subnode in &node.get_all_children() {
//             print_subtree(subnode, new_parent.clone());
//         }
//     } else {
//         println!("{}", node.get_name());
//         let name = node.get_child("group.commander_bag")
//                        .get_child("locstring.name")
//                        .get_value("value");

//         let abilities = node.get_child("group.commander_bag")
//                             .get_child("list.commander_abilities")
//                             .get_all_children();

//         println!("{}", new_parent);
//         println!("{}", name);
//         for node in &abilities {
//             println!("{}", node.get_value("value"));
//         }
//     }
// }