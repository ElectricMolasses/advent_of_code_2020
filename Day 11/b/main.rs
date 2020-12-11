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

fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        println!();
        for character in line {
            print!("{}", *character);
        }
    }
    println!();
}

fn run_until_stable(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut current_grid: Vec<Vec<char>> = grid.clone();
    let mut next_grid: Vec<Vec<char>>;

    next_grid = game_of_life(&current_grid, 1);

    while !do_grids_match(&current_grid, &next_grid) {
        print_grid(&current_grid);
        current_grid = next_grid.clone();
        next_grid = game_of_life(&current_grid, 1);
    }
    print_grid(&next_grid);

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

    for _t in 0..turns_to_run {
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                
                // Empty
                if grid[y][x] == 'L' {
                    let occupied_seats = count_visible_occupied_seats(&current_grid, x as i32, y as i32);
                    if occupied_seats == 0 {
                        next_grid[y][x] = '#';
                    }
                }
                if grid[y][x] == '#' {
                    let occupied_seats = count_visible_occupied_seats(&current_grid, x as i32, y as i32);
                    if occupied_seats >= 5 {
                        next_grid[y][x] = 'L';
                    }
                }

            }
        }
        current_grid = next_grid.clone();
    }

    return next_grid;
}

fn count_visible_occupied_seats(grid: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {
    let mut count: i32 = 0;
    // upper range is not inclusive.
    // Instead of checking a box check in all 8 straight directions.
    
    // Check to Left
    for nx in (0..x).rev() {
        if grid[y as usize][nx as usize] == 'L' {
            break;
        }
        if grid[y as usize][nx as usize] == '#' {
            count += 1;
            break;
        }
    }
    // Check to Right
    for nx in x+1..grid[y as usize].len() as i32 {
        if grid[y as usize][nx as usize] == 'L' {
            break;
        }
        if grid[y as usize][nx as usize] == '#' {
            count += 1;
            break;
        }
    }
    // Check Above
    for ny in (0..y).rev() {
        if grid[ny as usize][x as usize] == 'L' {
            break;
        }
        if grid[ny as usize][x as usize] == '#' {
            count += 1;
            break;
        }
    }
    // Check Below
    for ny in y+1..grid.len() as i32 {
        if grid[ny as usize][x as usize] == 'L' {
            break;
        }
        if grid[ny as usize][x as usize] == '#' {
            count += 1;
            break;
        }
    }

    // dIaGoNaL tImE
    let mut current_x: i32 = x - 1;
    let mut current_y: i32 = y - 1;
    while current_x >= 0 && current_x < grid[0].len() as i32
            && current_y >= 0 && current_y < grid.len() as i32 {
                if grid[current_y as usize][current_x as usize] == 'L' {
                    break;
                }
                if grid[current_y as usize][current_x as usize] == '#' {
                    count += 1;
                    break;
                }
                current_x -= 1;
                current_y -= 1;
    }

    current_x = x + 1;
    current_y = y + 1;
    while current_x >= 0 && current_x < grid[0].len() as i32
            && current_y >= 0 && current_y < grid.len() as i32 {
                if grid[current_y as usize][current_x as usize] == 'L' {
                    break;
                }
                if grid[current_y as usize][current_x as usize] == '#' {
                    count += 1;
                    break;
                }
                current_x += 1;
                current_y += 1;
    }

    current_x = x + 1;
    current_y = y - 1;
    while current_x >= 0 && current_x < grid[0].len() as i32
            && current_y >= 0 && current_y < grid.len() as i32 {
                if grid[current_y as usize][current_x as usize] == 'L' {
                    break;
                }
                if grid[current_y as usize][current_x as usize] == '#' {
                    count += 1;
                    break;
                }
                current_x += 1;
                current_y -= 1;
    }

    current_x = x - 1;
    current_y = y + 1;
    while current_x >= 0 && current_x < grid[0].len() as i32
            && current_y >= 0 && current_y < grid.len() as i32 {
                if grid[current_y as usize][current_x as usize] == 'L' {
                    break;
                }
                if grid[current_y as usize][current_x as usize] == '#' {
                    count += 1;
                    break;
                }
                current_x -= 1;
                current_y += 1;
    }

    return count;
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = day_11();

    Ok(())
}