use std::fmt::Display;

use crate::common::errors::Errors;

/// Pair is a developer friendly representation of a key, value pair

#[derive(Debug, Clone)]
pub struct Pair(pub String, pub String);

impl Pair {
    pub fn new<T: Into<String>>(k: T, v: T) -> Self {
        Pair(k.into(), v.into())
    }
}

//impl AsRef<String> for Pair {
//    fn as_ref(&self) -> &String {
//        &format!("{}=\"{}\"", self.0, self.1)
//    }
//}
//
//impl Into<String> for Pair {
//    fn into(self) -> String {
//        format!("{}=\"{}\"", self.0, self.1)
//    }
//}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}=\"{}\"", self.0, self.1)
    }
}

/// Vector is a developer friendly representation of a collection of attributes
#[derive(Debug, Clone)]
pub struct Vector(pub String, pub Vec<Pair>);

impl Vector {
    pub fn new<T: Into<String>>(k: T) -> Self {
        Vector(k.into(), Vec::new())
    }

    pub fn put_str<T: Into<String>>(&mut self, k: T, v: T) {
        self.1.push(Pair::new(k, v))
    }

    pub fn get_str<T: Into<String>>(&self, k: T) -> Result<&str, Errors> {
        let key = k.into();
        let pairs: Vec<&Pair> = self.1.iter().filter(|&p| p.0 == key).collect();
        if pairs.len() > 1 {
            Err(Errors::HasChilds(key))
        } else if pairs.is_empty() {
            Err(Errors::NotFound(key))
        } else {
            Ok(pairs[0].1.as_str())
        }
    }

    pub fn get_i64(&self, name: &str) -> Result<i64, Errors> {
        let value_str = self.get_str(name)?;

        let v = value_str.parse()?;
        Ok(v)
    }

    pub fn rm<T: Into<String> + Copy>(&mut self, k: T) -> Result<(), Errors> {
        let pairs_len = self.1.len();
        self.1.retain(|pair| pair.0 != k.into());
        if pairs_len == self.1.len() {
            return Err(Errors::Template(format!(
                "can't remove {} from template: not found",
                k.into(),
            )));
        }

        Ok(())
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}=[", self.0)?;
        let mut pairs = Vec::new();
        for p in self.1.iter() {
            pairs.push(format!("{}", p));
        }
        write!(f, "{}", pairs.join(",\n"))?;
        write!(f, " ]")
    }
}
