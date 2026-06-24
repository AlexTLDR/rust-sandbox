use crate::Status::Online;

#[derive(Debug)]
enum Status {
    Online,
    Offline,
    Connecting { attempts: u32 },
    Maintenance,
}

#[derive(Debug)]
struct Device {
    id: u32,
    name: String,
    status: Status, // enum used as a field
}
impl Device {
    fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            status: Status::Offline, // default status
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
}

fn main() {
    let mut router = Device::new(101, String::from("Main Router"));
    println!("{}", router.get_status_message());
    router.set_status(Status::Connecting { attempts: 1 });
    println!("{}", router.get_status_message());
    router.set_status(Online);
    println!("{}", router.get_status_message());
    println!("Current device state: {:?}", router)
}
