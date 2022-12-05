use std::env;
use std::fs;
use std::collections::HashSet;
use std::any::type_name;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let input = fs::read_to_string(filename).expect("File error");

    let sacks = input.split_terminator("\n");

    let score: i32 = sacks.map(|sack| {priority(sack)}).sum();

    println!("{:?}", score);

}

fn priority(sack: &str) -> i32 {
    let comp_1 = Vec::from(sack[0..sack.len()/2]);
    let items = HashSet::from(comp_1.clone());

    // for letter in comp_1 {
    //     items.insert(letter.clone());
    // }

    // println!("{:?}", sack);
    // println!("{:?}", comp_1);

    println!("{:?}", items);
    
    0
}


fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
