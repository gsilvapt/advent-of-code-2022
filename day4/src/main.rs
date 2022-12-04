use std::{env, fs};

use day4::{compute_overlap_pairs, compute_all_overlaps};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "1" => println!("[part1]: total score: {}", part1("input")),
        "2" => println!("[part2]: total score: {}", part2("input")),
        _ => println!("unrecognized option"),
    };
}

fn part1(file_path: &str) -> u32 {
    let input = fs::read_to_string(file_path).expect("failed reading file from system");
    compute_overlap_pairs(input.as_str())
}

fn part2(file_path: &str) -> u32 {
    let input = fs::read_to_string(file_path).expect("failed reading file from system");
    compute_all_overlaps(input.as_str())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn computes_overlapping_pairs_correctly() {
        const ACTUAL: u32 = 2;
        let expected = part1("test_input");
        assert_eq!(expected, ACTUAL);
    }
}
