use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const EIGHT_NEIGHBORS: &[(i64, i64)] = &[
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, -1),
    (-1, 1),
    (1, 1),
    (-1, -1),
];

#[derive(Debug)]
enum Cell {
    Symbol,
    Number(usize),
}

fn parse_number(buffer: &[i64]) -> i64 {
    buffer.iter().fold(0, |acc, elem| acc * 10 + elem)
}

fn parse_map(input: &str) -> (Vec<i64>, HashMap<(i64, i64), Cell>) {
    let mut number_buffer = vec![];
    let mut numbers = vec![];
    let mut locations = HashMap::new();

    for (line_number, line) in input.lines().enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            let coordinates = (line_number as i64, char_index as i64);

            if char.is_ascii_digit() {
                number_buffer.push(char.to_digit(10).unwrap().into());

                locations.insert(coordinates, Cell::Number(numbers.len()));
            } else {
                if !number_buffer.is_empty() {
                    numbers.push(parse_number(&number_buffer));
                    number_buffer.clear();
                }

                if char != '.' {
                    locations.insert(coordinates, Cell::Symbol);
                }
            }
        }

        if !number_buffer.is_empty() {
            numbers.push(parse_number(&number_buffer));
            number_buffer.clear();
        }
    }
    (numbers, locations)
}

fn solve_part_one(input: &str) -> i64 {
    let (numbers, locations) = parse_map(input);

    locations
        .iter()
        .filter(|(_, cell)| matches!(cell, Cell::Number(_)))
        .filter(|((x, y), _)| {
            EIGHT_NEIGHBORS
                .iter()
                .any(|(dx, dy)| matches!(locations.get(&(x + dx, y + dy)), Some(Cell::Symbol)))
        })
        .filter_map(|(_, c)| match c {
            Cell::Number(v) => Some(v),
            _ => None,
        })
        .unique()
        .map(|v| numbers[*v])
        .sum()
}

fn solve_part_two(input: &str) -> i64 {
    let (numbers, locations) = parse_map(input);

    locations
        .iter()
        .filter(|(_, v)| matches!(v, Cell::Symbol))
        .map(|((x, y), _)| {
            EIGHT_NEIGHBORS
                .iter()
                .filter_map(|(dx, dy)| match locations.get(&(x + dx, y + dy)) {
                    Some(Cell::Number(i)) => Some(*i),
                    _ => None,
                })
                .collect::<HashSet<_>>()
        })
        .filter(|n| n.len() == 2)
        .map(|v| v.iter().map(|i| numbers[*i]).product::<i64>())
        .sum()
}

pub fn solve(input: &str) -> (i64, i64) {
    (solve_part_one(input), solve_part_two(input))
}
