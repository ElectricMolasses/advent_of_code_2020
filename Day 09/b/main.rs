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
    
    let rule_breaker = find_rule_breaker(&list);
    println!("{}", rule_breaker);

    let contiguous_set = find_contiguous_for(rule_breaker, &list);
    println!("{}", contiguous_set.len());
    for item in &contiguous_set {
        println!("{}", item);
    }

    println!("{}", add_largest_and_smallest(&contiguous_set));

    Ok(())
}

fn add_largest_and_smallest(list: &Vec<i64>) -> i64 {
    let mut smallest: i64 = list[0];
    let mut largest: i64 = list[0];

    for item in list {
        if smallest > *item {
            smallest = *item;
        }
        if largest < *item {
            largest = *item;
        }
    }

    return smallest + largest;
}

fn find_contiguous_for(target: i64, list: &Vec<i64>) -> Vec<i64> {
    let mut current_sum: i64 = 0;

    for i in 0..list.len() {
        current_sum = list[i];
        if current_sum == target {
            continue;
        }
        let mut pointer: usize = i+1;
        while current_sum < target {
            current_sum += list[pointer];
            if current_sum == target {
                return list[i..pointer].to_vec();
            }
            pointer += 1;
        }
    }

    return Vec::new();
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