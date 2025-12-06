use lib_aoc::input_lib;

fn main() {
    let part = input_lib::get_part();
    let mut input = input_lib::get_input_as_string(file!(), false);
    input.retain(|c| c != '\r');

    if part == 1 {    
        let parsing = input
        .split('\n')
        .map(|l| {
            l.split(" ")
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
        
        let mut result = 0u64;
        
        for x in 0..parsing[0].len() {
            result += match parsing.last().unwrap()[x] {
                "*" => {
                    let mut part_res = 1u64;
                    for y in 0..parsing.len() - 1 {
                        part_res *= parsing[y][x].parse::<u64>().unwrap();
                    }
                    part_res
                },
                _ => {
                    let mut part_res = 0u64;
                    for y in 0..parsing.len() - 1 {
                        part_res += parsing[y][x].parse::<u64>().unwrap();
                    }
                    part_res
                }
            };
        }   
        println!("{result}");
    }
    else {
        let lines = input
        .split('\n')
        .collect::<Vec<_>>();

        let operations = lines.last().unwrap()
        .split_ascii_whitespace()
        .collect::<Vec<_>>();

        let mut number_groups = Vec::<Vec::<u64>>::new();
        let mut numbers = Vec::<u64>::new();
        for x in 0..lines[0].len() {
            let mut number = 0u64;
            for y in 0..lines.len() - 1 {
                match lines[y].as_bytes()[x] {
                    n if n >= b'0' && n <= b'9' => {
                        number *= 10;
                        number += (n - b'0') as u64;
                    },
                    _ => {}
                }
            }
            if number != 0 {
                numbers.push(number);
            }
            else {
                number_groups.push(numbers.clone());
                numbers.clear();
            }
        }
        number_groups.push(numbers.clone());

        let res = operations.iter()
        .enumerate()
        .fold(0u64, |acc, (idx, opp)| {
            acc + match *opp {
                "*" => {
                    number_groups[idx].iter()
                    .fold(1u64, |acc, n| acc * *n)
                },
                _ => {
                    number_groups[idx].iter()
                    .fold(0u64, |acc, n| acc + *n)
                }
            }
        });

        println!("{res}");
    }
}
