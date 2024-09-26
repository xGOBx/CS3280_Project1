use std::fs;
use std::path::Path;
use std::time::Instant;
mod lib;
mod generate_data;

fn main() {
    let first_branch_folder = "data/ALBNM"; 
    if !Path::new(first_branch_folder).exists() {
        println!("Branch folders not found. Generating branch folders and sales data...");
    }

    let branch_codes = generate_data::generate_branch_data();

    let output_folder = "data/weekly_summary";
    if !Path::new(output_folder).exists() {
        fs::create_dir(output_folder).expect("Failed to create weekly_summary folder");
    }

    let start = Instant::now();

    let branch_folders = branch_codes
        .iter()
        .map(|code| format!("data/{}", code))
        .collect::<Vec<String>>();
    
    let result = lib::process_input_file(&branch_folders);

    match result {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("An error occurred: {:?}", e),
    }

    let duration = start.elapsed();
    println!("Total time: {:?}", duration);

    println!("Phew! I am done.");
}
