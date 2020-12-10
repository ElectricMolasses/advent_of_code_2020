use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;
// Base joltage is 0.

fn day_10() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut list: Vec<u64> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        list.push(line.parse::<u64>().unwrap());
    }
    
    list.sort();
    list.push(list[list.len() - 1] + 3);
    let mut known: HashMap<u64, u64> = HashMap::new();

    println!("{}", recurse_the_universe(0, list[list.len() - 1], &list, &mut known));

    Ok(())
}

fn recurse_the_universe(current: u64, the_end: u64, the_list: &Vec<u64>, known: &mut HashMap<u64, u64>) -> u64 {
    let mut total: u64 = 0;

    if current == the_end {
        return 1;
    }

    for (index, item) in the_list.iter().enumerate() {
        let mut sub_total: u64 = 0;
        if item - current <= 3 && item - current > 0 {
            if (known.contains_key(item)) {
                total += known[item];
            } else {
                sub_total += recurse_the_universe(*item, the_end, &the_list[index..].to_vec(), known);
                known.insert(*item, sub_total);
                total += sub_total;
            }
        }
    }

    return total;
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = day_10();

    Ok(())
}