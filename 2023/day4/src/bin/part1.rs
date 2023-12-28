use nom::{
    bytes::complete::tag,
    character::complete::{i32, line_ending, space0, space1},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn evaluate_card(&self) -> usize {
        self.numbers.iter().fold(0, |acc, n| {
            if self.winning_numbers.contains(n) {
                if acc == 0 {
                    1
                } else {
                    acc * 2
                }
            } else {
                acc
            }
        })
    }
}

fn parse_numbers(s: &str) -> IResult<&str, Vec<usize>> {
    let (s, digits) = delimited(space0, separated_list1(space1, i32), space0)(s)?;

    Ok((s, digits.into_iter().map(|d| d as usize).collect()))
}

fn parse_card(s: &str) -> IResult<&str, Card> {
    let (s, card_id) = preceded(tag("Card"), preceded(space0, i32))(s)?;

    let (s, (winning_numbers, numbers)) = preceded(
        tag(":"),
        separated_pair(parse_numbers, tag("|"), parse_numbers),
    )(s)?;

    Ok((
        s,
        Card {
            id: card_id as usize,
            winning_numbers,
            numbers,
        },
    ))
}

fn parse_cards(s: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, parse_card)(s)
}

fn process(input: &str) -> usize {
    let cards = parse_cards(input).expect("should parse").1;

    cards.iter().fold(0, |acc, card| acc + card.evaluate_card())
}

fn main() {
    let lines = include_str!("../input.txt").lines();

    println!(
        "{}",
        process(lines.collect::<Vec<&str>>().join("\n").as_str())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process(input), 13);
    }
}
