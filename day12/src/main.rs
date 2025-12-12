use lib_aoc::input_lib;

fn main() {
    let mut input = input_lib::get_input_as_string(file!(), false);

    input.retain(|c| c != '\r');

    let (shapes_raw, regions_raw) = input
    .rsplit_once("\n\n")
    .unwrap();
    
    let shapes: Vec<u64> = shapes_raw
    .split("\n\n")
    .fold(Vec::<u64>::new(), |mut acc, shape_raw| {
        acc.push(
            shape_raw
            .chars()
            .fold(0u64, |acc, c| match c {'#' => acc + 1, _ => acc})
        );
        acc
    });

    println!("{}", regions_raw
    .split('\n')
    .filter(|line| {
        let (area_raw, shape_list_raw) = line.split_once(": ").unwrap();
        let area = area_raw
        .split('x')
        .fold(1u64, |acc, n_raw| acc * n_raw.parse::<u64>().unwrap());
        area >= shape_list_raw
                    .split(' ')
                    .enumerate()
                    .map(|(idx, n_raw)| {
                        shapes[idx] * n_raw.parse::<u64>().unwrap()
                    })
                    .sum()
    }).count())
}
