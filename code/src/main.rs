use std::fs;
use std::path::Path;
use std::time::Instant;
mod generate_data;

/**
 * Main function.
 * 
 * This function serves as the entry point to run the entire process.
 */
fn main() {
    run();
}


/**
 * Initializes the processing of branch sales data.
 * 
 * This method sets up the process by checking if the branch folders already exist.
 * It determines whether to generate new data or use existing data, prepares the
 * output folder, and processes the input sales files to generate a weekly summary.
 */
fn run() {
    let branch_codes = setup_branch_data();

    prepare_output_folder("data/weekly_summary");

    let branch_folders = branch_codes
        .iter()
        .map(|code| format!("data/{}", code))
        .collect::<Vec<String>>();

    let start = Instant::now();
    process_sales_data(&branch_folders);
    let duration = start.elapsed();

    println!("Total time: {:?}", duration);
    println!("Phew! I am done.");
}

/**
 * Checks if branch folders exist, and determines whether to generate new data or load existing data.
 * 
 * If the branch folders are missing, it will generate new sales data.
 * If the folders are already present, it will load the existing sales data.
 *
 * Returns:
 * - A vector of branch codes to be used in further processing.
 */
fn setup_branch_data() -> Vec<String> {
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
 * Prepares the output folder by deleting it if it exists and then recreating it.
 * 
 * This method ensures that the `weekly_summary` folder is always empty at the start
 * of each run, preventing old data from being mixed with new data.
 *
 * Parameters:
 * - `output_folder`: The path to the output folder.
 */
fn prepare_output_folder(output_folder: &str) {
    // Delete 
    if Path::new(output_folder).exists() {
        match fs::remove_dir_all(output_folder) {
            Ok(_) => println!("Existing weekly_summary folder deleted."),
            Err(e) => eprintln!("Failed to delete weekly_summary folder: {}", e),
        }
    }

    // Recreate 
    if !Path::new(output_folder).exists() {
        match fs::create_dir_all(output_folder) {
            Ok(_) => println!("Weekly summary folder created successfully."),
            Err(e) => eprintln!("Failed to create weekly_summary folder: {}", e),
        }
    }
}


/**
 * Processes the sales data for the given branch folders.
 * 
 * This method reads sales files from the specified folders and generates a weekly summary.
 *
 * Parameters:
 * - `branch_folders`: A reference to a vector of strings representing the branch folder paths.
 */
fn process_sales_data(branch_folders: &[String]) {
    let result = cs3280_project1::process_input_file(branch_folders);

    match result {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("An error occurred: {:?}", e),
    }
}

