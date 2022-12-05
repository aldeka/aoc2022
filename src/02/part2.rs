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
        "A" => {
            match play2 {
                "X" => return scissors + loss, // loss
                "Y" => return rock + draw,     // draw
                "Z" => return paper + win,     // win
                _ => return 0,
            }
        }
        // Paper
        "B" => {
            match play2 {
                "X" => return rock + loss,    // loss
                "Y" => return paper + draw,   // draw
                "Z" => return scissors + win, // win
                _ => return 0,
            }
        }
        // Scissors
        "C" => {
            match play2 {
                "X" => return paper + loss,    // loss
                "Y" => return scissors + draw, // draw
                "Z" => return rock + win,      // win
                _ => return 0,
            }
        }
        _ => return 0,
    }
}

fn main() {
    println!("Hello");

    let contents =
        fs::read_to_string("src/02/input.txt").expect("Should have been able to read the file");

    let lines = contents.split("\n");
    let mut total_score = 0;
    for line in lines {
        println!("Line: {line}");
        if line != "" {
            let mut pieces = line.split(" ");
            let play1 = pieces.nth(0).expect("Play one exists").to_string();
            let play2 = pieces.nth(0).expect("Play two exists").to_string();
            println!("{play1} {play2}");
            let score = score_play(&play1, &play2);
            println!("Score: {score}");
            total_score += score;
        }
    }
    println!("Final score:\n{total_score}");
}
