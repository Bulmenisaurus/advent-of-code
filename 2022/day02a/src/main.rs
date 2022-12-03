enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

fn get_match_result(our_choice: RPSChoice, opponent_choice: RPSChoice) -> i32 {
    let score_win = 6;
    let score_tie = 3;
    let score_lose = 0;

    match our_choice {
        RPSChoice::Rock => match opponent_choice {
            RPSChoice::Rock => score_tie,
            RPSChoice::Paper => score_lose,
            RPSChoice::Scissors => score_win,
        },
        RPSChoice::Paper => match opponent_choice {
            RPSChoice::Rock => score_win,
            RPSChoice::Paper => score_tie,
            RPSChoice::Scissors => score_lose,
        },
        RPSChoice::Scissors => match opponent_choice {
            RPSChoice::Rock => score_lose,
            RPSChoice::Paper => score_win,
            RPSChoice::Scissors => score_tie,
        },
    }
}

fn main() {
    let res: i32 = include_str!("./input.txt")
        .split("\n")
        .map(|i| {
            let parsed: Vec<&str> = i.split(" ").collect();

            let opp_choice = match parsed[0] {
                "A" => RPSChoice::Rock,
                "B" => RPSChoice::Paper,
                "C" => RPSChoice::Scissors,
                _ => unreachable!(),
            };

            let our_choice = match parsed[1] {
                "X" => RPSChoice::Rock,
                "Y" => RPSChoice::Paper,
                "Z" => RPSChoice::Scissors,
                _ => unreachable!(),
            };

            (match our_choice {
                RPSChoice::Rock => 1,
                RPSChoice::Paper => 2,
                RPSChoice::Scissors => 3,
            } + get_match_result(our_choice, opp_choice))
        })
        .sum();
    println!("{}", res);
}
