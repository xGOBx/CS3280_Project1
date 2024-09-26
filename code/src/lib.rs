use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

pub fn process_input_file(folders: &[String]) -> Result<String, io::Error> {
    for folder in folders {
        let file_path = format!("{}/branch_weekly_sales.txt", folder);
        
        println!("Looking for file at: {}", file_path);
        
        if !Path::new(&file_path).exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("Input file not found at path: {}", file_path)));
        }

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

        let summary_line = format!("{}, {}, {}", branch_code, product_code, total_sales);
        write_to_summary_file(&summary_line)?;
    }

    Ok("OK".to_string())
}

pub fn write_to_summary_file(data: &str) -> Result<(), io::Error> {
    let summary_file_path = "data/weekly_summary/weekly_sales_summary.txt";
    let mut file = File::options()
        .create(true)
        .append(true)
        .open(summary_file_path)?;

    writeln!(file, "{}", data)?;
    Ok(())
}
