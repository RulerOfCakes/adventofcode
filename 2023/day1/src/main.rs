mod part1;
mod part2;
fn main() {
    let part = std::env::args().nth(1).expect("no part given");
    if part == "part1" {
        part1::part1();
    } else {
        part2::part2();
    }
}
