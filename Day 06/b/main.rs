use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashSet;

fn intersection_of_two(a: &HashSet<char>, b: &HashSet<char>) -> HashSet<char> {
    return a.intersection(b).copied().collect();
}

fn num_valid_passports() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut answers: Vec<HashSet<char>> = Vec::new();
    let mut group_answers: HashSet<char> = HashSet::new();
    let mut secondary_answers: HashSet<char> = HashSet::new();

    let mut is_new_section = true;
    
    for (index, line) in reader.lines().enumerate() {
        
        let line = line.unwrap();

        if line.is_empty() {
            answers.push(group_answers);
            group_answers = HashSet::new();
            is_new_section = true;
        } else {
            if is_new_section {
                is_new_section = false;
                for character in line.chars() {
                    group_answers.insert(character); 
                }
            } else {
                secondary_answers = HashSet::new();
                for character in line.chars() {
                    secondary_answers.insert(character);
                }
                group_answers = intersection_of_two(&group_answers, &secondary_answers);
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