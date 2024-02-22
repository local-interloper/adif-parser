use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};
use crate::adif::Adif;
use crate::tag::{get_tags, Tag};

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
}

#[test]
fn adif_from_tags() {
    let file = OpenOptions::new()
        .read(true)
        .open("data/log.adi")
        .expect("Unable to open test data");

    let mut raw = String::new();
    BufReader::new(file).read_to_string(&mut raw).expect("Failed to read data");

    let tags = get_tags(&raw).expect("Failed to retrieve tags");

    let adif = Adif::try_from(&tags).expect("Failed to create an ADIF from tags");

    println!("{}", adif);
}

#[test]
fn tag_to_string() {
    let mut str = String::new();
    let mut tag = Tag {
        name: "RST_SENT".to_string(),
        value: Some("599".to_string()),
    };


    str = tag.into();
    assert_eq!("<RST_SENT:3>599".to_string(), str);

    tag = Tag {
        name: "EOR".to_string(),
        value: None
    };

    str = tag.into();
    assert_eq!("<EOR>".to_string(), str);
}