use itertools::Itertools;

fn determine_simple_kind(cards: &[i64]) -> i64 {
    let counts = cards
        .iter()
        .counts()
        .values()
        .sorted()
        .copied()
        .collect_vec();

    match counts[..] {
        [5] => 6,
        [1, 4] => 5,
        [2, 3] => 4,
        [1, 1, 3] => 3,
        [1, 2, 2] => 2,
        [1, 1, 1, 2] => 1,
        [1, 1, 1, 1, 1] => 0,
        _ => unreachable!(),
    }
}

fn determine_kind_with_joker(cards: &[i64]) -> i64 {
    let (cards, jokers): (Vec<i64>, Vec<i64>) = cards.iter().partition(|c| **c != 0);

    let mut counts = cards
        .iter()
        .counts()
        .values()
        .sorted()
        .copied()
        .collect_vec();

    let num_jokers = jokers.len();

    if let Some(max) = counts.last_mut() {
        *max += num_jokers;
    } else {
        counts.push(num_jokers);
    }

    match counts[..] {
        [5] => 6,
        [1, 4] => 5,
        [2, 3] => 4,
        [1, 1, 3] => 3,
        [1, 2, 2] => 2,
        [1, 1, 1, 2] => 1,
        [1, 1, 1, 1, 1] => 0,
        _ => unreachable!(),
    }
}

fn parse_cards(cards: &str) -> Vec<i64> {
    cards
        .chars()
        .map(|c| {
            match c {
                '2'..='9' => c.to_digit(10).unwrap(),
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!(),
            }
            .into()
        })
        .collect()
}

fn parse_cards_with_joker(cards: &str) -> Vec<i64> {
    cards
        .chars()
        .map(|c| {
            match c {
                'J' => 0,
                '2'..='9' => c.to_digit(10).unwrap(),
                'T' => 10,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!(),
            }
            .into()
        })
        .collect()
}

fn solve_part_one(input: &str) -> i64 {
    input
        .lines()
        .filter_map(|line| {
            line.split_once(' ').map(|(cards, bid)| {
                let cards = parse_cards(cards);
                (
                    determine_simple_kind(&cards),
                    cards,
                    bid.parse::<i64>().ok().unwrap(),
                )
            })
        })
        .sorted()
        .enumerate()
        .map(|(rank, (_, _, bid))| (rank as i64 + 1) * bid)
        .sum()
}

fn solve_part_two(input: &str) -> i64 {
    input
        .lines()
        .filter_map(|line| {
            line.split_once(' ').map(|(cards, bid)| {
                let cards = parse_cards_with_joker(cards);
                (
                    determine_kind_with_joker(&cards),
                    cards,
                    bid.parse::<i64>().ok().unwrap(),
                )
            })
        })
        .sorted()
        .enumerate()
        .map(|(rank, (_, _, bid))| (rank as i64 + 1) * bid)
        .sum()
}

pub fn solve(input: &str) -> (i64, i64) {
    (solve_part_one(input), solve_part_two(input))
}
