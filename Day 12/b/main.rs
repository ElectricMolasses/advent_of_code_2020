use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    // x, y
    let mut ship_coordinates = (0, 0);
    // The waypoint is just a vector.
    let mut waypoint_coordinates = (10, 1);

    for instruction in instructions {
        if instruction.0 == 'N' {
            waypoint_coordinates.1 += instruction.1;
        }
        if instruction.0 == 'S' {
            waypoint_coordinates.1 -= instruction.1;
        }
        if instruction.0 == 'E' {
            waypoint_coordinates.0 += instruction.1;
        }
        if instruction.0 == 'W' {
            waypoint_coordinates.0 -= instruction.1;
        }
        if instruction.0 == 'L' {
            let num_turns = instruction.1 / 90;

            for _i in 0..num_turns {
                let new_x = -waypoint_coordinates.1;
                let new_y = waypoint_coordinates.0;
                waypoint_coordinates.0 = new_x;
                waypoint_coordinates.1 = new_y;
            }
        }
        if instruction.0 == 'R' {
            let num_turns = instruction.1 / 90;

            for _i in 0..num_turns {
                let new_x = waypoint_coordinates.1;
                let new_y = -waypoint_coordinates.0;
                waypoint_coordinates.0 = new_x;
                waypoint_coordinates.1 = new_y;
            }
        }
        if instruction.0 == 'F' {
            ship_coordinates.0 += waypoint_coordinates.0 * instruction.1;
            ship_coordinates.1 += waypoint_coordinates.1 * instruction.1;
        }
    }

    return ship_coordinates;
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = day_12();

    Ok(())
}