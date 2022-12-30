use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File error");

    let sacks: Vec<&str> = input.split_terminator("\n").collect();

    let score: i32 = sacks.chunks(3).map(|sack| {priority(sack)}).sum();

    println!("{:?}", score);

}

fn priority(sack: &[&str]) -> i32 {
    let &[a, b, c] = &sack else {
        panic!("Failed to match triplet");
    };

    let a = &a.chars().collect::<HashSet<char>>();
    let b = &b.chars().collect::<HashSet<char>>();
    let c = &c.chars().collect::<HashSet<char>>();

    let intersec = a
        .iter()
        .filter(|k| b.contains(k))
        .filter(|k| c.contains(k))
        .collect::<Vec<&char>>();
        // .intersection(&b)
        // .collect::<HashSet<char>>()
        // .intersection(&c)

    assert!(intersec.len() == 1, "Sack: {:?}\na: {:?}\nb: {:?}\nc: {:?}\n Intersection: {:?}", sack, a, b, c, intersec);

    let letter = intersec[0].clone() as i32;

    if letter < 97 { // Lowercase ASCII
        letter - 65 + 26 + 1
    } else {
        letter - 97 + 1
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::priority;
// 
//     #[test]
//     fn test_priority() {
//         assert_eq!(priority("aa"), 1);
//         assert_eq!(priority("bb"), 2);
//         assert_eq!(priority("zz"), 26);
//         assert_eq!(priority("AA"), 27);
//         assert_eq!(priority("ZZ"), 52);
//     }
// }
