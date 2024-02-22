use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::contact::Contact;
use crate::tag::{Tag, get_tags};

#[derive(Default, Clone)]
pub struct Adif {
    pub adif_ver: Option<String>,
    pub created_timestamp: Option<String>,
    pub programid: Option<String>,
    pub programversion: Option<String>,
    pub userdef: Vec<Tag>,
    pub contacts: Vec<Contact>,
}

impl TryFrom<&Vec<Tag>> for Adif {
    type Error = Box<dyn Error>;

    fn try_from(tags: &Vec<Tag>) -> Result<Self, Self::Error> {
        let mut adif = Adif::default();

        let mut contact_buf = Contact::default();

        for tag in tags {
            let Some(value) = &tag.value else {
                if tag.name.as_str() == "EOR" { adif.contacts.push(contact_buf.clone()) }
                continue;
            };

            if tag.name.starts_with("USERDEF") {
                adif.userdef.push(tag.clone());
                continue;
            }

            match tag.name.as_str() {
                "ADIF_VER" => { adif.adif_ver = Some(value.clone()); }
                "CREATED_TIMESTAMP" => { adif.created_timestamp = Some(value.clone()); }
                "PROGRAMID" => { adif.programid = Some(value.clone()); }
                "PROGRAMVERSION" => { adif.programversion = Some(value.clone()); }
                _ => { contact_buf.0.insert(tag.name.clone(), value.clone()); }
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

impl Display for Adif {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for contact in &self.contacts {
            f.write_fmt(format_args!("{}", contact))?;
        }

        Ok(())
    }
}