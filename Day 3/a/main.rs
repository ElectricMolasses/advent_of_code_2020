use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn we_be_sleddin() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let hill: Vec<String> = contents.trim().split("\n")
                                .map(|c| c.to_string())
                                .collect();

    let mut total_count: i32 = 0;
    let mut offset: usize = 0;
    let width = hill[0].len() - 1;

    for i in 0..hill.len() {
        if hill[i].chars().collect::<Vec<char>>()[offset] == '#' {
            total_count += 1;
        }
        offset += 3;
        offset = offset % width;
    }

    println!("{}", total_count);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = we_be_sleddin();

    Ok(())
}