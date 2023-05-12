use std::fmt::Display;

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

impl Display for TemplateBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pairs = Vec::new();
        for p in self.pairs.iter() {
            pairs.push(format!("{}", p));
        }
        let mut vectors = Vec::new();
        for p in self.vectors.iter() {
            vectors.push(format!("{}", p));
        }
        writeln!(f, "{}", pairs.join("\n"))?;
        write!(f, "{}", vectors.join("\n"))
    }
}

#[derive(Debug)]

pub struct TemplateBuilder {
    pairs: Vec<Pair>,
    vectors: Vec<Vector>,
}

impl TemplateBuilder {
    pub fn new() -> Self {
        Self {
            pairs: Vec::new(),
            vectors: Vec::new(),
        }
    }

    pub fn add_pair<T: Into<String>>(&mut self, k: T, v: T) {
        self.pairs.push(Pair::new(k, v));
    }

    //pub fn add_vector<T: Into<String>>(&mut self, v: Vector) {
    /// Add an existing vector into the template builder
    pub fn add_vector(&mut self, v: Vector) {
        self.vectors.push(v);
    }

    /// Create an empty vector, add it to the template, and return a mutable reference to it
    pub fn make_vector<T: Into<String>>(&mut self, k: T) -> &mut Vector {
        let v = Vector::new(k);
        self.vectors.push(v);
        self.vectors.last_mut().unwrap()
    }

    pub fn rm_pair<T: Into<String> + Copy>(&mut self, k: T) {
        self.pairs.retain(|pair| pair.0 != k.into());
    }

    pub fn rm_vector<T: Into<String> + Copy>(&mut self, k: T) {
        self.vectors.retain(|pair| pair.0 != k.into());
    }

    // Will remove both: Pairs and vectors
    pub fn rm<T: Into<String> + Copy>(&mut self, k: T) {
        self.rm_pair(k);
        self.rm_vector(k);
    }
}

impl Default for TemplateBuilder {
    fn default() -> Self {
        Self::new()
    }
}
