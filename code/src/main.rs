use std::fs; 
use std::path::Path; 
use std::time::Instant; 
mod lib; 
mod generate_data; 

/**
 * Main function to generate branch sales data and process it.
 *
 * This function checks if branch folders and sales data exist; if not, it generates them
 * using the `generate_data` module. It then processes the generated data files and saves
 * a weekly sales summary.
 *
 * - If the "weekly_summary" folder does not exist, it creates it.
 * - Measures the total time taken to complete the processing.
 * - Displays the result of the processing (success or failure) and the elapsed time.
 */
fn main() {
    // Check if the first branch folder exists. If not, generate all branch folders and sales data.
    let first_branch_folder = "data/ALBNM"; 
    if !Path::new(first_branch_folder).exists() {
        println!("Branch folders not found. Generating branch folders and sales data...");
    }

    // Generate sales data for all branches.
    let branch_codes = generate_data::generate_branch_data();

    // Check if the weekly summary folder exists; create it if not.
    let output_folder = "data/weekly_summary";
    if !Path::new(output_folder).exists() {
        fs::create_dir(output_folder).expect("Failed to create weekly_summary folder");
    }

    // Start a timer to measure the total time for processing.
    let start = Instant::now();

    let branch_folders = branch_codes
        .iter()
        .map(|code| format!("data/{}", code))
        .collect::<Vec<String>>();
    
    // Process the input files for all branches.
    let result = lib::process_input_file(&branch_folders);

    match result {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("An error occurred: {:?}", e),
    }

    let duration = start.elapsed();
    println!("Total time: {:?}", duration);

    println!("Phew! I am done.");
}
