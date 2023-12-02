use std::cmp::max;

const MAX_ALLOWED_RED: u32 = 12;
const MAX_ALLOWED_GREEN: u32 = 13;
const MAX_ALLOWED_BLUE: u32 = 14;

fn parse_games<'input>(
    input: &'input str,
) -> impl Iterator<Item = (u32, (u32, u32, u32))> + 'input {
    input
        .lines()
        .map(|game| game.split_once(":").expect("Invalid game."))
        .map(|(name, runs)| {
            (
                name.split_once(" ")
                    .and_then(|(_, id)| id.parse().ok())
                    .expect("Invalid game ID!"),
                runs.split(";")
                    .flat_map(|run| {
                        run.split(",")
                            .map(|cubes| cubes.trim().split_once(" ").expect("Invalid game runs."))
                    })
                    .map(|(count, color)| (count.parse().expect("Invalid cube count."), color))
                    .fold(
                        (0, 0, 0),
                        |(max_r, max_g, max_b), (count, color)| match color {
                            "red" => (max(max_r, count), max_g, max_b),
                            "green" => (max_r, max(max_g, count), max_b),
                            "blue" => (max_r, max_g, max(max_b, count)),
                            _ => unreachable!(),
                        },
                    ),
            )
        })
}

fn solve_part_one(input: &str) -> u32 {
    parse_games(input)
        .filter(|(_, (r, g, b))| {
            *r <= MAX_ALLOWED_RED && *g <= MAX_ALLOWED_GREEN && *b <= MAX_ALLOWED_BLUE
        })
        .map(|(id, _)| id)
        .sum()
}

fn solve_part_two(input: &str) -> u32 {
    parse_games(input).map(|(_, (r, g, b))| r * g * b).sum()
}

pub fn solve(input: &str) -> (Option<u32>, Option<u32>) {
    (Some(solve_part_one(input)), Some(solve_part_two(input)))
}
