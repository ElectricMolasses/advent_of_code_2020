use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn read_what_the_fuck() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut required_occurences: Vec<(usize, usize)> = Vec::new();
    let mut required_letter: Vec<char> = Vec::new();
    let mut password_list: Vec<String> = Vec::new();

    let mut total_count: i32 = 0;

    for s in contents.split("\n") {
        let pieces: Vec<&str> = s.trim().split(|c| c == '-' || c == ' ' || c == ':').collect();

        // Extra space on index 4, not going to fix
        required_occurences.push((pieces[0].parse::<usize>().unwrap(), pieces[1].parse::<usize>().unwrap()));
        required_letter.push(pieces[2].parse::<char>().unwrap());
        password_list.push(pieces[4].to_string());

        for piece in pieces {
            println!("{}", piece);
        }
    }

    for i in 0..required_occurences.len() {
        let mut character_count: i32 = 0;

        let password_chars: Vec<char> = password_list[i].chars().collect();
        
        let (mut low, mut high) = required_occurences[i];
        low -= 1;
        high -= 1;

        if (password_chars[low] == required_letter[i] && password_chars[high] != required_letter[i])
        || (password_chars[low] != required_letter[i] && password_chars[high] == required_letter[i]) {
            total_count += 1;
        }
    }

    println!("{}", total_count);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = read_what_the_fuck();

    Ok(())
}