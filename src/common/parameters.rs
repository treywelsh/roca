pub enum UpdateType {
    Replace = 0,
    Merge = 1,
}

impl UpdateType {
    pub fn value(self) -> i32 {
        self as i32
    }
}
