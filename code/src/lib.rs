use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use log::error;

/**
 * Processes input files located in specified folders, reads sales data, 
 * and writes summaries of total sales directly to the summary file.
 *
 * This function opens each folder specified in the `folders` array, looks for
 * a file named `branch_weekly_sales.txt` inside a "data" directory, and calculates
 * the total sales of a specific product ("PROD001") for that branch.
 *
 * The result is written directly to the summary file using a mutex to ensure
 * thread-safe writing.
 *
 * Parameters:
 * - `folders`: A slice of folder names (strings) where the sales data files are located.
 * - `summary_file_mutex`: An `Arc<Mutex<File>>` to safely write the processed summary across multiple threads.
 *
 * Returns:
 * - A `Result` that is `Ok(String)` if the function completes successfully, or an `io::Error` if any I/O operation fails.
 *
 * Errors:
 * - If the function encounters missing files, file access errors, or parsing issues, 
 *   it logs the errors and continues processing the remaining folders.
 * - Errors related to file creation or writing to the log file will stop execution and return an `io::Error`.
 */
pub fn process_input_file(
    folders: &[String],
    summary_file_mutex: Arc<Mutex<File>>
) -> Result<String, io::Error> {
    for folder in folders {
        let mut file_path = PathBuf::from("data"); 
        file_path.push(folder); 
        file_path.push("branch_weekly_sales.txt");

        if !file_path.exists() {
            let err_msg = format!("Input file not found in {}", file_path.display());
            error!("{}", err_msg);
            continue; 
        }

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

        // Directly to the summary file.
        let mut summary_file = summary_file_mutex.lock().unwrap();
        writeln!(summary_file, "{}", summary)?;
    }

    Ok("OK".to_string())
}
