use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

// . Floor
// L Empty Seat
// # Occupied Seat

fn day_11() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        grid.push(Vec::new());
        let length = grid.len();
        
        for character in line.chars() {
            grid[length - 1].push(character);
        }
    }

    // If a seat is empty, and there are no occupied adjacent seats, the seat becomes occupied.
    // If a seat is occupied and four or more seats adjacent to it are occupied, it becomes empty. 

    // game_of_life(&grid, 1);
    
    grid = run_until_stable(&grid);
    println!("{}", count_occupied_seats(&grid));

    Ok(())
}

fn count_occupied_seats(grid: &Vec<Vec<char>>) -> i32 {
    let mut count: i32 = 0;

    for line in grid {
        for value in line {
            if *value == '#' {
                count += 1;
            }
        }
    }

    return count;
}

fn run_until_stable(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut current_grid: Vec<Vec<char>> = grid.clone();
    let mut next_grid: Vec<Vec<char>> = grid.clone();

    next_grid = game_of_life(&current_grid, 1);

    while !do_grids_match(&current_grid, &next_grid) {
        current_grid = next_grid.clone();
        next_grid = game_of_life(&current_grid, 1);
    }

    return next_grid;
}

fn do_grids_match(grid_a: &Vec<Vec<char>>, grid_b: &Vec<Vec<char>>) -> bool {
    for i in 0..grid_a.len() {
        for j in 0..grid_a[i].len() {
            if grid_a[i][j] != grid_b[i][j] {
                return false;
            }
        }
    }
    return true;
}

fn game_of_life(grid: &Vec<Vec<char>>, turns_to_run: i32) -> Vec<Vec<char>> {
    let mut current_grid: Vec<Vec<char>> = grid.clone();
    let mut next_grid: Vec<Vec<char>> = grid.clone();

    for t in 0..turns_to_run {
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                
                // Empty
                if grid[y][x] == 'L' {
                    let occupied_seats = count_adjacent_occupied_seats(&current_grid, x as i32, y as i32);
                    if occupied_seats == 0 {
                        next_grid[y][x] = '#';
                    }
                }
                if grid[y][x] == '#' {
                    let occupied_seats = count_adjacent_occupied_seats(&current_grid, x as i32, y as i32);
                    if occupied_seats >= 4 {
                        next_grid[y][x] = 'L';
                    }
                }

            }
        }
        current_grid = next_grid.clone();
    }

    return next_grid;
}

fn count_adjacent_occupied_seats(grid: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {
    let mut count: i32 = 0;
    // upper range is not inclusive.
    for ny in (y-1)..(y+2) {
        for nx in (x-1)..(x+2) {
            if (ny >= 0 && nx >= 0
             && ny < grid.len() as i32 && nx < grid[ny as usize].len() as i32) {
                
                if nx == x && ny == y {
                    // nothing
                } else if grid[ny as usize][nx as usize] == '#' {
                    count += 1;
                }
            }
        }
    }

    return count;
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = day_11();

    Ok(())
}