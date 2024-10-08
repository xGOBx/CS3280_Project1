use std::fs::{self, File};
use std::io::Write;
use rand::Rng;
use chrono::NaiveDate;

pub fn generate_branch_data() -> Vec<&'static str> {
    let branch_codes = vec![
        "ALBNM", "TXHOU", "NYNYC", "LABUR", "CHCHI", "MIAMI", "DENCO", "PHOAZ", "SATX", "LASNV",
        "SEAWA", "DETPR", "ATLGA", "BOSMA", "BALMD", "MINMN", "CHARNC", "ININD", "OKLOK", "LOUKY",
        "PORTOR", "TUSAL", "MIPER", "NORVA", "ORLFL", "TBFLA", "AKRCO", "RICHVA", "SPOWA", "OMAHA",
        "FTWTC", "NASTN", "GRMI", "PITPA", "COLCO", "ATLOK", "FRSCA", "JACFL", "SLCUT", "BUFNY"
    ];

    let product_code = "PROD001";
    let start_date = NaiveDate::from_ymd(2023, 1, 1);

    for branch_code in branch_codes.iter() {
        let branch_folder = format!("data/{}", branch_code);
        fs::create_dir_all(&branch_folder).expect("Failed to create branch folder");

        let file_path = format!("{}/branch_weekly_sales.txt", branch_folder);
        let mut file = File::create(file_path).expect("Failed to create sales file");

        for day_offset in 0..7 {
            let date_sold = start_date + chrono::Duration::days(day_offset);
            let quantity_sold = rand::thread_rng().gen_range(1..50); 
            let line = format!("{}, {}, {}, {}\n", branch_code, product_code, quantity_sold, date_sold);
            file.write_all(line.as_bytes()).expect("Failed to write data");
        }
    }

    println!("Branch folders and sales data generated successfully.");
    
    return branch_codes ;
 
}
