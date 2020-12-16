use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::{HashMap, HashSet};

fn day_15() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut starting_numbers: Vec<i32> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        for item in line.split(",") {
            starting_numbers.push(item.parse::<i32>().unwrap());
        }
    }

   let answer = count_until(starting_numbers, 30000000);
   println!("{}", answer);

    Ok(())
}

fn count_until(starting_numbers: Vec<i32>, until: usize) -> i32 {
    let mut found_numbers: HashMap<i32, (i32, i32)> = HashMap::new();
    let mut is_duplicate: HashSet<i32> = HashSet::new();
    let mut current_number: i32 = -1;

    let mut index: usize = 0;
    
    for number in starting_numbers {
        if found_numbers.contains_key(&number) {
            let holder = found_numbers[&number];
            found_numbers.remove_entry(&number);
            found_numbers.insert(number, (index as i32, holder.0));
            is_duplicate.insert(number);
        } else {
            found_numbers.insert(number, (index as i32, -1));
        }
        index += 1;
        current_number = number;
    }

    while index < until {
        if found_numbers.contains_key(&current_number) {

            if is_duplicate.contains(&current_number) {
                current_number = found_numbers[&current_number].0 - found_numbers[&current_number].1;
                if found_numbers.contains_key(&current_number) {
                    let holder = found_numbers[&current_number];
                    found_numbers.remove_entry(&current_number);
                    found_numbers.insert(current_number, (index as i32, holder.0));
                    if !is_duplicate.contains(&current_number) {
                        is_duplicate.insert(current_number);
                    }
                } else {
                    found_numbers.insert(current_number, (index as i32, -1));
                }
            } 
            else
            {
                current_number = 0;
                if is_duplicate.contains(&current_number) {
                    let holder = found_numbers[&current_number];
                    found_numbers.remove_entry(&current_number);
                    found_numbers.insert(current_number, (index as i32, holder.0));
                } else {
                    if found_numbers.contains_key(&current_number) {
                        is_duplicate.insert(current_number);
                        let holder = found_numbers[&current_number];
                        found_numbers.remove_entry(&current_number);
                        found_numbers.insert(current_number, (index as i32, holder.0));
                    }
                }
            }
        } else {
            current_number = 0;
            found_numbers.insert(current_number, (index as i32, -1));
        }

        index += 1;
    }

    return current_number;
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = day_15();

    Ok(())
}