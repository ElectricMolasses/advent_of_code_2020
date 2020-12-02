use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn read_file_to_i32() -> Result<Vec<i32>, Box<dyn Error>> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let mut number_list: Vec<i32> = Vec::new();

    for s in contents.split("\n") {
        number_list.push(s.trim().parse::<i32>().unwrap());
    }

    Ok(number_list)
}

fn main() -> std::io::Result<()> {
    let number_list = read_file_to_i32().unwrap();

    for i in 0..(number_list.len()) {
        for j in i+1..(number_list.len()) {
            for k in j+1..(number_list.len()) {
                if number_list[i] + number_list[j] + number_list[k] == 2020 {
                    println!("{}", number_list[i] * number_list[j] * number_list[k]);
                    return Ok(())
                }
            }
        }
    }

    Ok(())
}