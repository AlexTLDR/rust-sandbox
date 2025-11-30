const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // let mut missiles = STARTING_MISSILES;
    // let ready = READY_AMOUNT;
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT); //no need for mutable variables if we are not changing the value of missiles
    // missiles = missiles - ready;
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready);
}
