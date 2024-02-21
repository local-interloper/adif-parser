use std::error::Error;
use crate::contact::Contact;
use crate::tag::{AdifTag, get_tags};

#[derive(Default, Clone)]
struct Adif {
    pub adif_ver: Option<String>,
    pub created_timestamp: Option<String>,
    pub programid: Option<String>,
    pub programversion: Option<String>,
    pub userdef: Vec<AdifTag>,
    pub contacts: Vec<Contact>,
}

impl TryFrom<&Vec<AdifTag>> for Adif {
    type Error = Box<dyn Error>;

    fn try_from(tags: &Vec<AdifTag>) -> Result<Self, Self::Error> {
        let mut adif = Adif::default();

        let mut contact_buf = Contact::default();

        for tag in tags {
            match &tag.value {
                None => {
                    match tag.name.as_str() {
                        "EOH" => continue,
                        "EOR" => {
                            adif.contacts.push(contact_buf.clone());
                            continue;
                        }
                        _ => {}
                    }
                }
                Some(value) => { contact_buf.0.insert(tag.name.clone(), value.clone()); }
            }
        }

        Ok(adif)
    }
}

impl TryFrom<&String> for Adif {
    type Error = Box<dyn Error>;

    fn try_from(raw: &String) -> Result<Self, Self::Error> {
        let tags = get_tags(raw)?;
        Adif::try_from(&tags)
    }
}