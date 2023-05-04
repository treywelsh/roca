use std::{fmt::Display, num::ParseIntError};

#[derive(Debug)]
pub enum Errors {
    SXDPath(sxd_xpath::Error),
    SXDDocParser(sxd_document::parser::Error),
    ParseInt(ParseIntError),
    SeveralNodes(String),
    NotFound(String),
    XMLRPC(serde_xmlrpc::Error),
    OpenNebula(String),
    HTTP(reqwest::Error),
    Roca(String),
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

impl From<serde_xmlrpc::Error> for Errors {
    fn from(err: serde_xmlrpc::Error) -> Self {
        Self::XMLRPC(err)
    }
}

impl From<reqwest::Error> for Errors {
    fn from(err: reqwest::Error) -> Self {
        Self::HTTP(err)
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
            Self::XMLRPC(e) => write!(f, "roca: XML-RPC error: {}", e),
            Self::HTTP(e) => write!(f, "roca: HTTP error: {}", e),
            Self::OpenNebula(e) => write!(f, "roca: OpenNebula error: {}", e),
            Self::Roca(e) => write!(f, "roca library internal error: {}", e),
        }
    }
}
