use lib_aoc::input_lib;
use std::collections::HashMap;
use itertools::Itertools;

type Nodes = HashMap::<String, Vec::<String>>;

fn baby_dfs(nodes: &Nodes, devices: Vec<String>, objectiv: &str, memo: &mut HashMap::<String, u64>) -> u64 {
    devices
    .iter()
    .fold(0u64, |acc, device| {
        match memo.get(device) {
            Some(val) => acc + val,
            _ => {
                match device.as_str() {
                    _ if device == objectiv => acc + 1,
                    _ => {
                        let res = match nodes.get(device) {
                            Some(pouet) => baby_dfs(nodes, pouet.clone(), objectiv, memo),
                            None => 0
                        };
                        memo.insert(device.clone(), res);
                        acc + res
                    }
                }
            }
        }
    })
}

fn main() {
    let part = input_lib::get_part();
    let mut input = input_lib::get_input_as_string(file!(), false);

    input.retain(|c| c != '\r');

    let mut nodes = Nodes::new();
    input
    .split('\n')
    .for_each(|s| {
        match s.split_once(':') {
            None => unreachable!(),
            Some((key, values)) => {
                nodes
                .insert(
                    String::from(key),
                    values[1..]
                    .split(' ')
                    .map(|split_unusable_type| String::from(split_unusable_type))
                    .collect_vec());
            }
        }
    });

    let mut memo = HashMap::<String, u64>::new();

    println!("{}",
        match part {
            1 => baby_dfs(&nodes, nodes.get("you").unwrap().clone(), "out", &mut memo),
            _ => {
                let res;
                let fft_to_dac = baby_dfs(&nodes, nodes.get("fft").unwrap().clone(), "dac", &mut memo);
                memo.clear();
                if fft_to_dac > 0 {
                    let dac_to_out = baby_dfs(&nodes, nodes.get("dac").unwrap().clone(), "out", &mut memo);
                    memo.clear();
                    let svr_to_fft = baby_dfs(&nodes, nodes.get("svr").unwrap().clone(), "fft", &mut memo);
                    res = fft_to_dac * svr_to_fft * dac_to_out;
                }
                else {
                    let dac_to_fft = baby_dfs(&nodes, nodes.get("dac").unwrap().clone(), "fft", &mut memo);
                    memo.clear();
                    let fft_to_out = baby_dfs(&nodes, nodes.get("fft").unwrap().clone(), "out", &mut memo);
                    memo.clear();
                    let svr_to_dac = baby_dfs(&nodes, nodes.get("svr").unwrap().clone(), "dac", &mut memo);
                    res = dac_to_fft * svr_to_dac * fft_to_out;
                }
                res
            }
        })

}
