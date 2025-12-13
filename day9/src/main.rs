use lib_aoc::input_lib;
use itertools::Itertools;
use lib_aoc::math::sub_abs;

fn reorder_points(a: u64, b: u64) -> (u64, u64) {
    if a <= b {
        return (a, b);
    }
    (b, a)
}

fn seg_intersect_rectangle(segment: &Vec<(u64, u64)>, rectangle: &Vec<&(u64, u64)>) -> bool {
    let a = segment[0];
    let b = segment[1];

    let (min_y, max_y) = reorder_points(rectangle[0].1, rectangle[1].1);
    let (min_x, max_x) = reorder_points(rectangle[0].0, rectangle[1].0);

    // is horizontal
    if a.1 == b.1 {
        // potentially cutting
        if a.1 > min_y && a.1 < max_y {
            let (min_seg_x, max_seg_x) = reorder_points(a.0, b.0);
            return !(max_seg_x <= min_x || min_seg_x >= max_x);
        }
    }
    //is vertical
    else {
        // potentially cutting
        if a.0 > min_x && a.0 < max_y {
            let (min_seg_y, max_seg_y) = reorder_points(a.1, b.1);
            return !(max_seg_y <= min_y || min_seg_y >= max_y);
        }
    }
    false
}
fn main() {
    let part = input_lib::get_part();
    let input = input_lib::get_input_as_string(file!(), false);

    let corners = input.split_whitespace()
    .map(|s| match s
        .split(',')
        .collect::<Vec<_>>()[..] {
            [first, second, ..] => (
                first.parse::<u64>().unwrap(),
                second.parse::<u64>().unwrap()
            ),
            _ => unreachable!()
        }
    )
    .collect_vec();

    let res = match part {
        1 => {
            corners
            .iter()
            .combinations(2)
            .map(|comb| (sub_abs(comb[0].0, comb[1].0) + 1) * (sub_abs(comb[0].1, comb[1].1) + 1))
            .max()
            .unwrap()
        },
        _ => {
            let pairs = corners
            .iter()
            .combinations(2);

            let mut segments = Vec::<Vec<(u64, u64)>>::new();

            for i in 0..corners.len() - 1 {
                segments.push(vec!(corners[i], corners[i + 1]));
            }
            
            let mut remains = Vec::<Vec<&(u64, u64)>>::new();
            for pair in pairs {
                if !segments.iter().any(|seg| seg_intersect_rectangle(&seg, &pair)) {
                    remains.push(pair);
                }
            }

            remains
            .iter()
            .map(|comb| (sub_abs(comb[0].0, comb[1].0) + 1) * (sub_abs(comb[0].1, comb[1].1) + 1))
            .max()
            .unwrap()
        }
    };

    println!("{res}");
}
