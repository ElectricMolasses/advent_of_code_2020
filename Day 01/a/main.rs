use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let mut number_list: Vec<i32> = Vec::new();

    for s in contents.split("\n") {
        number_list.push(s.trim().parse::<i32>().unwrap());
    }

    for i in 0..(number_list.len()) {
        for j in i+1..(number_list.len()) {
            if number_list[i] + number_list[j] == 2020 {
                println!("{}", number_list[i] * number_list[j]);
                return Ok(())
            }
        }
    }

    Ok(())
}