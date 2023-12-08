use std::collections::HashMap;

use num::integer::lcm;

const SINGULAR_ENTRY: &str = "AAA";
const ENTRY_SUFFIX: char = 'A';
const EXIT_SUFFIX: char = 'Z';
const LEFT: char = 'L';
const RIGHT: char = 'R';

fn compute_num_steps_to_end(
    instructions: &str,
    starting_location: &&str,
    graph: &HashMap<&str, (&str, &str)>,
) -> i64 {
    let instructions = instructions.chars().cycle();
    let mut current_location = starting_location;

    for (step_count, step) in instructions.enumerate() {
        if current_location.ends_with(EXIT_SUFFIX) {
            return step_count as i64;
        }

        current_location = graph
            .get(current_location)
            .map(|(left, right)| match step {
                LEFT => left,
                RIGHT => right,
                _ => unreachable!(),
            })
            .expect("Invalid location.");
    }

    // This should not happen
    -1
}

fn parse_graph(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let (instructions, graph) = input.split_once("\n\n").expect("Invalid map!");
    let graph = graph.lines().map(parse_adjacency).collect();

    (instructions, graph)
}

fn parse_adjacency(adjacency: &str) -> (&str, (&str, &str)) {
    adjacency
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
}

fn solve_part_one(instructions: &str, graph: &HashMap<&str, (&str, &str)>) -> i64 {
    compute_num_steps_to_end(instructions, &SINGULAR_ENTRY, graph)
}

fn solve_part_two(instructions: &str, graph: &HashMap<&str, (&str, &str)>) -> i64 {
    let starting_points: Vec<_> = graph
        .keys()
        .filter(|location_name| location_name.ends_with(ENTRY_SUFFIX))
        .collect();

    starting_points
        .iter()
        .map(|&starting_location| compute_num_steps_to_end(instructions, starting_location, graph))
        .reduce(lcm)
        .expect("Failed to calculate steps.")
}

pub fn solve(input: &str) -> (i64, i64) {
    let (instructions, graph) = parse_graph(input);

    (
        solve_part_one(instructions, &graph),
        solve_part_two(instructions, &graph),
    )
}
