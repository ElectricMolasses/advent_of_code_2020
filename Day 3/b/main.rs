use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn we_be_sleddin(offset_right: usize, offset_down: usize) -> Result<(i32), Box<dyn Error>> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let hill: Vec<String> = contents.trim().split("\n")
                                .map(|c| c.to_string())
                                .collect();

    let mut total_count: i32 = 0;
    let mut current_offset: usize = 0;
    let width = hill[0].len() - 1;

    for i in (0..hill.len()).step_by(offset_down) {
        if hill[i].chars().collect::<Vec<char>>()[current_offset] == '#' {
            total_count += 1;
        }
        current_offset += offset_right;
        current_offset = current_offset % width;
    }

    Ok(total_count)
}

fn main() -> Result<(), Box<dyn Error>> {
    let one = we_be_sleddin(1, 1).unwrap();
    let two = we_be_sleddin(3, 1).unwrap();
    let three = we_be_sleddin(5, 1).unwrap();
    let four = we_be_sleddin(7, 1).unwrap();
    let five = we_be_sleddin(1, 2).unwrap();

    println!("{}", one);
    println!("{}", two);
    println!("{}", three);
    println!("{}", four);
    println!("{}", five);
    println!("{}", one * two * three * four * five);

    Ok(())
}