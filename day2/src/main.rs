use std::env;
use std::fs;
use day2::Draws;


fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "1" => part1(),
        // "2" => part2(),
        _ => println!("unrecognized option"),
    };
}

fn part1() {
    let input: String = fs::read_to_string("input").expect("failed reading input file from system");
    let draw: Draws = Draws::from(input.iter());

    let score: i32 = draw.score();
    println!("[part1]: Total score: {}", score);
}
