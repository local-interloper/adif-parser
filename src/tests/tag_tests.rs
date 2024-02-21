use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};
use crate::tag::get_tags;

#[cfg(test)]

#[test]
fn parse_tags() {
    let file = OpenOptions::new()
        .read(true)
        .open("data/log.adi")
        .expect("Unable to open test data");

    let mut raw = String::new();
    BufReader::new(file).read_to_string(&mut raw).expect("Failed to read data");

    let tags = get_tags(&raw).expect("Failed to parse tags");

    tags.iter().for_each(|tag| println!("{} {:?}", tag.name, tag.value));
}
