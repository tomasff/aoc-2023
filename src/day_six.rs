fn count_race_solutions(time: f64, distance: f64) -> i64 {
    let sq = (time * time - 4.0 * distance).sqrt() / 2.0;

    ((time / 2.0 + sq).ceil() - (time / 2.0 - sq).floor()) as i64 - 1
}

fn solve_part_one(input: &str) -> i64 {
    input
        .split_once('\n')
        .map(|(times, distances)| {
            times
                .split_whitespace()
                .skip(1)
                .zip(distances.split_whitespace().skip(1))
                .filter_map(|(time, distance)| match (time.parse(), distance.parse()) {
                    (Ok(t), Ok(d)) => Some((t, d)),
                    _ => None,
                })
                .map(|(time, distance)| count_race_solutions(time, distance))
                .product()
        })
        .expect("Invalid puzzle input.")
}

fn solve_part_two(input: &str) -> i64 {
    input
        .split_once('\n')
        .map(|(times, distances)| {
            let time: f64 = times
                .split_whitespace()
                .skip(1)
                .collect::<String>()
                .parse()
                .expect("Invalid race time.");

            let distance: f64 = distances
                .split_whitespace()
                .skip(1)
                .collect::<String>()
                .parse()
                .expect("Invalid race distance.");

            count_race_solutions(time, distance)
        })
        .expect("Invalid puzzle input.")
}

pub fn solve(input: &str) -> (i64, i64) {
    (solve_part_one(input), solve_part_two(input))
}
