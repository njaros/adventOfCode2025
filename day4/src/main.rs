use lib_aoc::input_lib;
use std::cmp::min;
use std::cmp::max;
use std::{thread, time};
use std::time::Duration;

const TIME: Duration = time::Duration::from_millis(20);

fn display_grid(grid: &Vec<Vec<u32>>) {
    for l in grid {
        for val in l {
            match val {
                1 => {print!("@")},
                _ => {print!(".")}
            }
        }
        println!();
    }
    println!();
    thread::sleep(TIME);
}

fn count_neighboors(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut count = 0u32;
    let y_max = grid.len() as i32;
    let x_max = grid[0].len() as i32;

    for n_y in max(0, y as i32 - 1)..min(y_max, y as i32 + 2) {
        for n_x in max(0, x as i32 - 1)..min(x_max, x as i32 + 2) {
            if ((n_y as usize != y) || (n_x as usize != x)) && grid[n_y as usize][n_x as usize] == 1 {
                count += 1;
            }
        }
    }

    count
}

fn unreachable_rolls(grid: &mut Vec<Vec<u32>>) -> u32 {
    let mut result: u32 = 0;
    let mut unreachable_coordinates = Vec::<(usize, usize)>::new();
    
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 1 && count_neighboors(grid, x, y) < 4 {
                result += 1;
                unreachable_coordinates.push((x, y));
            }
        }
    }
    unreachable_coordinates
    .iter()
    .for_each(|(x, y)| { grid[*y][*x] = 0 });

    result
}

fn main() {
    let part = input_lib::get_part();
    let input = input_lib::get_input_as_string(file!(), false);

    let mut grid = input
    .split('\n')
    .map(|line| {
        line
        .chars()
        .map(|c| {
            match c {
                '@' => {1u32},
                _ => {0u32}
            }
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

    display_grid(&grid);

    if part == 1 {
        println!("{}", unreachable_rolls(&mut grid));
    }
    else {
        let mut result = 0u32;
        let mut partial_result = unreachable_rolls(&mut grid);

        while partial_result != 0 {
            display_grid(&grid);
            result += partial_result;
            partial_result = unreachable_rolls(&mut grid);    
        }
        display_grid(&grid);
        println!("{result}");
    }

}
