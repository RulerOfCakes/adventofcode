use std::io::{self, BufRead};

const VALID_DIGITS: [(&str, u32); 9] = [
    ("one", 1u32),
    ("two", 2u32),
    ("three", 3u32),
    ("four", 4u32),
    ("five", 5u32),
    ("six", 6u32),
    ("seven", 7u32),
    ("eight", 8u32),
    ("nine", 9u32),
];

pub fn part2() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut ans = 0;

    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let mut index = 0;

        let line_iter = std::iter::from_fn(move || {
            if index >= line.len() {
                return None;
            }
            let reduced_line = &line[index..];

            let result = VALID_DIGITS
                .iter()
                .find(|(word, _)| reduced_line.starts_with(word))
                .map(|(_, val)| char::from_digit(*val, 10).unwrap())
                .or_else(|| {
                    let result = reduced_line.chars().next();
                    result
                });
            index += 1;
            result
        });

        let mut digit_iter = line_iter.filter_map(|c| c.to_digit(10));
        let first_digit = digit_iter.next().unwrap();

        let last_digit = match digit_iter.last() {
            Some(c) => c,
            None => first_digit,
        };
        let found_number = first_digit * 10 + last_digit;
        ans += found_number;
    }
    print!("{}", ans)
}
