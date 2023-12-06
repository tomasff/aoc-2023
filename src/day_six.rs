use std::iter::zip;

fn solve_part_one(input: &str) -> u32 {
    let races: Vec<Vec<f32>> = input
        .lines()
        .filter_map(|line| {
            line.split_once(":").map(|(_, values)| {
                values
                    .split_whitespace()
                    .filter_map(|number| number.parse().ok())
                    .collect()
            })
        })
        .collect();

    zip(&races[0], &races[1])
        .into_iter()
        .map(|(time, distance)| (time.powi(2) - 4.0 * distance).sqrt().ceil() as u32)
        .product()
}

pub fn solve(input: &str) -> (Option<u32>, Option<u32>) {
    (Some(solve_part_one(input)), None)
}
