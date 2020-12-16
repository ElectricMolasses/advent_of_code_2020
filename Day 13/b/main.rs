use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn day_13() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut _earliest_timestamp: i64 = 0;
    let mut bus_ids: Vec<i64> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if index == 0 {
            _earliest_timestamp = line.parse::<i64>().unwrap();
        } else {
            for item in line.split(",") {
                if item == "x" {
                    bus_ids.push(-1);
                } else {
                    bus_ids.push(item.parse::<i64>().unwrap());
                }
                
            }
        }
    }

    println!("{}", find_magic_time(&bus_ids));

    Ok(())
}

fn find_magic_time(bus_ids: &Vec<i64>) -> i64 {
    
    let mut timestamp: i64 = 1;
    let mut combined_offset: i64 = 1;

    for index in 0..bus_ids.len() {
        if bus_ids[index] != -1 {
            while (timestamp + index as i64) % bus_ids[index] != 0 {
                timestamp += combined_offset;
            }
            combined_offset *= bus_ids[index];
        }
    }

    return timestamp;
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = day_13();

    Ok(())
}