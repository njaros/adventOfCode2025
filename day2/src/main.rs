use lib_aoc::input_lib;
use lib_aoc::math;

fn log_10(mut a: u64) -> u64 {
    let mut res: u64 = 0;

    while a > 0 {
        a /= 10;
        res += 1;
    }
    res
}

fn find_invalid_ids_p1(a: u64, b: u64) -> u64 {
    let mut result = 0;
    
    for i in a..b + 1 {
        let half_log = math::u64_pow(10u64, log_10(i) / 2u64);
        let right_half = i / half_log;
        let left_half = right_half * half_log;
        if i == left_half + right_half {
            result += i;
        }
    }
    result
}

fn find_invalid_ids_p2(a: u64, b: u64) -> u64 {
    let mut result = 0;
    
    for i in a..b + 1 {
        let log = log_10(i);
        for j in 1..(log / 2) + 1 {
            if log % j == 0 {
                let modulo = math::u64_pow(10u64, j);
                let compare_to = i % modulo;
                let mut tmp = i;
                while tmp != 0 && tmp % modulo == compare_to {
                    tmp = tmp / modulo;
                }
                if tmp == 0 {
                    result += i;
                    break
                }
            }
        }
    }
    result
}

fn main() {
    let part = input_lib::get_part();
    let mut input = input_lib::get_input_as_string(file!(), false);

    input.retain(|c| c != '\n');
    let res: u64;

    res = input
    .split(',')
    .collect::<Vec<&str>>()
    .iter()
    .fold(0u64, |acc, range| {
        let pair = range.split('-')
        .collect::<Vec<&str>>();
        let a = pair[0].parse::<u64>().unwrap();
        let b = pair[1].parse::<u64>().unwrap();
        match part {
                1 => {
                acc + find_invalid_ids_p1(a, b)
            }
            _ => {
                acc + find_invalid_ids_p2(a, b)
            }
        }
    });

    println!("{res}");
}
