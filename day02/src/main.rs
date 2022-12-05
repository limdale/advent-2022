use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

const WIN: i32 = 1;
const DRAW: i32 = 0;
const LOSE: i32 = -1;

#[derive(Hash, Eq, PartialEq, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let lines = BufReader::new(file).lines();
    let lines_vec: Vec<String> = lines.map(|s| s.unwrap()).collect();

    let player_map = HashMap::from([
        ("X", &Choice::Rock),
        ("Y", &Choice::Paper),
        ("Z", &Choice::Scissors)
    ]);

    let opponent_map = HashMap::from([
        ("A", &Choice::Rock),
        ("B", &Choice::Paper),
        ("C", &Choice::Scissors)
    ]);

    let points_map = HashMap::from([
        (&Choice::Rock, 1),
        (&Choice::Paper, 2),
        (&Choice::Scissors, 3)
    ]);

    let lose_map = HashMap::from([
        (&Choice::Rock, &Choice::Paper),
        (&Choice::Paper, &Choice::Scissors),
        (&Choice::Scissors, &Choice::Rock)
    ]);

    let win_map = HashMap::from([
        (&Choice::Rock, &Choice::Scissors),
        (&Choice::Paper, &Choice::Rock),
        (&Choice::Scissors, &Choice::Paper)
    ]);

    let result_map = HashMap::from([
        ("X", LOSE),
        ("Y", DRAW),
        ("Z", WIN)
    ]);

    let mut total_points_1 = 0;
    let mut total_points_2 = 0;
    for line in lines_vec.iter() {
        let game: Vec<&str> = line.trim().split(" ").collect();
 
        // Calculate points for problem 1
        let player = &player_map[game[1]];
        let opponent = &opponent_map[game[0]];
        let result = if player == opponent {
            DRAW
        } else if &win_map[player] == opponent {
            WIN
        } else {
            LOSE
        };
        total_points_1 += (result + 1) * 3 + points_map[player];


        // Calculate points for problem 2
        let expected_result = result_map[game[1]];
        let player_2 = if expected_result == LOSE {
            win_map[opponent]
        } else if expected_result == WIN {
            lose_map[opponent]
        } else {
            opponent
        };

        total_points_2 += (expected_result + 1) * 3 + points_map[player_2]
        
    }

    println!("Total 1: {}", total_points_1);
    println!("Total 2: {}", total_points_2);

    Ok(())
}
