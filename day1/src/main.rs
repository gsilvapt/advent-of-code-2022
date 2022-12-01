use std::env;
use std::fs;

use day1::compute_top_three;
use day1::{compute_most_calories, split_calorie_group};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "1" => part1(),
        "2" => part2(),
        _ => println!("unrecognized option"),
    };
}

fn part2() {
    let input: String = fs::read_to_string("input").expect("failed reading input file from system");
    let mut calories: Vec<i32> = split_calorie_group(&input);
    let top_three: Vec<i32> = compute_top_three(&mut calories);
    let sum: i32 = top_three.into_iter().sum();
    println!("[part2]: Most calories from top 3: {}", sum);
}

fn part1() {
    let input: String = fs::read_to_string("input").expect("failed reading input file from system");
    let calories: Vec<i32> = split_calorie_group(&input);
    let most_cal: i32 = compute_most_calories(calories);
    println!("[part1]: Most calories: {}", most_cal);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_most_calories_correctly() {
        const ACTUAL: i32 = 24000;

        let test_input: String =
            fs::read_to_string("test_input").expect("failed reading input file from system");
        let calories: Vec<i32> = split_calorie_group(&test_input);
        let expected = compute_most_calories(calories);
        assert_eq!(expected, ACTUAL);
    }

    #[test]
    fn computes_top_three_correctly() {
        const ACTUAL_TOP_THREE: [i32; 3] = [24000, 11000, 10000];
        const ACTUAL_SUM: i32 = 45000;

        let test_input: String =
            fs::read_to_string("test_input").expect("failed reading input file from system");
        let mut calories: Vec<i32> = split_calorie_group(&test_input);
        let top_three = compute_top_three(&mut calories);
        assert_eq!(top_three, ACTUAL_TOP_THREE);

        let expected: i32 = top_three.iter().sum();
        assert_eq!(expected, ACTUAL_SUM);
    }
}
