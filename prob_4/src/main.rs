use std::fs;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("File error");

    let assignments: Vec<(Range, Range)> = input
        .split_terminator("\n")
        .map(|a| parse_assignment(a))
        .collect();

    // Part 1
    // let overlaps = assignments
    //     .iter()
    //     .filter(|(a, b)| a.contains(&b) || b.contains(&a))
    //     .count();

    let overlaps = assignments
        .iter()
        .filter(|(a, b)| a.overlaps(&b))
        .count();
    println!("{:?}", overlaps);

}

#[derive(Debug, PartialEq)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn new(start: i32, end: i32) -> Range {
        assert!(start <= end);
        Range {
            start: start,
            end: end,
        }
    }

    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

fn parse_assignment(assignment: &str) -> (Range, Range) {
   let c = RE.captures(assignment).expect("Regex failed to match");

   let left = Range::new(
       c[1].parse::<i32>().unwrap(),
       c[2].parse::<i32>().unwrap(),
   );
   let right = Range::new(
       c[3].parse::<i32>().unwrap(),
       c[4].parse::<i32>().unwrap(),
   );
   (left, right)
}

#[cfg(test)]
mod tests {
    use crate::{Range, parse_assignment};

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_assignment("0-1,10-100"),
            (Range::new(0,1), Range::new(10,100))
        )
    }

    #[test]
    fn test_contains() {
        // True
        assert!(Range::new(0,1).contains(&Range::new(0,1)));
        assert!(Range::new(10,20).contains(&Range::new(15,16)));
    }
}
