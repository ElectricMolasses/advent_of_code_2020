use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashSet;

fn num_valid_passports() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut rows: Vec<String> = Vec::new();
    let mut seats: HashSet<i32> = HashSet::new();

    let mut answers: Vec<HashSet<char>> = Vec::new();
    let mut group_answers: HashSet<char> = HashSet::new();

    for (index, line) in reader.lines().enumerate() {
        
        let line = line.unwrap();

        if line.is_empty() {
            answers.push(group_answers);
            group_answers = HashSet::new();
        } else {
            for character in line.chars() {
                group_answers.insert(character);
            }
        }
    }
    answers.push(group_answers);

    let mut total = 0;

    for entry in answers {
        total += entry.len();
    }

    println!("{}", total);
    

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = num_valid_passports();

    Ok(())
}