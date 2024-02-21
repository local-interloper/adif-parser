use std::error::Error;
use std::fmt::{Display, Formatter, Write};

#[derive(Debug)]
pub enum AdifParseErrorKind {
    IndexingError,
    InvalidTag(String),
}

#[derive(Debug)]
pub struct AdifParseError {
    pub kind: AdifParseErrorKind,
}

impl Error for AdifParseError {}

impl Display for AdifParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            AdifParseErrorKind::InvalidTag(s) => f.write_fmt(format_args!("Invalid tag: {}", s)),
            AdifParseErrorKind::IndexingError => f.write_str("Indexing error"),
        }
    }
}

impl AdifParseError {
    pub fn new(kind: AdifParseErrorKind) -> Self {
        Self {
            kind
        }
    }
}
