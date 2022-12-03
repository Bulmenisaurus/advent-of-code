enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

enum RoundResult {
    Win,
    Lose,
    Tie,
}

fn get_choice(opponent_choice: RPSChoice, our_result: RoundResult) -> RPSChoice {
    let score_win = 6;
    let score_tie = 3;
    let score_lose = 0;

    match our_result {
        RoundResult::Win => match opponent_choice {
            RPSChoice::Rock => RPSChoice::Paper,
            RPSChoice::Paper => RPSChoice::Scissors,
            RPSChoice::Scissors => RPSChoice::Rock,
        },
        RoundResult::Lose => match opponent_choice {
            RPSChoice::Rock => RPSChoice::Scissors,
            RPSChoice::Paper => RPSChoice::Rock,
            RPSChoice::Scissors => RPSChoice::Paper,
        },
        RoundResult::Tie => opponent_choice,
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

            let our_result = match parsed[1] {
                "X" => RoundResult::Lose,
                "Y" => RoundResult::Tie,
                "Z" => RoundResult::Win,
                _ => unreachable!(),
            };

            let result_score = match our_result {
                RoundResult::Win => 6,
                RoundResult::Tie => 3,
                RoundResult::Lose => 0,
            };

            let our_choice = get_choice(opp_choice, our_result);
            let choice_score = match our_choice {
                RPSChoice::Rock => 1,
                RPSChoice::Paper => 2,
                RPSChoice::Scissors => 3,
            };

            result_score + choice_score
        })
        .sum();
    println!("{}", res);
}
