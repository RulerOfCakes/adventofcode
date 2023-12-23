use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
enum Value {
    Symbol,
    Empty,
    Number(usize),
}

fn process(input: &str) -> usize {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().map(move |(j, c)| {
                (
                    (i + 1, j + 1),
                    match c {
                        '.' => Value::Empty,
                        c if c.is_ascii_digit() => Value::Number(
                            c.to_digit(10).expect("should be a valid ascii digit") as usize,
                        ),
                        _ => Value::Symbol,
                    },
                )
            })
        })
        .collect::<BTreeMap<(usize, usize), Value>>();

    let mut numbers: Vec<Vec<((usize, usize), usize)>> = vec![];
    map.iter().for_each(|((x, y), v)| {
        if let Value::Number(n) = v {
            if let Some(nv) = numbers.last_mut() {
                if let Some(((last_x, last_y), _)) = nv.last() {
                    if *last_x == *x && *last_y + 1 == *y {
                        nv.push(((*x, *y), *n));
                    } else {
                        numbers.push(vec![((*x, *y), *n)]);
                    }
                } else {
                    nv.push(((*x, *y), *n));
                }
            } else {
                numbers.push(vec![((*x, *y), *n)]);
            }
        }
    });

    numbers.iter().fold(0, |acc, nv| {
        let mut current_number = 0;
        let mut adjacent_symbol = false;

        nv.iter().for_each(|((x, y), n)| {
            let directions = [
                (-1, 0),
                (1, 0),
                (0, -1),
                (0, 1),
                (-1, -1),
                (-1, 1),
                (1, -1),
                (1, 1),
            ];

            adjacent_symbol |= directions.iter().any(|(dx, dy)| {
                let pos = ((*x as isize + dx) as usize, (*y as isize + dy) as usize);
                let value = map.get(&pos);
                matches!(value, Some(Value::Symbol))
            });

            current_number = current_number * 10 + n;
        });

        if adjacent_symbol {
            acc + current_number
        } else {
            acc
        }
    })
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
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(process(input), 4361);
    }

    #[test]
    fn test_weird() {
        let input = "......
..755.
$.....";
        assert_eq!(process(input), 0);
    }
}
