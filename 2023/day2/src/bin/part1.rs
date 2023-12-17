use std::io::{self, BufRead};
pub fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut tsum = 0;
    let mut ans = 0;

    for line in lines {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        let games_split = line.split(": ").collect::<Vec<&str>>();

        let game_num = games_split[0].split(' ').collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();

        let games = games_split[1].split("; ").collect::<Vec<&str>>();

        ans += if games.iter().any(|game| {
            game.split(", ").any(|ball_set| {
                let ball_set = ball_set.split(' ').collect::<Vec<&str>>();
                let ball_num = ball_set[0].parse::<i32>().unwrap();
                let ball_color = ball_set[1];

                match ball_color {
                    "red" => ball_num > 12,
                    "green" => ball_num > 13,
                    _ => ball_num > 14,
                }
            })
        }) {
            game_num
        } else {
            0
        };
        tsum += game_num;
    }

    print!("{}", tsum - ans);
}
