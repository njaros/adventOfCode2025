use lib_aoc::input_lib;
use lib_aoc::math::sub_abs;
use lib_aoc::math::u64_pow;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

fn find_in_directory(dir: &HashMap::<(u64, u64, u64), (u64, u64, u64)>, mut p: (u64, u64, u64)) -> (u64, u64, u64) {
    while p != *dir.get(&p).unwrap() {
        p = *dir.get(&p).unwrap();
    }
    p
}

fn square_dist(p1: (u64, u64, u64), p2: (u64, u64, u64)) -> u64 {
    u64_pow(sub_abs(p1.0, p2.0), 2) + u64_pow(sub_abs(p1.1, p2.1), 2) + u64_pow(sub_abs(p1.2, p2.2), 2)
}

fn main() {
    let part = input_lib::get_part();
    let mut input = input_lib::get_input_as_string(file!(), false);

    input.retain(|c| c != '\r');

    let points = input
    .split_whitespace()
    .fold(Vec::<(u64, u64, u64)>::new(), |mut acc, s| {
        let mut numbers = s.split(',')
        .map(|n_s| n_s.parse::<u64>().unwrap());
        let x = numbers.next().unwrap();
        let y = numbers.next().unwrap();
        let z = numbers.next().unwrap();
        acc.push((x, y, z));
        acc
    });

    let mut distances = BTreeMap::<u64, Vec<((u64, u64, u64), (u64, u64, u64))>>::new();

    for idx in 0..points.len() - 1 {
        for next in idx + 1..points.len() {
            let d = square_dist(points[idx], points[next]);
            match distances.get(&d) {
                None => { distances.insert(d, vec!((points[idx], points[next]))); }
                Some(v) => {
                    let mut new_v = Vec::<((u64, u64, u64), (u64, u64, u64))>::new();
                    for p in v {
                        new_v.push(*p);
                    }
                    new_v.push((points[idx], points[next]));
                    distances.insert(d, new_v);
                }
            }
        }
    }

    let mut eq_class = HashMap::<(u64, u64, u64), HashSet::<(u64, u64, u64)>>::new();

    // directory informing in which key is located each box in the eq_class
    let mut directory = HashMap::<(u64, u64, u64), (u64, u64, u64)>::new();

    points
    .iter()
    .for_each(|p| {
        let mut new_hash_set = HashSet::<(u64, u64, u64)>::new();
        new_hash_set.insert(*p);
        eq_class.insert(*p, new_hash_set);
        directory.insert(*p, *p);
    });

    let mut v = Vec::<((u64, u64, u64), (u64, u64, u64))>::new();
    if part == 1 {
        for _ in 0..1000 {
            if v.is_empty() {
                v = distances.pop_first().unwrap().1;
            }
            let (p1, p2) = v.pop().unwrap();
            let eater = find_in_directory(&directory, p1);
            let eaten = find_in_directory(&directory, p2);
            if eater != eaten {
                directory.insert(eaten, eater);
                let eaten_class = (eq_class.get(&eaten).unwrap()).clone();
                let mut niktamer = HashSet::<(u64, u64, u64)>::new();
                let vatefaireencule = eq_class.get(&eater);
                match vatefaireencule {
                    None => {panic!()},
                    Some(fdp) => {
                        for elt in fdp {
                            niktamer.insert(*elt);
                        }
                    }
                }
                eaten_class.iter()
                .for_each(|to_eat| {
                    niktamer.insert(*to_eat);
                });
                eq_class.insert(eater, niktamer);
                eq_class.remove(&eaten);
            }
        }
        
        let mut size = eq_class.iter()
        .map(|(_, v)| v.len())
        .collect::<Vec<_>>();
    
        println!("{}", eq_class.len());
    
        size.sort_by(|a, b| b.cmp(a));
    
        let mut result = 1u64;
        for i in 0..3 {
            result *= size[i] as u64;
        }
    
        println!("{result}");
    }

    else {
        let mut x1 = 0u64;
        let mut x2 = 0u64;
        while eq_class.len() > 1 {
            if v.is_empty() {
                v = distances.pop_first().unwrap().1;
            }
            let (p1, p2) = v.pop().unwrap();
            x1 = p1.0;
            x2 = p2.0;
            let eater = find_in_directory(&directory, p1);
            let eaten = find_in_directory(&directory, p2);
            if eater != eaten {
                directory.insert(eaten, eater);
                let eaten_class = (eq_class.get(&eaten).unwrap()).clone();
                let mut niktamer = HashSet::<(u64, u64, u64)>::new();
                let vatefaireencule = eq_class.get(&eater);
                match vatefaireencule {
                    None => {panic!()},
                    Some(fdp) => {
                        for elt in fdp {
                            niktamer.insert(*elt);
                        }
                    }
                }
                eaten_class.iter()
                .for_each(|to_eat| {
                    niktamer.insert(*to_eat);
                });
                eq_class.insert(eater, niktamer);
                eq_class.remove(&eaten);
            }
        }
        println!("{}", x1 * x2);
    }
}
