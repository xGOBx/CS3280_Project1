use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;
use rand::Rng;
use chrono::NaiveDate;

/**
 * Generates weekly sales data for a list of branch codes.
 *
 * This function creates a directory for each branch inside the "data" folder and generates
 * a sales file (`branch_weekly_sales.txt`) for each branch. The file contains sales data for
 * one product (`PROD001`) over the course of a week (7 days), with each day's sales
 * recorded as a line of CSV data (branch_code, product_code, quantity_sold, date_sold).
 *
 * Returns:
 * - A vector of branch codes for which the sales data was generated.
 */
pub fn generate_branch_data() -> Vec<&'static str> {
    let branch_codes = vec![
        "ALBNM", "TXHOU", "NYNYC", "LABUR", "CHCHI", "MIAMI", "DENCO", "PHOAZ", "SATX", "LASNV",
        "SEAWA", "DETPR", "ATLGA", "BOSMA", "BALMD", "MINMN", "CHARNC", "ININD", "OKLOK", "LOUKY",
        "PORTOR", "TUSAL", "MIPER", "NORVA", "ORLFL", "TBFLA", "AKRCO", "RICHVA", "SPOWA", "OMAHA",
        "FTWTC", "NASTN", "GRMI", "PITPA", "COLCO", "ATLOK", "FRSCA", "JACFL", "SLCUT", "BUFNY"
    ];

    let product_code = "PROD001"; 
    let start_date = NaiveDate::from_ymd_opt(2023, 1, 1).expect("Invalid date");

    for branch_code in branch_codes.iter() {
        let branch_folder = format!("data/{}", branch_code);
        fs::create_dir_all(&branch_folder).expect("Failed to create branch folder");

        let file_path = format!("{}/branch_weekly_sales.txt", branch_folder);
        let mut file = File::create(file_path).expect("Failed to create sales file");

        // Generate sales data for 7 consecutive days (1 week).
        for day_offset in 0..7 {
            let date_sold = start_date + chrono::Duration::days(day_offset);
            let quantity_sold = rand::thread_rng().gen_range(1..50); 
            let line = format!("{}, {}, {}, {}\n", branch_code, product_code, quantity_sold, date_sold);
            file.write_all(line.as_bytes()).expect("Failed to write data");
        }
    }

    println!("Branch folders and sales data generated successfully.");
    
    return branch_codes;
}

/**
 * Loads existing sales data for branches that have generated files.
 *
 * This function reads existing `branch_weekly_sales.txt` files from the "data" directory
 * and returns a vector of strings containing the sales data.
 *
 * Returns:
 * - A vector of strings, where each string is the contents of a sales file for a branch.
 *
 * Errors:
 * - If a file cannot be read, an error message is printed, and that file's data will be skipped.
 */
pub fn load_existing_branch_data() -> Vec<String> {
    let branch_codes = vec![
        "ALBNM", "TXHOU", "NYNYC", "LABUR", "CHCHI", "MIAMI", "DENCO", "PHOAZ", "SATX", "LASNV",
        "SEAWA", "DETPR", "ATLGA", "BOSMA", "BALMD", "MINMN", "CHARNC", "ININD", "OKLOK", "LOUKY",
        "PORTOR", "TUSAL", "MIPER", "NORVA", "ORLFL", "TBFLA", "AKRCO", "RICHVA", "SPOWA", "OMAHA",
        "FTWTC", "NASTN", "GRMI", "PITPA", "COLCO", "ATLOK", "FRSCA", "JACFL", "SLCUT", "BUFNY"
    ];

    let mut existing_data = Vec::new();

    for branch_code in branch_codes.iter() {
        let file_path = format!("data/{}/branch_weekly_sales.txt", branch_code);
        if Path::new(&file_path).exists() {
            match fs::read_to_string(&file_path) {
                Ok(data) => existing_data.push(data),
                Err(e) => eprintln!("Failed to read sales data for {}: {}", branch_code, e),
            }
        }
    }

    existing_data
}
