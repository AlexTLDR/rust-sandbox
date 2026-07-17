use traits::{Brew, EspressoMachine, Moka};

fn main() {
    // let pot = Moka { size: 3 };
    // let coffee = pot.extract();
    // println!("{}", coffee);

    let machines: Vec<Box<dyn Brew>> = vec![
        Box::new(Moka { size: 6 }),
        Box::new(EspressoMachine { pressure: 9 }),
    ];
    for machine in machines {
        println!("Coffee shop says: {}", machine.extract());
    }
}
