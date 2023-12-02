use std::io::{self, BufRead};

pub fn part1() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut ans = 0;

    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let first_digit = line
            .chars()
            .find(|c| c.is_ascii_digit())
            .unwrap()
            .to_digit(10)
            .unwrap();
        let last_digit = line
            .chars()
            .rev()
            .find(|c| c.is_ascii_digit())
            .unwrap()
            .to_digit(10)
            .unwrap();
        let found_number = first_digit * 10 + last_digit;
        ans += found_number;
    }
    print!("{}", ans)
}
