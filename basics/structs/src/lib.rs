// 1. Create a struct named `Polygon` with the fields and their types listed below. Then build the
// program with `cargo build` to ensure you don't have any syntax errors.
//
// - name - String
// - sides - u32
// - visible - bool

pub struct Polygon {
    pub name: String,
    sides: u32,
    pub visible: bool,
}

// 2. Create an implementation block for the `Polygon` struct.
//
// In the implementation block define an associated function named `new` that:
// - accepts an argument `name` of type `String`
// - returns a `Polygon` (you may use `Self` as an alias for `Polygon` inside of the `impl` block)
//   - with `name` set to the value from the `name` argument.
//   - with `sides` set to `3`
//   - with `visible` set to `true`
//
// NOTE: Associated functions do NOT take a form a `self` as their first argument (that would turn
// the function into a method)
//
// Then build the program with `cargo build` to ensure you don't have any syntax errors.

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
            3 => "triangle",
            4 => "square",
            5 => "pentagon",
            _ => "polygon",
        }
        .to_string()
    }
    pub fn increment_sides(&mut self) {
        self.sides += 1;
    }
    pub fn sides(&self) -> u32 {
        self.sides
    }
}
