use lib_aoc::input_lib;
use lp_solvers::{lp_format::{Constraint, LpObjective, LpProblem}, problem::{Problem, StrExpression, Variable}, solvers::{CbcSolver, SolverTrait}};
use std::collections::VecDeque;
use itertools::Itertools;
use lib_aoc::math::u64_pow;

fn push_button_p1(button: &Vec<u64>, lights: u64) -> u64 {
    lights ^ button
    .iter()
    .fold(0u64, |acc, n| acc + u64_pow(2, *n))
}

fn bfs_p1(line: &str) -> u64 {
    // Beware ! the [..##..] things are little endian
    let goal: u64;
    let buttons: Vec::<Vec::<u64>>;

    let vec_to_parse = line
    .split(' ')
    .collect_vec();

    goal = vec_to_parse[0]
    .chars()
    .enumerate()
    .fold(0u64, |acc, (idx, letter)| {
        match letter {
            '#' => acc ^ u64_pow(2, idx as u64 - 1), // - 1 because the letter is '[' first.
            _ => acc
        }
    });

    buttons = vec_to_parse[1..vec_to_parse.len() - 1]
    .iter()
    .fold(Vec::<Vec::<u64>>::new(), |mut acc, button_as_str| {
        acc.push(
            (&button_as_str[1..button_as_str.len() - 1])
            .split(',')
            .map(|n_as_str| n_as_str.parse::<u64>().unwrap())
            .collect_vec()
        );
        acc
    });

    let mut jobs = VecDeque::<(u64, Vec<u64>, u64)>::new();
    for button in &buttons[..] {
        jobs.push_back((0, button.to_vec(), 0));
    }

    let mut current = 0u64;
    let mut shortest = 0u64;
    while current != goal {
        let job = jobs.pop_front().unwrap();
        shortest = job.2 + 1;
        current = push_button_p1(&job.1, job.0);
        for button in &buttons[..] {
            // Prevent push same button 2 time in a row.
            if *button != job.1 {
                jobs.push_back((current, button.to_vec(), shortest));
            }
        }
    }

    shortest
}

fn print_vec(v: Vec<u64>) {
    print!("[");
    for i in 0..v.len() {
        print!("{}", v[i]);
        if i < v.len() - 1 {
            print!(", ");
        }
    }
    print!("]");
}

fn print_vecln(v: Vec<u64>) {
    print_vec(v);
    println!();
}

fn sub_max(v: &mut Vec<u64>, sub: &Vec<u64>) -> u64 {
    let min = sub
    .iter()
    .map(|idx| v[*idx as usize])
    .min()
    .unwrap();

    sub
    .iter()
    .for_each(|idx| v[*idx as usize] -= min);

    min
}

fn apply_sub(v: &mut Vec<u64>, sub: &Vec<u64>, times: u64) {
    sub
    .iter()
    .for_each(|idx| v[*idx as usize] -= times);
}

fn cancel_sub(v: &mut Vec<u64>, sub: &Vec<u64>, times: u64) {
    sub
    .iter()
    .for_each(|idx| v[*idx as usize] += times);
}

fn get_max_sub(v: &Vec<u64>, sub: &Vec<u64>) -> u64 {
    sub
    .iter()
    .map(|idx| v[*idx as usize])
    .min()
    .unwrap()
}

fn find_coefficients(mut goal: Vec::<u64>, buttons: &Vec::<Vec::<u64>>) -> Option<u64> {
    let sum = buttons
    .iter()
    .fold(0u64, |acc, button| acc + sub_max(&mut goal, &button));

    match goal.iter().any(|x| *x != 0) {
        false => {
            println!("{sum}");
            Some(sum)
        },
        true => None
    }
}

fn recu_comb(all_comb: &mut Vec::<Vec::<Vec::<u64>>>, building_elt: &mut Vec::<Vec::<u64>>, remainings: Vec::<Vec::<u64>>) {
    if remainings.is_empty() {
        all_comb.push(building_elt.clone());
    }
    else {
        for i in 0..remainings.len() {
            building_elt.push(remainings[i].clone());
            let mut copy = remainings.clone();
            copy.remove(i);
            recu_comb(all_comb, building_elt, copy);
            building_elt.pop();
        }
    }
}

fn all_combination(buttons: Vec<Vec<u64>>) -> Vec<Vec<Vec<u64>>> {
    let mut combinations = Vec::<Vec::<Vec::<u64>>>::new();

    recu_comb(&mut combinations, &mut Vec::<Vec::<u64>>::new(), buttons.clone());
    println!("{} -- {}", buttons.len(), combinations.len());
    combinations
}

fn algo_de_con_recu(buttons: &Vec<Vec<u64>>, goal: &mut Vec<u64>, n: usize) -> Option<u64> {
    if !goal.iter().any(|n| *n != 0)
    {
        return Some(0);
    }
    if n >= buttons.len()
    {
        return None;
    }
    for i in (0..=get_max_sub(&goal, &buttons[n])).rev() {
        apply_sub(goal, &buttons[n], i);
        match algo_de_con_recu(buttons, goal, n + 1) {
            Some(val) => return Some(val + i),
            None => {}
        }
        cancel_sub(goal, &buttons[n], i);
    }
    None
}

fn linear_combination_p2(line: &str) -> u64 {
    // Beware ! the [..##..] things are little endian
    let mut goal: Vec<u64>;

    let vec_to_parse = line
    .split(' ')
    .collect_vec();

    goal = vec_to_parse[vec_to_parse.len() - 1]
    .trim_start_matches('{')
    .trim_end_matches('}')
    .split(',')
    .map(|n_as_str| n_as_str.parse::<u64>().unwrap())
    .collect_vec();

    let buttons = vec_to_parse[1..vec_to_parse.len() - 1]
    .iter()
    .fold(Vec::<Vec::<u64>>::new(), |mut acc, button_as_str| {
        acc.push(
            (&button_as_str[1..button_as_str.len() - 1])
            .split(',')
            .map(|n_as_str| n_as_str.parse::<u64>().unwrap())
            .collect_vec()
        );
        acc
    });

    // let res = all_combination(buttons)
    // .iter()
    // .map(|comb| find_coefficients(goal.clone(), &comb))
    // .filter(|coeff| coeff.is_some())
    // .map(|wrapped| wrapped.unwrap())
    // .min()
    // .unwrap();

    let res = algo_de_con_recu(&buttons, &mut goal, 0).unwrap();

    println!("{res}");

    res

}

fn do_lp_solver_of_shame(line: &str) -> f32 {
    type Vector = Vec<u8>;
    
    let (head, constraints_raw) = line.rsplit_once(' ').unwrap();
    let (_, vectors_raw) = head.split_once(' ').unwrap();
    let goal: Vec<u8> = constraints_raw
    .trim_end_matches('}')
    .trim_start_matches('{')
    .split(',')
    .map(|n| n.parse::<u8>().unwrap())
    .collect_vec();

    let dim = goal.len();

    let vectors: Vec<Vector> = vectors_raw
    .split(' ')
    .fold(Vec::<Vector>::new(), |mut acc, vector_raw| {
        acc.push(
            vector_raw
            .trim_end_matches(')')
            .trim_start_matches('(')
            .split(',')
            .fold(vec![0u8; dim], |mut acc, n| {
                acc[n.parse::<usize>().unwrap()] = 1;
                acc
            })
        );
        acc
    });

    let mut objective_vec = Vec::<String>::new();

    let variables: Vec::<Variable> = (0..vectors.len())
    .fold(Vec::<Variable>::new(), |mut acc, n| {
        let mut name = String::from("btn");
        objective_vec.push(name.clone());
        name.push_str(n.to_string().as_str());
        acc.push(Variable {
            name: name,
            is_integer: true,
            lower_bound: 0.,
            upper_bound: 100.
        });
        acc
    });

    let objective = StrExpression(objective_vec.iter().join(" + "));
    
    let constraints: Vec<Constraint<StrExpression>> = goal
    .iter()
    .enumerate()
    .map(|(idx, &g)| {
        let mut expr_vec = Vec::<String>::new();
        for i in 0..vectors.len() {
            if vectors[i][idx] != 0 {
                expr_vec.push(objective_vec[i].clone());
            }
        }
        Constraint {
            lhs: StrExpression(expr_vec.iter().join(" + ")),
            operator: std::cmp::Ordering::Equal,
            rhs: g as f64
        }
    })
    .collect_vec();

    let solver = CbcSolver::default();
    let pb = Problem {
        name: String::from("aoc_solver"),
        sense: LpObjective::Minimize,
        objective: objective,
        variables: variables,
        constraints: constraints
    };

    let solution = solver.run(&pb);
    solution.expect("fail").results.into_values().sum()
}

fn main() {
    let part = input_lib::get_part();
    let mut input = input_lib::get_input_as_string(file!(), false);

    input.retain(|c| c != '\r');

    match part {
        1 => {
            let result = input
            .split('\n')
            .fold(0u64, |acc, line| {
                acc + bfs_p1(line)
            });
            println!("{result}");
        }
        _ => {
            let result = input
            .split('\n')
            .fold(0f32, |acc, line| {
                acc + do_lp_solver_of_shame(line)
            });
            println!("{result}");
        }
    }
}
