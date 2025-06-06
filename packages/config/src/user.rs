pub struct User {
    name: String,
    age: u8,
}

impl User {
    pub fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
