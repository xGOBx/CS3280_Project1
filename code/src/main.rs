use std::fs;
use std::path::Path;
use std::thread;
use std::sync::mpsc;
use std::time::Instant;
use std::io::Write; 
mod lib;
mod generate_data;

fn write_to_summary_file(data: &str) -> Result<(), std::io::Error> {
    let summary_file_path = "data/weekly_summary/weekly_sales_summary.txt";
    let mut file = std::fs::File::options()
        .create(true)
        .append(true)
        .open(summary_file_path)?;

    writeln!(file, "{}", data)?;
    Ok(())
}

fn main() {
   
    env_logger::init(); 

    let first_branch_folder = "data/ALBNM";
    if !Path::new(first_branch_folder).exists() {
        println!("Branch folders not found. Generating branch folders and sales data...");
    }

    let branch_codes = generate_data::generate_branch_data();

    let output_folder = "data/weekly_summary";
    if !Path::new(output_folder).exists() {
        fs::create_dir(output_folder).expect("Failed to create weekly_summary folder");
    }

    let (tx, rx) = mpsc::channel();

    let branch_groups: Vec<Vec<String>> = branch_codes
        .chunks(10)
        .map(|chunk| chunk.iter().map(|&s| s.to_string()).collect())
        .collect();

    let start = Instant::now();

    let mut handles = vec![];

    for group in branch_groups {
        let tx = tx.clone(); 

        let handle = thread::spawn(move || {
            if let Err(e) = lib::process_input_file(&group, tx) {
                eprintln!("Thread failed to process input: {:?}", e); 
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Failed to join thread: {:?}", e); 
        }
    }

    drop(tx);

    while let Ok(received) = rx.recv() {
        println!("Received: {}", received);
        write_to_summary_file(&received).expect("Failed to write to summary file");
    }

    
    let duration = start.elapsed();
    println!("Total time for processing: {:?}", duration);

    println!("Phew! I am done.");
}
