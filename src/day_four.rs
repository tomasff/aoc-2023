fn solve_part_one(input: &str) -> i64 {
    input
        .lines()
        .filter_map(|card| {
            card.split_once(':')
                .and_then(|(_, numbers)| numbers.split_once('|'))
                .map(|(winning, have)| {
                    winning
                        .split_whitespace()
                        .filter(|w| have.split_whitespace().any(|h| *w == h))
                        .count()
                })
                .filter(|c| *c > 0)
                .map(|count| i64::pow(2, (count - 1) as u32))
        })
        .sum()
}

fn solve_part_two(input: &str) -> i64 {
    let cards: Vec<i64> = input
        .lines()
        .filter_map(|card| {
            card.split_once(':')
                .and_then(|(_, numbers)| numbers.split_once('|'))
                .map(|(winning, have)| {
                    winning
                        .split_whitespace()
                        .filter(|w| have.split_whitespace().any(|h| *w == h))
                        .count() as i64
                })
        })
        .collect();

    let mut card_counts = vec![1; cards.len()];
    let mut total_cards = 0;

    for (card_number, &matches) in cards.iter().enumerate() {
        let card_count = card_counts[card_number];
        total_cards += card_count;

        for card_offset in 1..=matches {
            if let Some(current_count) = card_counts.get_mut(card_number + card_offset as usize) {
                *current_count += card_count;
            }
        }
    }

    total_cards
}

pub fn solve(input: &str) -> (i64, i64) {
    (solve_part_one(input), solve_part_two(input))
}
