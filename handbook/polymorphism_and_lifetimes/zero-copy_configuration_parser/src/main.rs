// We define a lifetime 'a so the returned Self can hold references to 'input'
trait Parse<'a> {
    fn parse(input: &'a str) -> Self;
}

// This function works for ANY type T that implements Parse
// The <'a> ensures that the returned T cannot outlive the input string.
fn load_configuration<'a, T>(input: &'a str) -> T
where
    T: Parse<'a>,
{
    println!("Loading configuration...");
    T::parse(input)
}

struct ServerConfig<'a> {
    address: &'a str,
    port: &'a str,
}

impl<'a> Parse<'a> for ServerConfig<'a> {
    fn parse(input: &'a str) -> Self {
        // We look for a space to separate the address and port.
        // Example Input: "localhost 8080"
        let index = input.find(' ').unwrap_or(input.len());
        // ZERO-COPY MAGIC:
        // We use slicing syntax to create references that point directly
        // into the original 'input' memory buffer.
        // No new strings are allocated here!
        ServerConfig {
            address: &input[..index],  // Borrow the first part
            port: &input[index + 1..], // Borrow the second part
        }
    }
}

fn main() {
    // An owned String — the parser will borrow slices from it (zero-copy)
    let input = String::from("localhost 8080");

    // The type annotation is what picks T = ServerConfig
    let config: ServerConfig = load_configuration(&input);

    // These are &str slices pointing directly INTO `input` — nothing was copied
    println!("Address: {}", config.address);
    println!("Port: {}", config.port);

    // `input` must stay alive as long as `config` is in use — that's the
    // lifetime 'a doing its job. Drop order here is fine: reverse of declaration.
}
