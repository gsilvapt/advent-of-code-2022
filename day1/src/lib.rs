pub fn split_calorie_group(input: &str) -> Vec<i32> {
    input
        .split("\n\n")
        .map(|cal| {
            let mut sum = 0;
            cal.lines().for_each(|l| {
                let num = l.parse::<i32>().unwrap();
                sum += num;
            });
            sum
        })
        .collect::<Vec<i32>>()
}

pub fn compute_most_calories(input: Vec<i32>) -> i32 {
    *input.iter().max().unwrap()
}

pub fn compute_top_three(input: &mut Vec<i32>) -> Vec<i32> {
    input.sort_by(|a, b| b.cmp(a));
    vec![input[0], input[1], input[2]]
}
