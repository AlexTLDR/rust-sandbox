pub trait Brew {
    fn extract(&self) -> String;
    fn clean(&self) -> String {
        String::from("Cleaning with a simple hot water rinse.")
    }
}

pub struct Moka {
    pub size: u8,
}
pub struct EspressoMachine {
    pub pressure: u8,
}
impl Brew for Moka {
    fn extract(&self) -> String {
        format!(
            "Bubbling up some coffee... ready to serve {} cups of coffee",
            self.size
        )
    }
}

impl Brew for EspressoMachine {
    fn extract(&self) -> String {
        format!("Extracting at {} bars of pressure.", self.pressure)
    }

    fn clean(&self) -> String {
        String::from("Running automatic descaling program.")
    }
}
