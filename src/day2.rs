fn main() {
    dbg!(count_safe_lines(
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        failure_condition
    ));
}

fn count_safe_lines(input: &str, failure_condition: fn(i32) -> bool) -> i32 {
    input.lines().fold(0, |acc, line| {
        let numbers = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if !numbers.is_sorted() && !numbers.is_sorted_by(|a, b| b < a) {
            return acc;
        }
        if numbers
            .windows(2)
            .map(|pair| (pair[0] - pair[1]).abs())
            .any(failure_condition)
        {
            return acc;
        }
        acc + 1
    })
}

fn failure_condition(diff: i32) -> bool {
    diff < 1 || diff > 3
}

mod tests {

    use super::*;

    #[test]
    fn resolves_first() {
        let result = count_safe_lines(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
            failure_condition,
        );
        assert_eq!(result, 2);
    }
}
