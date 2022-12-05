use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let input = fs::read_to_string(filename).expect("File error");

    let hands = input.split_terminator("\n");

    let score: i32 = hands.map(|hand| {rps_score(hand)}).sum();

    println!("{:?}", score);

}

fn rps_score(hand: &str) -> i32 {
    let opponent = match &hand.chars().nth(0).unwrap() {
        'A' => 0, // Rock
        'B' => 1, // Paper
        'C' => 2, // Scissors
        _ => panic!("Failed to match")
    };

    let played = match &hand.chars().nth(2).unwrap() {
        'X' => 0,
        'Y' => 1,
        'Z' => 2,
        _ => panic!("Failed to match")
    };

    let outcome = match ((played - opponent) as i32).rem_euclid(3) {
        0 => 3, // Draw
        1 => 6, // Win
        2 => 0, // Lose
        _ => panic!("Math error")
    };
    outcome + played + 1
}
