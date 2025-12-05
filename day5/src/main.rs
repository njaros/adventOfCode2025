use lib_aoc::input_lib;
use std::cmp::min;
use std::cmp::max;

fn overlap(a: (u64, u64), b: (u64, u64)) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}

fn main() {
    let part = input_lib::get_part();
    let input = input_lib::get_input_as_string(file!(), false);

    let (str_ranges, str_ingredients) = input
    .split_once("\n\n")
    .unwrap();
    
    let mut ranges = str_ranges
    .split('\n')
    .map(|range| {
        let (left, right) = range.split_once("-")
        .unwrap();
        ((left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap()), false)
    })
    .collect::<Vec<_>>();

    let ingredients = str_ingredients
    .split('\n')
    .map(|ingr| { ingr.parse::<u64>().unwrap() })
    .collect::<Vec<_>>();

    let result: u64;

    if part == 1 {
        result = ingredients
        .iter()
        .fold(0u64, |acc, ingr| {
            let mut ret = acc;
            for idx in 0..ranges.len() {
                if *ingr >= ranges[idx].0.0 && *ingr <= ranges[idx].0.1 {
                    ret += 1;
                    break;
                }
            }
            ret
        });
    }
    else {

        for idx_outer in 0..ranges.len() {
            if !ranges[idx_outer].1 {
                for idx_inner in idx_outer + 1..ranges.len() {
                    if !ranges[idx_inner].1 && overlap(ranges[idx_outer].0, ranges[idx_inner].0) {
                        ranges[idx_outer].1 = true;
                        ranges[idx_inner].0 = (
                            min(ranges[idx_inner].0.0, ranges[idx_outer].0.0),
                            max(ranges[idx_inner].0.1, ranges[idx_outer].0.1)
                        );
                    }
                }
            }
        }

        result = ranges
        .iter()
        .fold(0u64, |acc, ((left, right), eaten)| {
            match eaten {
                true => {acc},
                false => acc + (right - left) + 1
            }
        });
        
    }

    println!("{result}")

}
