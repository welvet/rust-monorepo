pub struct Cat {
    pub name: String
}

impl Cat {
    pub fn meow(&self) -> String {
        format!("{}", self.name)
    }
}
