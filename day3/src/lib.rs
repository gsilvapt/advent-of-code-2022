// score_uniques takes a vector of str slices and splits into two compartments to then find the
// common chars in each.
// The compartment split is done at half the lenght of the string.
pub fn score_uniques(input: &str) -> Option<i32> {
    let lines: Vec<&str> = input.lines().collect();
    let score = lines
        .iter()
        .filter_map(|l| {
            let parts = l.split_at(l.len() / 2);
            let first_cmp = parts.0.as_bytes();
            let second_cmp = parts.1.as_bytes();
            first_cmp
                .iter()
                .find(|byte| second_cmp.contains(byte))
                .map(|&byte| score_char(byte) as i32)
        })
        .sum();
    Some(score)
}

// score_char does some hacky hacks to score a char, where a..z is 1..26 and A..Z is 27..52.
// Does math with ascii codes.
fn score_char(code: u8) -> u8 {
    code % 32 + (26 * (code <= 90) as u8)
}

pub fn score_in_3_chunks(input: &str) -> Option<i32> {
    let lines: Vec<&str> = input.lines().collect();
    let score = lines
        .chunks(3)
        .filter_map(|chunks| {
            let mut chunks = chunks.iter();
            let first = chunks.next()?.as_bytes();
            let second = chunks.next()?.as_bytes();
            let third = chunks.next()?.as_bytes();
            first
                .iter()
                .find(|byte| second.contains(byte) && third.contains(byte))
                .map(|&byte| score_char(byte) as i32)
        })
        .sum();
    Some(score)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn score_uniques_correctly() {
        const ACTUAL: i32 = 157;
        let input: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let expected = score_uniques(input);
        assert_eq!(expected.unwrap(), ACTUAL);
    }

    #[test]
    fn score_chunks_correctly() {
        const ACTUAL: i32 = 70;

        let input: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let expected = score_in_3_chunks(input).unwrap();
        assert_eq!(expected, ACTUAL);
    }
}
