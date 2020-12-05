use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashSet;

fn num_valid_passports() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut rows: Vec<String> = Vec::new();
    let mut seats: HashSet<i32> = HashSet::new();

    for (index, line) in reader.lines().enumerate() {
        rows.push(line.unwrap());
    }
    let mut highest_seat = 0;
    for item in rows {
        // Remember to subtract one from your answer to convert to an index.
        let mut current_low: i32 = 0;
        let mut current_high: i32 = 127;
        for character in item[..7].chars() {
            let mut current_difference = 1 + current_high - current_low;
            if (character == 'F') {
                current_high -= current_difference / 2;
            } else {
                current_low += current_difference / 2;
            }
        }
        println!("{}:{}", current_low, current_high);
        let seat_row = current_low;

        current_low = 0;
        current_high = 7;
        for character in item[7..].chars() {
            let mut current_difference = 1 + current_high - current_low;
            if (character == 'L') {
                current_high -= current_difference / 2;
            } else {
                current_low += current_difference / 2;
            }
        }
        println!("{}:{}", current_low, current_high);
        let seat_column = current_low;

        let current_seat: i32 = seat_row * 8 + seat_column;
        seats.insert(current_seat);
        if (current_seat > highest_seat) {
            highest_seat = current_seat;
        }

    }
    println!("{}", highest_seat);

    // 128 * 8 seats
    // 1024 seats
    

    for i in 0..1023 {
        if (!seats.contains(&i)) {
            println!("{}", i);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = num_valid_passports();

    Ok(())
}