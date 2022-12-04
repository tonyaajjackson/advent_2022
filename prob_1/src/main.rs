use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let input = fs::read_to_string(filename).expect("File error");

    let mut calories: Vec<i32> = Vec::new();
    let elves = input.split_terminator("\n\n");

    for elf in elves {
        let meals: Vec<i32> = elf
            .split_terminator("\n")
            .map(|cal_string| {cal_string.parse::<i32>().unwrap()})
           .collect::<Vec<i32>>();

        calories.push(meals.iter().sum());
    }

    calories.sort();   
    println!("{:?}", calories.iter().rev().take(3).sum::<i32>());
}
