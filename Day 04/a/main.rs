use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;

use std::collections::HashMap;

fn num_valid_passports() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut passports: Vec<HashMap<String, String>> = Vec::new();
    let mut current_passport: HashMap<String, String> = HashMap::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        
        if (line.is_empty()) {
            passports.push(current_passport.clone());
            current_passport = HashMap::new();
        } else {
            for entry in line.split(" ") {
                let a: Vec<&str> = entry.split(":").collect();
                current_passport.insert(String::from(a[0]), String::from(a[1]));
            }
        }
    }
    passports.push(current_passport);
    // We now have all the passports in a hashmap
    println!("{}", passports.len());

    let mut total_valid: i32 = 0;
    let required_fields: [String; 7] = [
        "byr".to_string(),
        "iyr".to_string(),
        "eyr".to_string(),
        "hgt".to_string(),
        "hcl".to_string(),
        "ecl".to_string(),
        "pid".to_string(),
        //"cid".to_string()  This field is optional.
    ];

    let mut is_valid = true;
    for passport in passports {
        is_valid = true;
        for field in &required_fields {
            println!("{}", field);
            if !passport.contains_key(field) {
                is_valid = false;
            }
        }
        if is_valid {
            total_valid += 1;
        }
        
    }

    println!("{}", total_valid);


    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = num_valid_passports();

    Ok(())
}