const DIGIT_MAPPING: &'static [(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn recover_calibration(calibration: &str) -> u32 {
    let first_digit = calibration
        .chars()
        .find(char::is_ascii_digit)
        .and_then(|c| c.to_digit(10))
        .expect("Found no valid digits!");
    let last_digit = calibration
        .chars()
        .rfind(char::is_ascii_digit)
        .and_then(|c| c.to_digit(10))
        .expect("Found no valid digits!");

    first_digit * 10 + last_digit
}

fn recover_corrected_calibration(calibration: &str) -> u32 {
    let first_digit = DIGIT_MAPPING
        .iter()
        .filter_map(|(prefix, digit)| calibration.find(prefix).map(|index| (index, digit)))
        .min_by_key(|pair| pair.0)
        .map(|(_, digit)| digit)
        .expect("No valid digits found!");

    let last_digit = DIGIT_MAPPING
        .iter()
        .filter_map(|(prefix, digit)| calibration.rfind(prefix).map(|index| (index, digit)))
        .max_by_key(|pair| pair.0)
        .map(|(_, digit)| digit)
        .expect("No valid digits found!");

    first_digit * 10 + last_digit
}

pub fn solve(input: &String) -> (Option<u32>, Option<u32>) {
    (
        Some(input.lines().map(recover_calibration).sum()),
        Some(input.lines().map(recover_corrected_calibration).sum()),
    )
}
