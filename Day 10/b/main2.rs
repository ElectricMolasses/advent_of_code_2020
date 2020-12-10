use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;
// Base joltage is 0.


fn day_10() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut list: Vec<i64> = Vec::new();
    list.push(0);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        list.push(line.parse::<i64>().unwrap());
    }

    let mut splits: HashMap<i64, i64> = HashMap::new();
    
    list.sort();
    list.push(list[list.len() - 1] + 3);
    // Every numbers path has to calculate in both directions. ??????????
    let mut total: i64 = 1;
    for i in 0..list.len() {
        let mut paths: i64 = 0;
        for j in i+1..list.len() {
            if list[j] - list[i] <= 3 {
                paths += 1;
            } else {
                break;
            }
        }
        println!("{}::{}", i, paths);
        splits.insert(i as i64, paths);
        if paths > 1 {
            total *= paths - 1;
        }
    }

    println!("PRINT MAP");
    for i in 0..splits.len() {
        println!("{}::{}", i, splits[&(i as i64)]);
    }
    


    Ok(())
}

fn factorial(mut x: u128) -> u128 {
    let mut total: u128 = x;
    x -= 1;

    while (x > 0) {
        total *= x;
        x -= 1;
    }

    return total;
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = day_10();

    Ok(())
}