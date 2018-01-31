#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;
extern crate time;

use serde_xml_rs::deserialize;
use serde_xml_rs::serialize;
use std::fs::File;
use std::io::prelude::*;
use time::PreciseTime;

#[derive(Debug, Deserialize, Serialize)]
struct Item {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Age")]
    pub age: String,
}

#[derive(Debug, Deserialize)]
struct Onix {
    #[serde(rename = "Item", default)]
    pub items: Vec<Item>,
}

fn main() {
    let start = PreciseTime::now();
    let mut contents = String::new();
    let mut f = File::open("file.xml").unwrap();

    f.read_to_string(&mut contents).expect("failed to parse");
    let onix: Onix = deserialize(contents.as_bytes()).unwrap();

    for (idx, item) in onix.items.iter().enumerate() {
        let fname = format!("file-part-{}.xml", idx);
        let mut new_file = File::create(fname).unwrap();
        serialize(item, new_file).unwrap();
        println!("{},{}", idx, item.name);
    }
    let end = PreciseTime::now();
    println!("Ran in {} seconds", start.to(end));
}
