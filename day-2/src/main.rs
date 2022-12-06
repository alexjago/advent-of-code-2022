use std::io;

/// Rock, Paper, Scissors
fn main() -> io::Result<()> {
    let lines = io::stdin().lines();

    let mut score_a = 0;
    let mut score_b = 0;

    for line in lines {
        let l = line?;
        let moves: Vec<&str> = l.split_whitespace().take(2).collect();
        score_a += part_a(translate_moves(moves[0]), translate_moves(moves[1]));
        score_b += part_b(translate_moves(moves[0]), moves[1]);
    }

    // Part A: return
    println!("Part A: {}", score_a);
    // Part B: return
    println!("Part A: {}", score_b);

    Ok(())
}

fn translate_moves(play: &str) -> &'static str {
    match play {
        "A" | "X" => "R",
        "B" | "Y" => "P",
        "C" | "Z" => "S",
        _ => "",
    }
}
/// Return the opposition move which `play` beats
fn beats(play: &str) -> &'static str {
    match play {
        "R" => "S",
        "S" => "P",
        "P" => "R",
        _ => "",
    }
}
/// Return the opposition move which `play` loses to
fn loses(play: &str) -> &'static str {
    match play {
        "S" => "R",
        "P" => "S",
        "R" => "P",
        _ => "",
    }
}

/// Scores for winning move
fn scores(play: &str) -> i32 {
    match play {
        "R" => 1,
        "P" => 2,
        "S" => 3,
        _ => 0,
    }
}

// Calculate the score for the result of the round
fn round_score(oppo: &str, play: &str) -> i32 {
    if oppo == beats(play) {
        6
    } else if oppo == play {
        3
    } else {
        0
    }
}

/// We assume that ABC and XYZ both equal RPS
/// and that
fn part_a(oppo: &str, play: &str) -> i32 {
    scores(play) + round_score(oppo, play)
}

fn part_b(oppo: &str, game: &str) -> i32 {
    let play = match game {
        "X" => {
            // need to lose
            // play whatever oppo would beat
            beats(oppo)
        }
        "Y" => {
            // need to draw
            oppo
        }
        _ => {
            // Need to win
            // play whatever oppo would lose to
            loses(oppo)
        }
    };

    scores(play) + round_score(oppo, play)
}
