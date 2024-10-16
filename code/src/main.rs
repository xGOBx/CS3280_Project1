use std::fs::{self, File, OpenOptions};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Instant, Duration};
use log::{info, error};
use fern::Dispatch;
mod generate_data;


/**
 * Main function.
 * 
 * Entry point for the program, which calls the `run` method to initiate processing.
 */
fn main() {
    run();
}


/**
 * The main runner method that orchestrates the entire processing.
 * 
 * This method initializes logging, prepares output folders, retrieves branch codes,
 * sets up the shared summary file, and handles concurrent processing. It acts as the
 * main entry point to run all core functions in the correct order.
 */
fn run() {
    setup_logger().expect("Failed to initialize logger");

    let output_folder = "data/weekly_summary";
    prepare_output_folder(output_folder);

    info!("Starting to process folders.");

    let branch_codes = get_branch_codes();
    let summary_file_mutex = initialize_summary_file();

    let branch_groups: Vec<Vec<String>> = branch_codes
        .chunks(10)
        .map(|chunk| chunk.iter().map(|s| s.to_string()).collect())
        .collect();

    let start = Instant::now();
    process_branches_concurrently(branch_groups, summary_file_mutex);
    measure_processing_time(start);

    info!("Finished processing folders.");
}




/**
 * Initializes the logging system to log messages only to a file.
 * 
 * This function sets up a logging configuration that formats log entries
 * with timestamps, log levels, and the message content. The logs are saved
 * to `log.txt`, and it uses the `fern` crate for flexibility.
 *
 * Returns:
 * - `Ok(())` if the logger is successfully set up.
 * - `Err(Box<dyn std::error::Error>)` if there is an issue setting up the logger.
 */
fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::log_file("log.txt").map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?) // Log only to file
        .apply()?;
    Ok(())
}

/**
 * Prepares the output folder for summary files.
 * 
 * This function checks if the `weekly_summary` folder exists, and if so, deletes it to
 * ensure a clean start. It then recreates the folder to store new summary files. 
 * It logs the outcome of these operations to indicate success or failure.
 *
 * Parameters:
 * - `output_folder`: A string slice specifying the path to the output folder.
 */
fn prepare_output_folder(output_folder: &str) {
    if Path::new(output_folder).exists() {
        match fs::remove_dir_all(output_folder) {
            Ok(_) => info!("Existing weekly_summary folder deleted."),
            Err(e) => error!("Failed to delete weekly_summary folder: {}", e),
        }
    }

    match fs::create_dir_all(output_folder) {
        Ok(_) => info!("Weekly summary folder created successfully."),
        Err(e) => error!("Failed to create weekly_summary folder: {}", e),
    }
}

/**
 * Determines whether to generate new branch data or load existing data.
 * 
 * This function checks if branch folders already exist. If they don't, it generates
 * new sales data for the branches. If they do, it loads existing data and returns 
 * the branch codes. It logs actions taken to inform the user of the workflow.
 *
 * Returns:
 * - A vector of strings containing branch codes.
 */
fn get_branch_codes() -> Vec<String> {
    let first_branch_folder = "data/ALBNM";
    let branch_codes: Vec<String>;

    if !Path::new(first_branch_folder).exists() {
        println!("Branch folders not found. Generating branch folders and sales data...");
        branch_codes = generate_data::generate_branch_data()
            .iter()
            .map(|&s| s.to_string())
            .collect();
    } else {
        println!("Existing branch data found. Loading data...");
        let existing_data = generate_data::load_existing_branch_data();
        if existing_data.is_empty() {
            println!("No existing data found, generating new data.");
            branch_codes = generate_data::generate_branch_data()
                .iter()
                .map(|&s| s.to_string())
                .collect();
        } else {
            println!("Loaded existing data.");
            branch_codes = existing_data
                .iter()
                .map(|s| s.split(',').next().unwrap().trim().to_string())
                .collect();
        }
    }

    branch_codes
}

/**
 * Sets up a shared, thread-safe summary file using a Mutex.
 * 
 * This function opens a file for appending sales summaries and wraps it in an `Arc<Mutex>`
 * to allow safe, concurrent access across multiple threads. This ensures that the data
 * can be written safely without conflicts between threads.
 *
 * Returns:
 * - An `Arc<Mutex<File>>` that can be shared across threads for thread-safe file writing.
 */
fn initialize_summary_file() -> Arc<Mutex<File>> {
    let summary_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("data/weekly_summary/weekly_sales_summary.txt")
        .expect("Failed to open summary file");
    Arc::new(Mutex::new(summary_file))
}

/**
 * Processes branch groups concurrently by spawning threads.
 * 
 * This function takes groups of branch codes and processes their sales data concurrently
 * by spawning threads. Each thread will process a group and write the results directly to 
 * a shared summary file. Errors are logged for threads that fail to process their data.
 *
 * Parameters:
 * - `branch_groups`: A vector of vectors, where each inner vector contains branch codes for a group.
 * - `summary_file_mutex`: An `Arc<Mutex<File>>` for thread-safe writing to the summary file.
 */
fn process_branches_concurrently(branch_groups: Vec<Vec<String>>, summary_file_mutex: Arc<Mutex<File>>) {
    let mut handles = vec![];

    for group in branch_groups {
        let summary_file_mutex = Arc::clone(&summary_file_mutex);

        let handle = thread::spawn(move || {
            if let Err(e) = cs3280_project1::process_input_file(&group, summary_file_mutex) {
                error!("Thread failed to process input: {:?}", e); 
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.join() {
            error!("Failed to join thread: {:?}", e); 
        }
    }
}


/**
 * Measures and logs the total processing time.
 * 
 * This utility function calculates the elapsed time from a provided starting point 
 * (an `Instant`). It then displays the time taken to process the data, offering insight 
 * into performance metrics for the operation.
 *
 * Parameters:
 * - `start`: The `Instant` from which to measure elapsed time.
 *
 * Returns:
 * - A `Duration` object representing the total time taken.
 */
fn measure_processing_time(start: Instant) -> Duration {
    let duration = start.elapsed();
    println!("Total time for processing: {:?}", duration);
    duration
}

