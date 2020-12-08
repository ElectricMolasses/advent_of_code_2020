use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashSet;

fn day_8() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut input: Vec<(String, i32)> = Vec::new();
    let mut acc: i32 = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let current_line: Vec<&str> = line.split(" ").collect();
        input.push((current_line[0].to_string(), current_line[1].to_string().parse::<i32>().unwrap()));
    }

    let mut index: i32 = 0;
    let mut do_continue: bool = true;

    let mut lines_seen: HashSet<i32> = HashSet::new();

    while (do_continue) {
        if (lines_seen.contains(&index)) {
            println!("{}", acc);
            return Ok(());
        }
        lines_seen.insert(index);
        if input[index as usize].0 == "acc" {
            acc += input[index as usize].1;
            index += 1;
        }
        if input[index as usize].0 == "jmp" {
            println!("{}::{}", index, input[index as usize].1);
            index += input[index as usize].1;
        }
        if input[index as usize].0 == "nop" {
            index += 1;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = day_8();

    Ok(())
}