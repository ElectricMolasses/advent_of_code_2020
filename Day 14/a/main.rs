use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;

fn day_14() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut instructions: Vec<(String, String)> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let mut split_line = line.split(" = ");
        instructions.push((split_line.next().unwrap().to_string(), split_line.next().unwrap().to_string()));
    }

    let result = write_with_masks(instructions);
    let mut answer: u64 = 0;

    for (k, v) in result {
        answer += v;
    }

    println!("{}", answer);

    Ok(())
}

fn write_with_masks(instructions: Vec<(String, String)>) -> HashMap<u64, u64> {
    let mut result: HashMap<u64, u64> = HashMap::new();
    let mut mask: String = "".to_string();
    
    for instruction in instructions {
        if instruction.0.contains("mem") {
            let mem_address = instruction.0[4..instruction.0.len() - 1].to_string();
            let write_value = mask_as_binary_string(instruction.1.to_string(), mask.clone());
            println!("{}", write_value);
            result.insert(mem_address.parse::<u64>().unwrap(),
                            u64::from_str_radix(&write_value.chars().rev().collect::<String>(), 2).unwrap() as u64);
        }
        if instruction.0.contains("mask") {
            mask = instruction.1;
            println!("{}", mask);
        }
    }

    return result;
}



fn mask_as_binary_string(number: String, mask: String) -> String {
    let mut binary = convert_to_binary_string(number.parse::<u64>().unwrap());
    let mut ordered_mask: Vec<char> = mask.clone().chars().rev().collect();

    while binary.len() < ordered_mask.len() {
        binary.push_str(&"0".to_string());
    }

    println!("{}", binary);

    let mut masked_string: String = "".to_string();

    for (index, digit) in binary.chars().enumerate() {
        if ordered_mask[index] != 'X' {
            masked_string.push_str(&ordered_mask[index].to_string());
        } else {
            masked_string.push_str(&digit.to_string());
        }
        println!("{}", masked_string);
    }

    println!("{}", masked_string);
    return masked_string;
}

fn convert_to_binary_string(a: u64) -> String {
    let mut output: String = String::new();
    let mut current: u64 = a;

    if a == 0 {
        return "0".to_string();
    }

    while current > 0 {
        output.push_str(&(current % 2).to_string());
        current /= 2;
    }
    return output.chars().collect();
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = day_14();

    Ok(())
}