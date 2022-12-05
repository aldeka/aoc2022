use std::fs;

fn score_play(play1: &str, play2: &str) -> i32 {
    let rock = 1;
    let paper = 2;
    let scissors = 3;

    let loss = 0;
    let draw = 3;
    let win = 6;
    match play1 {
        // Rock
        "A" => match play2 {
            "X" => rock + draw,     // Rock
            "Y" => paper + win,     // Paper
            "Z" => scissors + loss, // Scissors
            _ => 0,
        },
        // Paper
        "B" => match play2 {
            "Y" => paper + draw,
            "Z" => scissors + win,
            "X" => rock + loss,
            _ => 0,
        },
        // Scissors
        "C" => match play2 {
            "Z" => scissors + draw,
            "X" => rock + win,
            "Y" => paper + loss,
            _ => 0,
        },
        _ => 0,
    }
}

fn main() {
    println!("Hello");

    let contents =
        fs::read_to_string("src/02/input.txt").expect("Should have been able to read the file");

    let lines = contents.split("\n");
    let mut total_score = 0;
    for line in lines {
        if line != "" {
            let mut pieces = line.split(" ");
            let play1 = pieces.nth(0).expect("Play one exists").to_string();
            let play2 = pieces.nth(0).expect("Play two exists").to_string();
            let score = score_play(&play1, &play2);
            total_score += score;
        }
    }
    println!("Final score:\n{total_score}");
}
