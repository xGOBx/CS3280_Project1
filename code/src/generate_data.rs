use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use log::{info, error};
use rand::Rng;
use chrono::NaiveDate;

/**
 * Generates weekly sales data for a list of branch codes.
 *
 * This function checks if a sales file (`branch_weekly_sales.txt`) already exists
 * within each branch folder. If the file does not exist, it generates new sales data.
 *
 * Returns:
 * - A vector containing the branch codes for which sales data was generated.
 *
 * Errors:
 * - If a directory or file creation fails, an error message is printed, 
 *   and that branch's sales data will be skipped.
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

        if !Path::new(&branch_folder).exists() {
            if let Err(e) = fs::create_dir_all(&branch_folder) {
                eprintln!("Failed to create folder for {}: {}", branch_code, e);
                continue;
            }
        }

        let file_path = format!("{}/branch_weekly_sales.txt", branch_folder);
        if Path::new(&file_path).exists() {
            info!("Sales data already exists for {}. Skipping...", branch_code);
            continue;
        }

        let mut file = match File::create(&file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to create sales file for {}: {}", branch_code, e);
                continue;
            }
        };

        for day_offset in 0..7 {
            let date_sold = start_date + chrono::Duration::days(day_offset);
            let quantity_sold: i32 = rand::thread_rng().gen_range(1..50);
            let line = format!("{}, {}, {}, {}\n", branch_code, product_code, quantity_sold, date_sold);

            if let Err(e) = file.write_all(line.as_bytes()) {
                eprintln!("Failed to write sales data for {}: {}", branch_code, e);
                break;
            }
        }
        info!("Generated sales data for {}", branch_code);
    }

    println!("Branch folders and sales data generation process completed.");
    branch_codes
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
                Ok(data) => {
                    info!("Loaded existing sales data for {}", branch_code);
                    existing_data.push(data);
                },
                Err(e) => error!("Failed to read sales data for {}: {}", branch_code, e),
            }
        }
    }

    existing_data
}
