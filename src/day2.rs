fn main() {
    dbg!(count_safe_lines("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"));
}

fn count_safe_lines(input: &str) -> i32 {
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
            .any(|diff| diff < 1 || diff > 3)
        {
            return acc;
        }
        acc + 1
    })
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
        );
        assert_eq!(result, 2);
    }
}
