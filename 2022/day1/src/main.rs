use std::cmp::Reverse;

use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<Option<u64>>>();
    let groups = input
        .split(|num| num.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .collect::<Vec<u64>>();

    let ans1 = groups.iter().max().unwrap();

    let ans2 = groups
        .iter()
        .map(Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u64>();
    println!("{ans1:?}, {ans2:?}");

    Ok(())
}
