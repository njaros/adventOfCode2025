use lib_aoc::input_lib;
use lib_aoc::math;
use std::cmp::max;

fn all_nine(joltage: &Vec<u32>) -> bool {
    !joltage
    .iter()
    .any(|n| *n != 9)
}

fn reset_joltage(joltage: &mut Vec<u32>) {
    joltage
    .iter_mut()
    .for_each(|n| *n = 0);
}

fn joltage_to_number(joltage: &Vec<u32>) -> u64 {
    joltage
    .iter()
    .rev()
    .enumerate()
    .fold(0u64, |acc, (i, n)| {
        acc + math::u64_pow(10, i as u64) * *n as u64
    })
}

fn main() {
    let part = input_lib::get_part();
    let input = input_lib::get_input_as_string(file!(), false);

    let mut joltage = Vec::<u32>::new();

    for _ in 0..match part {1 => {2}, _ => {12}} {
        joltage.push(0u32);
    }

    let result: u64 = input
    .split('\n')
    .fold(0, |acc, s| {
        let bank = s.chars()
        .map(|c| c.to_digit(10u32).unwrap())
        .collect::<Vec<u32>>();
        let mut idx: usize = 0;

        reset_joltage(&mut joltage);

        while idx < bank.len() && !all_nine(&joltage) {
            let current_batterie = bank[idx];
            'outer: for idx_joltage in max(0i32, joltage.len() as i32 - (bank.len() as i32 - idx as i32)) as usize..joltage.len() {
                if joltage[idx_joltage] < current_batterie {
                    joltage[idx_joltage] = current_batterie;
                    for idx_joltage_nexts in idx_joltage + 1..joltage.len() {
                        joltage[idx_joltage_nexts] = 0;
                        break 'outer;
                    }
                }
            }
            idx += 1;
        }
        acc + joltage_to_number(&joltage)
    });
    println!("{result}");
}
