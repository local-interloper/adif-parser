use std::error::Error;
use std::fmt::{Display, Formatter};
use regex::Regex;
use crate::error::AdifParseError;
use crate::error::AdifParseErrorKind::IndexingError;

#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub value: Option<String>,
}

pub fn get_tags(str: &str) -> Result<Vec<Tag>, Box<dyn Error>> {
    let mut tags = Vec::new();
    let reg = Regex::new("<.*?>")?;
    let matches = reg.find_iter(str);

    for r#match in matches {
        let mut raw = r#match.as_str();
        let raw_length = raw.len();
        raw = raw
            .get(1..raw_length - 1)
            .ok_or(AdifParseError::new(IndexingError))?;

        if !raw.contains(':') {
            tags.push(Tag {
                name: raw.to_string(),
                value: None,
            });

            continue;
        }

        let mut split = raw.split(':');

        let key = split
            .nth(0)
            .ok_or(AdifParseError::new(IndexingError))?
            .to_uppercase();

        let length: usize = split
            .nth(0)
            .ok_or(AdifParseError::new(IndexingError))?
            .parse()?;

        let value = str
            .get(r#match.end()..r#match.end() + length)
            .ok_or(AdifParseError::new(IndexingError))?
            .to_string();

        tags.push(Tag {
            name: key,
            value: Some(value),
        });
    }

    Ok(tags)
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}: {}",
            self.name,
            match &self.value {
                Some(value) => value.as_str(),
                None => "None"
            }
        ))
    }
}

impl Into<String> for Tag {
    fn into(self) -> String {
        match self.value {
            Some(value) => format!("<{}:{}>{}", self.name, value.len(), value),
            None => format!("<{}>", self.name)
        }
    }
}