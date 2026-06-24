#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
impl TrafficLight {
    fn from_str(color: &str) -> Option<TrafficLight> {
        match color.to_lowercase().as_str() {
            "red" => Some(TrafficLight::Red),
            "green" => Some(TrafficLight::Green),
            "yellow" => Some(TrafficLight::Green),
            _ => None,
        }
    }
    fn is_safe_to_go(color: &str) -> Option<bool> {
        match color.to_lowercase().as_str() {
            "green" => Some(true),
            "yellow" => Some(false),
            "red" => Some(false),
            _ => None,
        }
    }
}
fn main() {
    let green_light = TrafficLight::from_str("green");
    let invalid_light = TrafficLight::from_str("blue");
    println!(
        "Where there is Some -> calling green_light {:?} and when is None, calling invalid_light {:?}",
        green_light, invalid_light
    );

    if let Some(light) = TrafficLight::from_str("Red") {
        println!("Successfully created from 'Red':{:?}", light);
    } else {
        println!("Could not create light from Red");
    }
    println!(
        "If the color is green, it is {:?} that it is safe to go, but if the color is red, then it is {:?}",
        TrafficLight::is_safe_to_go("green"),
        TrafficLight::is_safe_to_go("red")
    );
}
