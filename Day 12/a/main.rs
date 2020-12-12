use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;

// . Floor
// L Empty Seat
// # Occupied Seat

fn day_12() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut instructions: Vec<(char, i32)> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let character = line.chars().next().unwrap();
        let number = line.to_string()[1..].parse::<i32>().unwrap();

        instructions.push((character, number));
    }

    // Able to move in cardinal directions, as well as turn left num degrees, turn right num degrees, and move foreard.
    let result = move_ship(&instructions);
    println!("{}::{}", result.0, result.1);

    Ok(())
}

fn move_ship(instructions: &Vec<(char, i32)>) -> (i32, i32) {

    let mut facings: HashMap<i32, char> = HashMap::new();
    facings.insert(0, 'N');
    facings.insert(90, 'E');
    facings.insert(180, 'S');
    facings.insert(270, 'W');

    let mut ship_facing: i32 = 90;

    // x, y
    let mut coordinates = (0, 0);

    for instruction in instructions {
        if instruction.0 == 'N' {
            coordinates.0 += instruction.1;
        }
        if instruction.0 == 'S' {
            coordinates.0 -= instruction.1;
        }
        if instruction.0 == 'E' {
            coordinates.1 += instruction.1;
        }
        if instruction.0 == 'W' {
            coordinates.1 -= instruction.1;
        }
        if instruction.0 == 'L' {
            ship_facing -= instruction.1;
            if ship_facing < 0 {
                ship_facing = 360 + ship_facing;
            }
        }
        if instruction.0 == 'R' {
            ship_facing += instruction.1;
            if ship_facing > 270 {
                ship_facing = ship_facing - 360;
            }
        }
        if instruction.0 == 'F' {
            if ship_facing == 0 {
                coordinates.0 += instruction.1;
            }
            if ship_facing == 90 {
                coordinates.1 += instruction.1;
            }
            if ship_facing == 180 {
                coordinates.0 -= instruction.1;
            }
            if ship_facing == 270 {
                coordinates.1 -= instruction.1;
            }
        }
    }

    return coordinates;
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = day_12();

    Ok(())
}