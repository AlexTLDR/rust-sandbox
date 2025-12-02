pub struct Polygon {
    pub name: String,
    pub sides: u32,
    pub visible: bool,
}

impl Polygon {
    pub fn new(name: String) -> Self {
        Self {
            name,
            sides: 3,
            visible: true,
        }
    }
    pub fn shape(&self) -> String {
        match self.sides {
            3 => String::from("triangle"),
            4 => String::from("square"),
            5 => String::from("pentagon"),
            _ => String::from("polygon"),
        }
    }
    pub fn increment_sides(&mut self) {
        self.sides += 1;
    }
}