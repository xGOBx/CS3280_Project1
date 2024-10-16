use std::fs::{self, File}; 
use std::io::Write; 
use rand::Rng; 
use chrono::NaiveDate; 

/**
 * Generates weekly sales data for a list of branch codes.
 *
 * This function creates a folder for each branch inside a "data" directory
 * and generates a sales file (`branch_weekly_sales.txt`) that records
 * random sales data for one product over the course of a week. Each day's 
 * data includes the branch code, product code, quantity sold, and date.
 *
 * Returns:
 * - A vector containing the branch codes for which sales data was generated.
 *
 * Errors:
 * - If a directory or file creation fails, an error message is printed, 
 *   and that branch's sales data will be skipped.
 */
pub fn generate_branch_data() -> Vec<&'static str> {
    // List of branch codes representing various locations.
    let branch_codes = vec![
        "ALBNM", "TXHOU", "NYNYC", "LABUR", "CHCHI", "MIAMI", "DENCO", "PHOAZ", "SATX", "LASNV",
        "SEAWA", "DETPR", "ATLGA", "BOSMA", "BALMD", "MINMN", "CHARNC", "ININD", "OKLOK", "LOUKY",
        "PORTOR", "TUSAL", "MIPER", "NORVA", "ORLFL", "TBFLA", "AKRCO", "RICHVA", "SPOWA", "OMAHA",
        "FTWTC", "NASTN", "GRMI", "PITPA", "COLCO", "ATLOK", "FRSCA", "JACFL", "SLCUT", "BUFNY"
    ];

    let product_code = "PROD001"; // Product code for the item being sold.
    let start_date = NaiveDate::from_ymd_opt(2023, 1, 1).expect("Invalid date");

    // Iterate over each branch and generate sales data.
    for branch_code in branch_codes.iter() {
        // Create a directory for the branch within the "data" folder.
        let branch_folder = format!("data/{}", branch_code);
        if let Err(e) = fs::create_dir_all(&branch_folder) {
            // Log an error message if directory creation fails.
            eprintln!("Failed to create folder for {}: {}", branch_code, e);
            continue;
        }

        // File path where the sales data will be stored.
        let file_path = format!("{}/branch_weekly_sales.txt", branch_folder);
        // Attempt to create the sales data file.
        let mut file = match File::create(&file_path) {
            Ok(f) => f,
            Err(e) => {
                // Log an error message if file creation fails.
                eprintln!("Failed to create sales file for {}: {}", branch_code, e);
                continue;
            }
        };

        // Generate sales data for 7 consecutive days (one week).
        for day_offset in 0..7 {
            
            let date_sold = start_date + chrono::Duration::days(day_offset);
            // Generate a random quantity sold between 1 and 49.
            let quantity_sold: i32 = rand::thread_rng().gen_range(1..50);
            // Format the sales data as a string.
            let line = format!("{}, {}, {}, {}\n", branch_code, product_code, quantity_sold, date_sold);

            if let Err(e) = file.write_all(line.as_bytes()) {
                eprintln!("Failed to write sales data for {}: {}", branch_code, e);
                break;
            }
        }

        println!("Generated sales data for {}", branch_code); 
    }

    println!("Branch folders and sales data generated successfully.");
    
    branch_codes
}
