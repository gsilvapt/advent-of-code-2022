const LINES_MUST_BE_SPLIT_COMMA: &str = "input must be split by line";
const PAIRS_MUST_BE_SPLIT_DASH: &str = "pairs must be split by dash";

pub fn compute_overlap_pairs(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(",").expect(LINES_MUST_BE_SPLIT_COMMA);
            let ((left_min, left_max), (right_min, right_max)) = (
                left.split_once("-").expect(PAIRS_MUST_BE_SPLIT_DASH),
                right.split_once("-").expect(PAIRS_MUST_BE_SPLIT_DASH),
            );
            (
                left_min.parse::<u32>().unwrap(),
                left_max.parse::<u32>().unwrap(),
                right_min.parse::<u32>().unwrap(),
                right_max.parse::<u32>().unwrap(),
            )
        })
        .filter(|(left_min, left_max, right_min, right_max)| {
            left_min >= right_min && left_max <= right_max
                || left_min <= right_min && left_max >= right_max
        })
        .count() as u32
}

pub fn compute_all_overlaps(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(",").expect(LINES_MUST_BE_SPLIT_COMMA);
            let ((left_min, left_max), (right_min, right_max)) = (
                left.split_once("-").expect(PAIRS_MUST_BE_SPLIT_DASH),
                right.split_once("-").expect(PAIRS_MUST_BE_SPLIT_DASH),
            );
            (
                left_min.parse::<u32>().unwrap(),
                left_max.parse::<u32>().unwrap(),
                right_min.parse::<u32>().unwrap(),
                right_max.parse::<u32>().unwrap(),
            )
        })
        .filter(|(left_min, left_max, right_min, right_max)| {
            left_min >= right_min && left_max <= right_max
                || left_max >= right_min && left_max <= right_max
                || left_min <= right_min && left_min >= right_max
                || left_min <= right_max && left_max >= right_min
        })
        .count() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn computes_pairs_correctly() {
        const ACTUAL: u32 = 2;
        let input: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let expected = compute_overlap_pairs(input);
        assert_eq!(expected, ACTUAL);
    }
    
    #[test]
    fn computes_non_overlapping_correctly() {
        const ACTUAL: u32 = 4;
        let input: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let expected = compute_all_overlaps(input);
        assert_eq!(expected, ACTUAL);
    }
}
