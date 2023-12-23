use std::collections::{BTreeMap, HashSet};

#[derive(Debug, PartialEq)]
enum Value {
    Gear,
    Empty,
    Number(usize),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinates {
    x: usize,
    y: usize,
}

struct GearInfo {
    ratio: usize,
    count: usize,
}

fn process(input: &str) -> usize {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().map(move |(j, c)| {
                (
                    Coordinates { x: i + 1, y: j + 1 },
                    match c {
                        '*' => Value::Gear,
                        c if c.is_ascii_digit() => Value::Number(
                            c.to_digit(10).expect("should be a valid ascii digit") as usize,
                        ),
                        _ => Value::Empty,
                    },
                )
            })
        })
        .collect::<BTreeMap<Coordinates, Value>>();

    let mut gears: BTreeMap<Coordinates, GearInfo> = BTreeMap::new();
    let mut numbers: Vec<Vec<(Coordinates, usize)>> = vec![];
    map.iter().for_each(|(Coordinates { x, y }, v)| {
        if let Value::Number(n) = v {
            if let Some(nv) = numbers.last_mut() {
                if let Some((
                    Coordinates {
                        x: last_x,
                        y: last_y,
                    },
                    _,
                )) = nv.last()
                {
                    if *last_x == *x && *last_y + 1 == *y {
                        nv.push((Coordinates { x: *x, y: *y }, *n));
                    } else {
                        numbers.push(vec![(Coordinates { x: *x, y: *y }, *n)]);
                    }
                } else {
                    nv.push((Coordinates { x: *x, y: *y }, *n));
                }
            } else {
                numbers.push(vec![(Coordinates { x: *x, y: *y }, *n)]);
            }
        }
    });

    numbers.iter().for_each(|nv| {
        let mut current_number = 0;

        let mut gear_positions = HashSet::new();

        nv.iter().for_each(|(Coordinates { x, y }, n)| {
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

            directions.iter().for_each(|(dx, dy)| {
                let pos = Coordinates {
                    x: (*x as isize + dx) as usize,
                    y: (*y as isize + dy) as usize,
                };
                let value = map.get(&pos);
                if let Some(Value::Gear) = value {
                    gear_positions.insert(pos);
                }
            });

            current_number = current_number * 10 + n;
        });

        gear_positions.iter().for_each(|Coordinates { x, y }| {
            let gear_info = gears
                .entry(Coordinates { x: *x, y: *y })
                .or_insert(GearInfo { ratio: 1, count: 0 });
            if gear_info.count < 2 {
                gear_info.ratio *= current_number;
            } else {
                gear_info.ratio = 0;
            }
            gear_info.count += 1;
        });
    });

    gears.iter().fold(0, |acc, (_, gear_info)| {
        if gear_info.count == 2 {
            acc + gear_info.ratio
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
        assert_eq!(process(input), 467835);
    }

    #[test]
    fn test_bad() {
        let input = "......
..755.
*.....";
        assert_eq!(process(input), 0);
    }

    #[test]
    fn test_bad_one() {
        let input = "......
..755.
.*....";
        assert_eq!(process(input), 0);
    }
}
