use anyhow::Result;
use env_logger::{Builder, Env};
use log::{debug, error, info, trace, warn};

fn risky_operation(value: i32) -> Result<()> {
    if value < 0 {
        // Log a warning before returning the error
        warn!("Attempting operation on negative value: {}", value);
        anyhow::bail!("Value cannot be negative: {}", value);
    }
    debug!("Performing operation on value: {}", value);
    // ... operation logic ...
    Ok(())
}
// Import macros // Using anyhow for simplicity in examples
fn main() -> Result<()> {
    // Initialize env_logger. By default, reads RUST_LOG env var.
    // We can set a default level if RUST_LOG is not defined.
    Builder::from_env(Env::default().default_filter_or("info")) // Default to 'info' if RUST_LOG isn't set
        .init();
    info!("Application started."); // Logged at INFO level
    match risky_operation(10) {
        Ok(_) => info!("Operation successful."),
        Err(e) => {
            // Log the error at ERROR level. {:?} with anyhow shows the cause chain.
            error!("Operation failed: {:?}", e);
        }
    }
    match risky_operation(-5) {
        Ok(_) => info!("Operation (negative) successful."),
        Err(e) => {
            error!("Operation (negative) failed: {:?}", e);
        }
    }
    debug!("This is detailed debug information."); // Only visible if RUST_LOG=debug or trace
    trace!("This message is very verbose."); // Only visible if RUST_LOG=trace
    info!("Application finished.");
    Ok(())
}

/*
# In your terminal, in your project's root directory:
# Run your program, showing only INFO, WARN, and ERROR messages (a good default).
RUST_LOG=info cargo run
# Show more detailed DEBUG messages and everything above (info, warn, error).
RUST_LOG=debug cargo run
# Show only WARNING and ERROR messages.
RUST_LOG=warn cargo run
# Show DEBUG messages only for your own crate's code, keeping libraries quieter.
# Replace 'your_crate_name' with the actual name of your package from Cargo.toml.
RUST_LOG=your_crate_name=debug cargo run
# Show everything (can be very verbose, useful for deep debugging).
RUST_LOG=trace cargo run

*/
