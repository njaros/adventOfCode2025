use lib_aoc::input_lib;
use lp_solvers::{lp_format::{Constraint, LpObjective}, problem::{Problem, StrExpression, Variable}, solvers::{CbcSolver, SolverTrait}};
use lp_solvers::solvers::Status::Optimal;
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

fn do_lp_solver_of_shame(line: &str) -> f32 {
    type Vector = Vec<u32>;
    
    let (head, constraints_raw) = line.rsplit_once(' ').unwrap();
    let (_, vectors_raw) = head.split_once(' ').unwrap();
    let goal: Vec<u32> = constraints_raw
    .trim_end_matches('}')
    .trim_start_matches('{')
    .split(',')
    .map(|n| n.parse::<u32>().unwrap())
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
            .fold(vec![0u32; dim], |mut acc, n| {
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
        name.push_str(n.to_string().as_str());
        objective_vec.push(name.clone());
        acc.push(Variable {
            name: name,
            is_integer: true,
            lower_bound: 0.,
            upper_bound: 1000.
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

    let solution = solver.run(&pb).expect("fail");
    assert_eq!(solution.status, Optimal);
    let s = solution.results.into_values().sum();
    s
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
