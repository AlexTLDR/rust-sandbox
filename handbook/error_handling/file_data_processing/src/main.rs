use anyhow::{Context, Result}; // Using anyhow for app level and context
use log::{error, info, warn};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use thiserror::Error; // Using thiserror for specific processing errors
// Specific error for this processing logic
#[derive(Error, Debug)]
enum ProcessingError {
    #[error("Invalid number format on line {line_num}: '{content}'")]
    InvalidFormat {
        line_num: usize,
        content: String,
        #[source] // Original source error
        source: std::num::ParseIntError,
    },
    #[error("Negative number {number} not allowed on line {line_num}")]
    NegativeNumber { line_num: usize, number: i32 },
    // Could add other specific errors here
}
// Function to process a single line
fn process_line(line_content: &str, line_num: usize) -> Result<i32, ProcessingError> {
    let number =
        line_content
            .trim()
            .parse::<i32>()
            .map_err(|e| ProcessingError::InvalidFormat {
                line_num,
                content: line_content.to_string(),
                source: e, // Include original parse error
            })?;
    if number < 0 {
        Err(ProcessingError::NegativeNumber { line_num, number })
    } else {
        Ok(number)
    }
}
// Main file processing function
fn process_file_and_sum(filename: &str) -> anyhow::Result<i32> {
    info!("Starting file processing: {}", filename);
    let file = File::open(filename).context(format!("Failed to open file '{}'", filename))?; // Anyhow context
    let reader = BufReader::new(file);
    let mut total_sum = 0;
    let mut lines_processed = 0;
    for (index, line_result) in reader.lines().enumerate() {
        let line_num = index + 1;
        let line = line_result.context(format!("Failed reading line {}", line_num))?; // Anyhow context for I/O
        match process_line(&line, line_num) {
            Ok(number) => {
                total_sum += number;
                lines_processed += 1;
            }
            Err(e) => {
                // Log the specific error but continue with other lines
                warn!("Error processing line {}: {} - Skipping line.", line_num, e);
                // Could also choose to stop here by returning Err(e.into())
                // which works if ProcessingError impls Error, as anyhow::Error impls From<E: Error>
            }
        }
    }
    info!(
        "File processing complete. Lines processed: {}. Total sum: {}",
        lines_processed, total_sum
    );
    Ok(total_sum)
}
fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    // Create a test file
    use std::io::Write;
    let mut file = File::create("test_data.txt")?;
    writeln!(file, "10")?;
    writeln!(file, "25")?;
    writeln!(file, "-5")?; // Error: negative
    writeln!(file, "abc")?; // Error: format
    writeln!(file, "15")?;
    drop(file); // Ensure file is closed
    match process_file_and_sum("test_data.txt") {
        Ok(sum) => {
            info!("Final sum calculated: {}", sum); // Should be 10 + 25 + 15 = 50
            assert_eq!(sum, 50); // Add a check
            println!("Sum calculated successfully: {}", sum);
        }
        Err(e) => {
            // Errors here would likely be I/O errors opening the file,
            // as internal errors are logged but not propagated by process_file_and_sum
            error!("Critical error during file processing: {:?}", e);
        }
    }
    std::fs::remove_file("test_data.txt")?; // Clean up test file
    Ok(())
}
