use std::fs;
use std::path::Path; 
use std::thread;
use std::sync::mpsc; 
use std::time::Instant; 
use std::io::Write;
mod lib; 
mod generate_data;

/**
 * Writes a string of data to a summary file.
 *
 * This function opens (or creates) a summary file located at 
 * `data/weekly_summary/weekly_sales_summary.txt` and appends the provided data
 * to it. If the file or directory doesn't exist, they will be created as needed.
 *
 * Parameters:
 * - `data`: The sales data string to append to the summary file.
 *
 * Returns:
 * - `Ok(())` if the operation succeeds, or an `io::Error` if it fails.
 */
fn write_to_summary_file(data: &str) -> Result<(), std::io::Error> {
    let summary_file_path = "data/weekly_summary/weekly_sales_summary.txt";
    let mut file = std::fs::File::options()
        .create(true) // Create the file if it doesn't exist.
        .append(true) // Append to the file if it exists.
        .open(summary_file_path)?; // Open the file for writing.

    writeln!(file, "{}", data)?;
    Ok(())
}

/**
 * Main function for generating branch sales data, processing it concurrently,
 * and logging results to a summary file.
 *
 * This function first checks if the branch folders and sales data exist. 
 * If not, it generates them using the `generate_data` module. It then splits 
 * the branch data into groups and processes them in parallel using threads.
 * The processed sales data is sent back via channels, and the results are 
 * written to a summary file.
 *
 * - Multithreading is employed to process sales data for multiple branches 
 *   concurrently, improving performance.
 * - Logs processing details and errors to the console.
 */
fn main() {
    env_logger::init(); 

    let first_branch_folder = "data/ALBNM";
    if !Path::new(first_branch_folder).exists() {
        println!("Branch folders not found. Generating branch folders and sales data...");
    }

    let branch_codes = generate_data::generate_branch_data();

    let output_folder = "data/weekly_summary";
    if !Path::new(output_folder).exists() {
        fs::create_dir(output_folder).expect("Failed to create weekly_summary folder");
    }

    // Create a channel for sending data between threads.
    let (tx, rx) = mpsc::channel();

    // Divide the branch codes into groups of 10 for concurrent processing.
    let branch_groups: Vec<Vec<String>> = branch_codes
        .chunks(10)
        .map(|chunk| chunk.iter().map(|&s| s.to_string()).collect())
        .collect();

    // Start the timer to measure the total processing time.
    let start = Instant::now();

    // A vector to hold the thread handles.
    let mut handles = vec![];

    // Spawn a thread for each group of branches to process the sales data.
    for group in branch_groups {
        let tx = tx.clone(); 

        let handle = thread::spawn(move || {
            // Process the input files for the group of branches and send the results via the channel.
            if let Err(e) = lib::process_input_file(&group, tx) {
                eprintln!("Thread failed to process input: {:?}", e); 
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Failed to join thread: {:?}", e); 
        }
    }

    // Drop the transmitter to signal that no more data will be sent.
    drop(tx);

    // Receive data from the channel and write it to the summary file.
    while let Ok(received) = rx.recv() {
        println!("Received: {}", received);
        write_to_summary_file(&received).expect("Failed to write to summary file");
    }

    // Measure and display the total time taken for processing.
    let duration = start.elapsed();
    println!("Total time for processing: {:?}", duration);

    println!("Phew! I am done.");
}
