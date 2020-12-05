use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;
use std::collections::HashSet;

fn num_valid_passports() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
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

    let mut allowed_hcl_characters: HashSet<char> = HashSet::new();
    allowed_hcl_characters.insert('0');
    allowed_hcl_characters.insert('1');
    allowed_hcl_characters.insert('2');
    allowed_hcl_characters.insert('3');
    allowed_hcl_characters.insert('4');
    allowed_hcl_characters.insert('5');
    allowed_hcl_characters.insert('6');
    allowed_hcl_characters.insert('7');
    allowed_hcl_characters.insert('8');
    allowed_hcl_characters.insert('9');
    allowed_hcl_characters.insert('a');
    allowed_hcl_characters.insert('b');
    allowed_hcl_characters.insert('c');
    allowed_hcl_characters.insert('d');
    allowed_hcl_characters.insert('e');
    allowed_hcl_characters.insert('f');

    let mut allowed_eye_colours: HashSet<String> = HashSet::new();
    allowed_eye_colours.insert("amb".to_string());
    allowed_eye_colours.insert("blu".to_string());
    allowed_eye_colours.insert("brn".to_string());
    allowed_eye_colours.insert("gry".to_string());
    allowed_eye_colours.insert("grn".to_string());
    allowed_eye_colours.insert("hzl".to_string());
    allowed_eye_colours.insert("oth".to_string());

    let mut is_valid = true;
    for passport in passports {
        is_valid = true;
        for field in &required_fields {
            if !passport.contains_key(field) {
                is_valid = false;
            } else {
                match field.as_str() {
                    "byr" => {
                        if !passport[field].parse::<i32>().is_ok() {
                            is_valid = false;
                            break;
                        }
                        let number = passport[field].parse::<i32>().unwrap();
                        if number < 1920 || number > 2002 {
                            is_valid = false;
                        }
                    }
                    "iyr" => {
                        if !passport[field].parse::<i32>().is_ok() {
                            is_valid = false;
                            break;
                        }
                        let number = passport[field].parse::<i32>().unwrap();
                        if number < 2010 || number > 2020 {
                            is_valid = false;
                        }
                    }
                    "eyr" => {
                        if !passport[field].parse::<i32>().is_ok() {
                            is_valid = false;
                            break;
                        }
                        let number = passport[field].parse::<i32>().unwrap();
                        if number < 2020 || number > 2030 {
                            is_valid = false;
                        }
                    }
                    "hgt" => {
                        let value = &passport[field];
                        if (value[value.len()-1..] == "m".to_string()) {
                            if value[value.len()-2..] != "cm".to_string() {
                                is_valid = false;
                                break;
                            }
                                
                            // Centimeters must be 3 digits
                            if !&value[..3].parse::<i32>().is_ok() {
                                is_valid = false;
                                break;
                            }
                            let centimeters = &value[..3].parse::<i32>().unwrap();
                            if centimeters < &150 || centimeters > &193 {
                                is_valid = false;
                            }
                        } else {
                            if value[value.len()-2..] != "in".to_string() {
                                is_valid = false;
                                break;
                            }
                                
                            if !&value[..2].parse::<i32>().is_ok() {
                                is_valid = false;
                                break;
                            }
                            let inches = &value[..2].parse::<i32>().unwrap();
                            if inches < &59 || inches > &76 {
                                is_valid = false;
                            }
                        }
                    }
                    "hcl" => {
                        let value = &passport[field].trim();
                        if (value.len() != 7) {
                            is_valid = false;
                            break;
                        }
                        if &value[..1] != "#" {
                            is_valid = false;
                            break;
                        }
                        for character in value[1..].chars() {
                            if !allowed_hcl_characters.contains(&character) {
                                is_valid = false;
                                break;
                            }
                        }
                    }
                    "ecl" => {
                        let value = &passport[field];

                        if !allowed_eye_colours.contains(value) {
                            is_valid = false;
                        }
                    }
                    "pid" => {
                        let value = &passport[field];

                        if value.len() != 9 {
                            is_valid = false;
                            break;
                        }
                        if !value.parse::<i32>().is_ok() {
                            is_valid = false;
                            break;
                        }
                    }
                    "cid" => {
                        println!("Found cid")
                    }
                    _ => println!("No Match Found")
                }
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