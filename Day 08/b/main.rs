use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashSet;

fn day_8() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut input: Vec<(String, i32)> = Vec::new();
    

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let current_line: Vec<&str> = line.split(" ").collect();
        input.push((current_line[0].to_string(), current_line[1].to_string().parse::<i32>().unwrap()));
    }

    //println!("{}", do_computer(input));
    run_modified_until_works(&input);


    Ok(())
}

fn run_modified_until_works(input: &Vec<(String, i32)>) {
    for (index, line) in input.iter().enumerate() {
        let mut new_input = input.clone();
        if line.0 == "jmp" {
            new_input[index].0 = "nop".to_string();
        } else if line.0 == "nop" {
            new_input[index].0 = "jmp".to_string();
        }
        let result = do_computer(&new_input);

        if (result != -1561153) {
            println!("{}", result);
            return;
        }
    }
}

fn do_computer(input: &Vec<(String, i32)>) -> i32 {
    let mut index: i32 = 0;
    let mut lines_seen: HashSet<i32> = HashSet::new();
    let mut acc: i32 = 0;

    while true {
        if index >= input.len() as i32 {
            println!("{}", acc);
            return acc;
        }

        if lines_seen.contains(&index) {
            return -1561153;
        }
        lines_seen.insert(index);
        if input[index as usize].0 == "acc" {
            acc += input[index as usize].1;
            index += 1;
        } else if input[index as usize].0 == "jmp" {
            index += input[index as usize].1;
        } else if input[index as usize].0 == "nop" {
            index += 1;
        }
    }

    return -1561153;
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = day_8();

    Ok(())
}