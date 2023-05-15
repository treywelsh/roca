use std::{fmt::Display, num::ParseIntError};

#[derive(Debug)]
pub enum Errors {
    XMLDoc(xml_doc::Error),
    ParseInt(ParseIntError),
    HasChilds(String),
    //SeveralNodes(String),  find method returns only the first, should use find_all ?
    NotFound(String),
    XMLRPC(serde_xmlrpc::Error),
    OpenNebula(String),
    HTTP(reqwest::Error),
    Roca(String),
}

impl From<xml_doc::Error> for Errors {
    fn from(err: xml_doc::Error) -> Self {
        Self::XMLDoc(err)
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
            // TODO: move this in my xml-doc github fork:
            Self::XMLDoc(e) => match e {
                xml_doc::Error::Io(e) => write!(f, "roca: xml IO error: {}", e),
                xml_doc::Error::CannotDecode => write!(f, "roca: XML decoding error"),
                xml_doc::Error::MalformedXML(e) => write!(f, "roca: XML malformed: {}", e),
                xml_doc::Error::ContainerCannotMove => {
                    write!(f, "roca: container element cannot have a parent.")
                }
                xml_doc::Error::HasAParent => {
                    write!(f, "roca: needs to detach element before assigning a parent")
                }
            },

            //Self::SeveralNodes(key) => write!(f, "roca: several keys found: {}", key),
            Self::HasChilds(key) => write!(f, "roca: this node has childs: {}", key),
            Self::NotFound(key) => write!(f, "roca: key wasn't found: {}", key),
            Self::ParseInt(e) => write!(f, "roca: Failed to parse as integer: {}", e),
            Self::XMLRPC(e) => write!(f, "roca: XML-RPC error: {}", e),
            Self::HTTP(e) => write!(f, "roca: HTTP error: {}", e),
            Self::OpenNebula(e) => write!(f, "roca: OpenNebula error: {}", e),
            Self::Roca(e) => write!(f, "roca library internal error: {}", e),
        }
    }
}
