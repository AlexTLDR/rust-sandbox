use crate::Status::Online;

#[derive(Debug)]
enum Status {
    Online,
    Offline,
    Connecting { attempts: u32 },
    Maintenance,
}

impl Status {
    fn is_online(&self) -> bool {
        matches!(self, Status::Online)
    }
}

#[derive(Debug)]
struct Device {
    id: u32,
    name: String,
    status: Status, // enum used as a field
    ip_address: String,
    location: String,
}
impl Device {
    fn new(id: u32, name: String, ip_address: String, location: String) -> Self {
        Self {
            id,
            name,
            status: Status::Offline, // default status
            ip_address,
            location,
        }
    }
    fn set_status(&mut self, new_status: Status) {
        self.status = new_status;
    }
    fn get_status_message(&self) -> String {
        match &self.status {
            Status::Online => format!("Device '{}' with id '{}' is online", self.name, self.id),
            Status::Offline => format!("Device '{}' with id '{}' is offline", self.name, self.id),
            Status::Connecting { attempts } => format!(
                "Device '{}' with id '{}' is connecting. Attempts: {}",
                self.name, self.id, attempts
            ),
            Status::Maintenance => format!(
                "Device '{}' with id '{}' is in maintenance",
                self.name, self.id
            ),
        }
    }
    fn change_ip(&mut self, ip: String) {
        self.ip_address = ip;
    }
    fn change_location(&mut self, location: String) {
        self.location = location;
    }
}

fn main() {
    let mut router = Device::new(
        101,
        String::from("Main Router"),
        String::from("192.168.1.1"),
        String::from("Server Room"),
    );
    println!("{}", router.get_status_message());
    router.set_status(Status::Connecting { attempts: 1 });
    println!("{}", router.get_status_message());
    router.set_status(Online);
    println!("{}", router.get_status_message());
    println!("Current device state: {:?}", router);
    router.change_ip(String::from("192.168.0.1"));
    router.change_location(String::from("GCP"));
    println!(
        "The new location of device named {} is {} and the new IP is {}",
        router.name, router.location, router.ip_address
    );
    let node = Status::Online;
    if node.is_online() {
        println!("The node is online")
    }
}
