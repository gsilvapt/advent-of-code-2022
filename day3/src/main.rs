use std::env;
use std::fs;

use day3::{score_in_3_chunks, score_uniques};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "1" => println!("[part1]: total score: {}", part1("input")),
        "2" => println!("[part2]: total score: {}", part2("input")),
        _ => println!("unrecognized option"),
    };
}

fn part1(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).expect("failed reading file from system");
    match score_uniques(input.as_str()) {
        Some(v) => v,
        None => panic!("failed to score uniques"),
    }
}

fn part2(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).expect("failed reading file from system");
    match score_in_3_chunks(input.as_str()) {
        Some(v) => v,
        None => panic!("failed to score uniques"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn computes_score_with_test_input() {
        const ACTUAL: i32 = 157;
        let expected: i32 = part1("test_input");
        assert_eq!(expected, ACTUAL);
    }

    #[test]
    fn computes_chunks_with_test_input() {
        const ACTUAL: i32 = 70;
        let expected: i32 = part2("test_input");
        assert_eq!(expected, ACTUAL);
    }
}
