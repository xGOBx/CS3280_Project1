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



}