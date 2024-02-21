use std::error::Error;
use regex::Regex;
use crate::error::AdifParseError;
use crate::error::AdifParseErrorKind::IndexingError;

#[derive(Clone)]
pub struct AdifTag {
    pub name: String,
    pub value: Option<String>,
}

pub fn get_tags(str: &str) -> Result<Vec<AdifTag>, Box<dyn Error>> {
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
            tags.push(AdifTag {
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

        tags.push(AdifTag {
            name: key,
            value: Some(value),
        });
    }

    Ok(tags)
}