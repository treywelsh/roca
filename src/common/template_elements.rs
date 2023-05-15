use std::fmt::Display;

/// Pair is a developer friendly representation of a key, value pair

#[derive(Debug)]
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
#[derive(Debug)]
pub struct Vector(pub String, pub Vec<Pair>);

impl Vector {
    pub fn new<T: Into<String>>(k: T) -> Self {
        Vector(k.into(), Vec::new())
    }

    pub fn add_pair<T: Into<String>>(&mut self, k: T, v: T) {
        self.1.push(Pair::new(k, v))
    }

    pub fn rm_pair<T: Into<String> + Copy>(&mut self, k: T) {
        self.1.retain(|pair| pair.0 != k.into());
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
