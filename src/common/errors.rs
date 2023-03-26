use std::{fmt::Display, num::ParseIntError};

#[derive(Debug)]
pub enum Errors {
    SXDPath(sxd_xpath::Error),
    SXDDocParser(sxd_document::parser::Error),
    ParseInt(ParseIntError),
    SeveralNodes(String),
    NotFound(String),
}

impl From<sxd_xpath::Error> for Errors {
    fn from(err: sxd_xpath::Error) -> Self {
        Self::SXDPath(err)
    }
}

impl From<sxd_document::parser::Error> for Errors {
    fn from(err: sxd_document::parser::Error) -> Self {
        Self::SXDDocParser(err)
    }
}

impl From<ParseIntError> for Errors {
    fn from(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SXDPath(e) => write!(f, "roca: XML path: {}", e),
            Self::SXDDocParser(e) => write!(f, "roca: xml document: {}", e),
            Self::SeveralNodes(key) => write!(f, "roca: several keys found: {}", key),
            Self::NotFound(key) => write!(f, "roca: key wasn't found: {}", key),
            Self::ParseInt(e) => write!(f, "roca: Failed to parse as integer: {}", e),
        }
    }
}
