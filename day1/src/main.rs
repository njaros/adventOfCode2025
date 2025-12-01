use lib_aoc::input_lib;

fn do_op(acc: i32, mut n: i32, modulo: i32) -> (i32, i32) {
    let mut clicks: i32 = 0;
    let result: i32;

    if n < 0 {
        clicks += (-n) / modulo;
        n = n + (modulo * clicks);
    }
    
    let sum = acc + n;

    if sum <= 0 {
        result = (modulo + sum) % modulo;
        match acc == 0 {
            true => {},
            false => {
                clicks += 1;
            }
        }
    }
    else
    {
        result = sum % modulo;
        clicks += sum / modulo;
    }
    return (clicks, result);
}

fn main() {
    let part = input_lib::get_part();
    let input: String = input_lib::get_input_as_string(file!(), false);
    let mut result = 0;

    let total = input
    .split('\n')
    .collect::<Vec<&str>>()
    .iter()
    .fold(50 as i32, |acc, line| {
        let (side, nb_as_str) = line.split_at(1);
        match nb_as_str.parse::<i32>() {
            Err(_) => { panic!() }
            Ok(n) => {
                match side {
                    "R" => {
                        let next = do_op(acc, n, 100);
                        match part {
                            1 => {
                                if next.1 == 0 {
                                    result += 1;
                                }
                            }
                            _ => {
                                result  += next.0;
                            }
                        }
                        return next.1;
                    }
                    _ => {
                        let next = do_op(acc, -n, 100);
                        match part {
                            1 => {
                                if next.1 == 0 {
                                    result += 1;
                                }
                            }
                            _ => {
                                result  += next.0;
                            }
                        }
                        return next.1;
                    }
                }
            }
        }
    });

    println!("{} -> {}", total, result);
}
