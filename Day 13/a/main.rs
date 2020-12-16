use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn day_13() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut earliest_timestamp: i32 = 0;
    let mut bus_ids: Vec<i32> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if index == 0 {
            earliest_timestamp = line.parse::<i32>().unwrap();
        } else {
            for item in line.split(",") {
                if item == "x" {
                    bus_ids.push(-1);
                } else {
                    bus_ids.push(item.parse::<i32>().unwrap());
                }
                
            }
        }
    }

    let tuple = find_earliest_bus(earliest_timestamp, &bus_ids);
    println!("{}", tuple.0 * tuple.1);

    Ok(())
}

fn find_earliest_bus(departure_time: i32, buses: &Vec<i32>) -> (i32, i32) {
    let mut current_time = departure_time;

    while true {
        for id in buses {
            if *id != -1 && current_time % id == 0 {
                return (*id, current_time - departure_time);
            }
        }
        current_time += 1;
    }
    

    return (0,0);
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = day_13();

    Ok(())
}