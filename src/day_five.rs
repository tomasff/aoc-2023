use std::{collections::HashMap, str::FromStr};

impl FromStr for CategoryMapping {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let section: Vec<_> = s.lines().collect();

        let (from, to) = section[0]
            .split_once(' ')
            .and_then(|(name, _)| name.split_once("-to-"))
            .ok_or(())?;

        let from = from.to_string();
        let to = to.to_string();

        let rules: Vec<Rule> = section[1..]
            .iter()
            .map(|r| r.parse().map_err(|_| ()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(CategoryMapping { from, to, rules })
    }
}

#[derive(Debug)]
struct CategoryMapping {
    from: String,
    to: String,
    rules: Vec<Rule>,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges: Vec<i64> = s
            .split_whitespace()
            .map(|v| v.parse::<i64>().map_err(|_| ()))
            .collect::<Result<Vec<_>, _>>()?;

        let destination_range_start = ranges[0];
        let source_range_start = ranges[1];
        let num_values = ranges[2];

        Ok(Rule {
            source_range_start,
            source_range_end: source_range_start + num_values - 1,
            destination_range_start,
            destination_range_end: destination_range_start + num_values - 1,
        })
    }
}

#[derive(Debug)]
struct Rule {
    source_range_start: i64,
    source_range_end: i64,

    destination_range_start: i64,
    destination_range_end: i64,
}

const DEFAULT_ORDER: &[&str] = &[
    "soil",
    "fertilizer",
    "water",
    "light",
    "temperature",
    "humidity",
    "location",
];

fn solve_part_one(input: &str) -> i64 {
    let sections: Vec<_> = input.split("\n\n").collect();

    let seeds: Vec<i64> = sections[0]
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();

    let maps: HashMap<String, Vec<Rule>> = sections[1..]
        .iter()
        .filter_map(|section| section.parse().ok())
        .map(|c: CategoryMapping| (c.to, c.rules))
        .collect();

    seeds
        .iter()
        .map(|&seed| {
            DEFAULT_ORDER.iter().fold(seed, |current, next_step| {
                maps.get(*next_step)
                    .map(|rules| {
                        rules
                            .iter()
                            .find(|r| {
                                current >= r.source_range_start && current <= r.source_range_end
                            })
                            .map_or(current, |r| {
                                r.destination_range_start + (current - r.source_range_start)
                            })
                    })
                    .expect("Invalid step!")
            })
        })
        .min()
        .unwrap()
}

fn solve_part_two(input: &str) -> i64 {
    let sections: Vec<_> = input.split("\n\n").collect();

    let seeds = sections[0]
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok());

    let seeds: Vec<(i64, i64)> = seeds.clone().zip(seeds.skip(1)).collect();

    let maps: HashMap<String, Vec<Rule>> = sections[1..]
        .iter()
        .filter_map(|section| section.parse().ok())
        .map(|c: CategoryMapping| (c.to, c.rules))
        .collect();

    seeds
        .iter()
        .flat_map(|&(seed_from, range)| {
            (seed_from..seed_from+range).map(|seed| {
                DEFAULT_ORDER.iter().fold(seed, |current, next_step| {
                    maps.get(*next_step)
                        .map(|rules| {
                            rules
                                .iter()
                                .find(|r| {
                                    current >= r.source_range_start && current <= r.source_range_end
                                })
                                .map_or(current, |r| {
                                    r.destination_range_start + (current - r.source_range_start)
                                })
                        })
                        .expect("Invalid step!")
                })
            })
        })
        .min()
        .unwrap()
}

pub fn solve(input: &str) -> (i64, i64) {
    (solve_part_one(input), solve_part_two(input))
}
