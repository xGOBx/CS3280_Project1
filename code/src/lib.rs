use std::fs::File; 
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::sync::mpsc::Sender; 
use log::{error, info};

/**
 * Processes input files located in specified folders, reads sales data, 
 * and sends summaries of total sales back to the main thread.
 *
 * This function opens each folder specified in the `folders` array, looks for
 * a file named `branch_weekly_sales.txt` inside a "data" directory, and calculates
 * the total sales of a specific product ("PROD001") for that branch.
 * The result is sent back to the main thread using the `tx` channel.
 *
 * Logging:
 * - Writes logs to a file `log.txt` for tracking processed folders and any errors.
 *
 * Parameters:
 * - `folders`: A slice of folder names (strings) where the sales data files are located.
 * - `tx`: A `Sender` channel for sending the processed summary back to the main thread.
 *
 * Returns:
 * - A `Result` that is `Ok(String)` if the function completes successfully, or an `io::Error` if any I/O operation fails.
 *
 * Errors:
 * - If the function encounters missing files, file access errors, or parsing issues, 
 *   it logs the errors and continues processing the remaining folders.
 * - Errors related to file creation or writing to the log file will stop execution and return an `io::Error`.
 */
pub fn process_input_file(folders: &[String], tx: Sender<String>) -> Result<String, io::Error> {
    let log_file = "log.txt"; 
    let mut log = File::options()
        .create(true)
        .append(true)
        .open(log_file)?;

    for folder in folders {
        let mut file_path = PathBuf::from("data"); 
        file_path.push(folder); 
        file_path.push("branch_weekly_sales.txt");

        println!("Looking for file at: {:?}", file_path); 

        // Check if the file exists, log an error if not.
        if !file_path.exists() {
            let err_msg = format!("Input file not found in {}", file_path.display());
            error!("{}", err_msg);
            writeln!(log, "{}", err_msg)?; 
            tx.send(format!("Error: Input file not found in {}", folder)).unwrap_or_default(); 
            continue; // Skip to the next folder.
        }

        // Log that the folder is being processed.
        info!("Processing folder: {}", folder);
        writeln!(log, "Processing folder: {}", folder)?; 

        let file = File::open(&file_path)?; 
        let reader = io::BufReader::new(file); 

        let mut total_sales = 0;
        let mut branch_code = String::new(); 
        let product_code = "PROD001"; 

        // Read each line from the sales file.
        for line in reader.lines() {
            let line = line?; 
            let parts: Vec<&str> = line.split(", ").collect(); 
            if parts.len() == 4 {
                branch_code = parts[0].to_string(); 
                let quantity_sold: i32 = parts[2].parse().unwrap_or(0);
                total_sales += quantity_sold; 
            }
        }

        let summary = format!("{}, {}, {}", branch_code, product_code, total_sales);
        tx.send(summary).expect("Failed to send data to main thread");
    }

    Ok("OK".to_string())
}
