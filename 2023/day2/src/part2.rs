use std::io::{self, BufRead};
pub fn part2() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut ans = 0;

    for line in lines {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        let games_split = line.split(": ").collect::<Vec<&str>>();

        let games = games_split[1].split("; ").collect::<Vec<&str>>();
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        games.iter().for_each(|game| {
            game.split(", ").for_each(|ball_set| {
                let ball_set = ball_set.split(' ').collect::<Vec<&str>>();
                let ball_num = ball_set[0].parse::<i64>().unwrap();
                let ball_color = ball_set[1];

                match ball_color {
                    "red" => min_red = min_red.max(ball_num),
                    "green" => min_green = min_green.max(ball_num),
                    _ => min_blue = min_blue.max(ball_num),
                }
            });
        });
        ans += min_red * min_green * min_blue;
    }

    print!("{}", ans);
}
