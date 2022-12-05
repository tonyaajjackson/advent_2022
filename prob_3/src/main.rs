use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File error");

    let sacks = input.split_terminator("\n");

    let score: i32 = sacks.map(|sack| {priority(sack)}).sum();

    println!("{:?}", score);

}

fn priority(sack: &str) -> i32 {
    let (a, b) = &sack.split_at(sack.len()/2);
    let left = &a.chars().collect::<HashSet<char>>();
    let right = &b.chars().collect::<HashSet<char>>();
    assert!(a.len() == b.len(), "Split error");

    let intersec = left.intersection(&right).collect::<Vec<&char>>();

    assert!(intersec.len() == 1, "Sack: {:?}\nLeft: {:?}\n Right: {:?}\n Intersection: {:?}", sack, left, right, intersec);

    let letter = intersec[0].clone() as i32;

    if letter < 97 { // Lowercase ASCII
        letter - 65 + 26 + 1
    } else {
        letter - 97 + 1
    }
}

#[cfg(test)]
mod tests {
    use crate::priority;

    #[test]
    fn test_priority() {
        assert_eq!(priority("aa"), 1);
        assert_eq!(priority("bb"), 2);
        assert_eq!(priority("zz"), 26);
        assert_eq!(priority("AA"), 27);
        assert_eq!(priority("ZZ"), 52);
    }
}
