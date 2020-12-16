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

    for (_k, v) in result {
        answer += v;
    }

    println!("{}", answer);

    Ok(())
}

fn write_with_masks(instructions: Vec<(String, String)>) -> HashMap<u64, u64> {
    let mut result: HashMap<u64, u64> = HashMap::new();
    let mut mask: String = "".to_string();
    let mut mem_addresses: Vec<String> = Vec::new();
    
    for instruction in instructions {
        if instruction.0.contains("mem") {
            let write_value = instruction.1.to_string().parse::<u64>().unwrap();
            let mem_address = instruction.0[4..instruction.0.len() - 1].to_string();

            mem_addresses = mask_all_mem_addresses(mask.clone(), 
                                        size_up(convert_to_binary_string(mem_address.parse::<u64>().unwrap()), mask.len()));

            for entry in &mem_addresses {
                let key = u64::from_str_radix(&entry.chars().collect::<String>(), 2).unwrap() as u64;
                println!("{}:{}", entry, key);
                if result.contains_key(&key) {
                    result.remove(&key);
                    result.insert(key, write_value);
                } else {
                    result.insert(key, write_value);
                }
                
            }
            
        }
        if instruction.0.contains("mask") {
            mask = instruction.1;
        }
    }

    return result;
}

fn size_up(address: String, target_size: usize) -> String {
    let mut new_address: String = address.clone().chars().rev().collect();

    while new_address.len() < target_size {
        new_address.push_str("0");
    }

    return new_address.chars().rev().collect();
}

fn mask_all_mem_addresses(mask: String, address: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut partials: Vec<String> = Vec::new();

    let mut last_index: usize = 0;
    let mut past_first: bool = false;

    for (index, character) in mask.chars().enumerate() {
        if character == 'X' {
            let mut new_partials: Vec<String> = Vec::new();
            
            if !past_first {
                let mut string1: String = "".to_string();

                for (inner_index, character) in mask[0..index].chars().enumerate() {
                    if character == '0' {
                        string1.push_str(&address[inner_index..inner_index+1]);
                    } else {
                        string1.push_str(&"1");
                    }
                }
                let mut string2: String = string1.clone();
                
                string1.push_str(&"0".to_string());
                string2.push_str(&"1".to_string());

                partials.push(string1);
                partials.push(string2);
                past_first = true;
                last_index = index;
            } else {
                for partial in partials.clone() {
                    let mut new_string: String = partial;

                    for (inner_index, character) in mask[last_index+1..index].chars().enumerate() {
                        if character == '0' {
                            new_string.push_str(&address[last_index+1+inner_index..last_index+1+inner_index+1]);
                        } else {
                            new_string.push_str(&"1");
                        }
                    }
                    
                    let mut new_string1: String = new_string.clone();
                    let mut new_string2: String = new_string.clone();
                    new_string1.push_str(&"0".to_string());
                    new_string2.push_str(&"1".to_string());
                    new_partials.push(new_string1.clone());
                    new_partials.push(new_string2.clone());
                }
                last_index = index;
                partials = new_partials.clone();
            }
        }
    }

    return partials;
}

fn mask_as_binary_string(number: String, mask: String) -> String {
    let mut binary = convert_to_binary_string(number.parse::<u64>().unwrap());
    let mut ordered_mask: Vec<char> = mask.clone().chars().rev().collect();

    while binary.len() < ordered_mask.len() {
        binary.push_str(&"0".to_string());
    }

    let mut masked_string: String = "".to_string();

    for (index, digit) in binary.chars().enumerate() {
        if ordered_mask[index] != 'X' {
            masked_string.push_str(&ordered_mask[index].to_string());
        } else {
            masked_string.push_str(&digit.to_string());
        }
    }

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
    return output.chars().rev().collect();
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = day_14();

    Ok(())
}