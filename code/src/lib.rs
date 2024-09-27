use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use log::{error, info};

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

        if !file_path.exists() {
            let err_msg = format!("Input file not found in {}", file_path.display());
            error!("{}", err_msg);
            writeln!(log, "{}", err_msg)?;
            tx.send(format!("Error: Input file not found in {}", folder)).unwrap_or_default();
            continue; 
        }

        info!("Processing folder: {}", folder);
        writeln!(log, "Processing folder: {}", folder)?;

        let file = File::open(&file_path)?; 
        let reader = io::BufReader::new(file);

        let mut total_sales = 0;
        let mut branch_code = String::new();
        let product_code = "PROD001"; 

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
