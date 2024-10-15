use std::fs::{self, File}; 
use std::io::{self, BufRead, Write}; 
use std::path::Path; 
/**
 * Processes input sales files from the provided folder paths and generates a weekly summary.
 *
 * This function iterates over a list of folder paths, reads the `branch_weekly_sales.txt` file from each folder,
 * and calculates the total sales for a specific product (`PROD001`). The sales summary for each branch is
 * written to a summary file (`weekly_sales_summary.txt`).
 *
 * Parameters:
 * - `folders`: A list of folder paths where each folder contains a `branch_weekly_sales.txt` file.
 *
 * Returns:
 * - `Ok(String)` if all files are processed successfully, or an `io::Error` if any file-related operation fails.
 */
pub fn process_input_file(folders: &[String]) -> Result<String, io::Error> {
    for folder in folders {
        let file_path = format!("{}/branch_weekly_sales.txt", folder);
        
        println!("Looking for file at: {}", file_path); 
        
        // Check if the file exists. If not, return a NotFound error.
        if !Path::new(&file_path).exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("Input file not found at path: {}", file_path)));
        }

        let file = File::open(&file_path)?; 
        let reader = io::BufReader::new(file); // Wrap the file in a buffered reader for efficient reading.

        let mut total_sales = 0; 
        let mut branch_code = String::new(); 
        let product_code = "PROD001"; 

        // Read each line in the sales file.
        for line in reader.lines() {
            let line = line?; 
            let parts: Vec<&str> = line.split(", ").collect(); 
            
            // Ensure the line has 4 parts (branch_code, product_code, quantity_sold, date_sold).
            if parts.len() == 4 {
                branch_code = parts[0].to_string();
                let quantity_sold: i32 = parts[2].parse().unwrap_or(0); 
                total_sales += quantity_sold; 
            }
        }

        let summary_line = format!("{}, {}, {}", branch_code, product_code, total_sales);
        write_to_summary_file(&summary_line)?; 
    }

    Ok("OK".to_string())
}

/**
 * Writes a sales summary line to the weekly summary file.
 *
 * This function appends the provided data to the `weekly_sales_summary.txt` file located in the `data/weekly_summary` directory.
 * If the file doesn't exist, it will be created.
 *
 * Parameters:
 * - `data`: The summary data string to be written to the summary file.
 *
 * Returns:
 * - `Ok(())` if the write operation is successful, or an `io::Error` if it fails.
 */
pub fn write_to_summary_file(data: &str) -> Result<(), io::Error> {
    let summary_file_path = "data/weekly_summary/weekly_sales_summary.txt";
    
    let mut file = File::options()
        .create(true)
        .append(true)
        .open(summary_file_path)?;

    writeln!(file, "{}", data)?;
    Ok(())
}
