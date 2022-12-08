use day2::{to_shapes, Draws, Shape};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "1" => println!("[part1]: total score: {}", part1("input")),
        "2" => println!("[part2]: total score: {}", part2("input")),
        _ => println!("unrecognized option"),
    };
}

fn part1(input: &str) -> i32 {
    let input: String = fs::read_to_string(input).expect("failed reading input file from system");
    let draws = to_shapes(input);
    draws.iter().map(|d| d.0.score(&d.1)).sum::<i32>()
}

fn part2(input: &str) -> i32 {
    let input: String = fs::read_to_string(input).expect("failed reading input file from system");
    let draws: Vec<(char, char)> = input
        .lines()
        .map(|line| {
            (
                line.chars().nth(0).expect("must have first choice"),
                line.chars().nth(2).expect("must have second choice"),
            )
        })
        .collect();

    draws
        .iter()
        .map(|d| {
            let oppo = match Shape::into_shape(d.0) {
                Ok(v) => v,
                Err(e) => panic!("failed converting into shape: {}", e),
            };

            let desired_res = Draws::from_char(d.1);
            let ours = Shape::reverse_shape_finder(&desired_res, &oppo);
            ours.score(&oppo)
        })
        .sum::<i32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn computes_scores_correctly() {
        const ACTUAL: i32 = 15;
        let expected: i32 = part1("test_input");

        assert_eq!(expected, ACTUAL);
    }

    #[test]
    fn computes_rigged_game() {
        const ACTUAL: i32 = 12;
        let expected: i32 = part2("test_input");
        assert_eq!(expected, ACTUAL);
    }
}
