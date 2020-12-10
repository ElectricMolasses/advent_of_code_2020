use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;
// Base joltage is 0.


fn day_10() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut list: Vec<i32> = Vec::new();
    list.push(0);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        list.push(line.parse::<i32>().unwrap());
    }
    
    list.sort();
    let mut differences: Vec<i32> = Vec::new();
    differences.push(0);
    differences.push(0);
    differences.push(0);
    differences.push(0);
    
    for i in 1..list.len() {
        differences[(list[i] - list[i - 1]) as usize] += 1;
        println!("{}::{}", i, list[i]);
    }

    println!("{}", differences[1] * (differences[3] + 1));

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = day_10();

    Ok(())
}