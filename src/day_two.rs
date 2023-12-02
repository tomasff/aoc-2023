use std::cmp::max;

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

fn solve_part_one(input: &str) -> Option<u32> {
    input.lines()
        .map(|game| game.split_once(":").expect("Invalid game!"))
        .map(|(name, runs)| (
                name.split_once(" ").and_then(|(_, id)| id.parse::<u32>().ok()),
                runs.split(";").flat_map(
                    |run| run.split(",").map(|cubes| cubes.trim().split_once(" ").expect("Invalid game"))
                )
                .map(|(count, color)| (count.parse::<u32>().expect("Invalid count"), color))
                .fold((0, 0, 0), |(max_r, max_b, max_g), (count, color)|
                    match color {
                        "red" => (max(max_r, count), max_b, max_g),
                        "green" => (max_r, max_b, max(max_g, count)),
                        "blue" => (max_r, max(max_b, count), max_g),
                        _ => unreachable!()
                    }
                )
            )
        )
        .filter(|(_, c)| c.0 <= RED && c.1 <= BLUE && c.2 <= GREEN)
        .map(|(id, _)| id)
        .sum()
}

pub fn solve(input: &str) -> (Option<u32>, Option<u32>) {
    (solve_part_one(input), None)
}