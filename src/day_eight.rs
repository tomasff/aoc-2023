use std::collections::HashMap;

use itertools::{FoldWhile, Itertools};
use num::integer::lcm;

const LEFT: char = 'L';
const RIGHT: char = 'R';

fn solve_part_one(instructions: &str, graph: &HashMap<&str, (&str, &str)>) -> i64 {
    instructions
        .chars()
        .cycle()
        .fold_while(
            (0, "AAA"),
            |(current_steps, current_location), next_step| {
                if current_location == "ZZZ" {
                    FoldWhile::Done((current_steps, current_location))
                } else {
                    FoldWhile::Continue((
                        current_steps + 1,
                        graph
                            .get(current_location)
                            .map(|(left, right)| match next_step {
                                LEFT => left,
                                RIGHT => right,
                                _ => unreachable!(),
                            })
                            .expect("Invalid location."),
                    ))
                }
            },
        )
        .into_inner()
        .0
}

fn solve_part_two(instructions: &str, graph: &HashMap<&str, (&str, &str)>) -> i64 {
    let starting_points: Vec<_> = graph
        .keys()
        .filter(|location_name| location_name.ends_with('A'))
        .collect();

    starting_points
        .iter()
        .map(|&starting_location| {
            instructions
                .chars()
                .cycle()
                .fold_while(
                    (0, starting_location),
                    |(current_steps, current_location), next_step| {
                        if current_location.ends_with('Z') {
                            FoldWhile::Done((current_steps, current_location))
                        } else {
                            FoldWhile::Continue((
                                current_steps + 1,
                                graph
                                    .get(current_location)
                                    .map(|(left, right)| match next_step {
                                        LEFT => left,
                                        RIGHT => right,
                                        _ => unreachable!(),
                                    })
                                    .expect("Invalid location."),
                            ))
                        }
                    },
                )
                .into_inner()
                .0
        })
        .reduce(lcm)
        .expect("Failed to calculate steps.")
}

fn parse_graph(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let (instructions, graph) = input.split_once("\n\n").expect("Invalid map!");

    let graph: HashMap<&str, (&str, &str)> = graph
        .lines()
        .map(|network_mapping| {
            network_mapping
                .split_once(" = ")
                .map(|(location, choices)| {
                    (
                        location,
                        choices
                            .strip_prefix('(')
                            .and_then(|choices| choices.strip_suffix(')'))
                            .and_then(|choices| choices.split_once(", "))
                            .expect("Prasing: Invalid location choices."),
                    )
                })
                .expect("Parsing: Invalid location.")
        })
        .collect();

    (instructions, graph)
}

pub fn solve(input: &str) -> (i64, i64) {
    let (instructions, graph) = parse_graph(input);

    (
        solve_part_one(instructions, &graph),
        solve_part_two(instructions, &graph),
    )
}
