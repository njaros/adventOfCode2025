use lib_aoc::input_lib;
use std::collections::HashMap;

fn recu_beam_dfs_p1(sapin: &mut Vec<Vec<u8>>, (x, y): (usize, usize)) -> u64 {
    let mut res = 0u64;

    for y_loop in y..sapin.len() {
        if sapin[y_loop][x] == b'^' {
            res += 1;
            if x != 0 && sapin[y_loop][x - 1] != b'|' {
                res += recu_beam_dfs_p1(sapin, (x - 1, y_loop));
            }
            if x != sapin.len() - 1 && sapin[y_loop][x + 1] != b'|' {
                res += recu_beam_dfs_p1(sapin, (x + 1, y_loop));
            }
            break;
        }
        else {
            if sapin[y_loop][x] != b'|' {
                sapin[y_loop][x] = b'|';
            }
            else {
                break;
            }
        }
    }
    res
}

fn recu_beam_dfs_p2(sapin: &mut Vec<Vec<u8>>, (x, y): (usize, usize), cache: &mut HashMap<(usize, usize), u64>) -> u64 {
    let mut res = 0u64;
    if cache.contains_key(&(x, y)) {
        return cache[&(x, y)];
    }

    for y_loop in y..sapin.len() {
        if sapin[y_loop][x] == b'^' {
            if x != 0 {
                res += recu_beam_dfs_p2(sapin, (x - 1, y_loop), cache);
            }
            if x != sapin.len() - 1 {
                res += recu_beam_dfs_p2(sapin, (x + 1, y_loop), cache);
            }
            cache.insert((x, y), res);
            return res;
        }
    }
    1
}
fn main() {
    let part = input_lib::get_part();
    let input = input_lib::get_input_as_string(file!(), false);

    let mut sapin = input
    .split('\n')
    .map(|line| line
        .as_bytes()
        .iter()
        .map(|c| *c)
        .collect::<Vec<u8>>())
    .collect::<Vec<Vec<u8>>>();

    let start = (sapin[0]
        .iter()
        .enumerate()
        .find(|(_, x)| **x == b'S')
        .unwrap().0, 0);

    let result;

    if part == 1 {
        result = recu_beam_dfs_p1(&mut sapin, start);
    }
    else {
        let mut cache = HashMap::<(usize, usize), u64>::new();
        result = recu_beam_dfs_p2(&mut sapin, start, &mut cache);
    }

    println!("{result}");
}
