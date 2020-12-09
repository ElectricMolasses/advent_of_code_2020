use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn day_9() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    
    let mut list: Vec<i64> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        list.push(line.parse::<i64>().unwrap());
    }
    
    println!("{}", find_rule_breaker(&list));

    Ok(())
}

fn find_rule_breaker(list: &Vec<i64>) -> i64 {
    let mut last_25: Vec<i64> = Vec::new();

    for (index, item) in list.iter().enumerate() {
        if index < 25 {
            last_25.push(*item);
            continue;
        }
        if !validate(&last_25, *item) {
            return *item;
        }
        last_25 = last_25.split_off(1);
        last_25.push(*item);
    }
    
    return 1;
}

fn validate(last_25: &Vec<i64>, current: i64) -> bool {
    if last_25.len() != 25 {
        println!("SCREM!");
    }
    for i in 0..last_25.len() {
        for j in 0+i..last_25.len() {
            if (last_25[i] + last_25[j] == current) {
                return true;
            }
        }
    }

    return false;
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = day_9();

    Ok(())
}