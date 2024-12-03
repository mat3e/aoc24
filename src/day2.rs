fn main() {
    dbg!(count_safe_lines_tolerating(to_lines_numbers(
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
    )));
}

fn count_safe_lines_tolerating(lines_of_numbers: Vec<Numbers>) -> usize {
    lines_of_numbers.iter().fold(
        0,
        |acc, numbers| {
            if valid_line(numbers) {
                return acc + 1
            }
            for i in 0..numbers.len() {
                let mut numbers = numbers.clone();
                numbers.remove(i);
                if valid_line(&numbers) {
                    return acc + 1
                }
            }
            acc
        },
    )
}

fn count_safe_lines(lines_of_numbers: Vec<Numbers>) -> usize {
    lines_of_numbers
        .iter()
        .filter(|numbers| valid_line(numbers))
        .count()
}

fn valid_line(numbers: &Numbers) -> bool {
    if !numbers.is_sorted() && !numbers.is_sorted_by(|a, b| b < a) {
        return false;
    }
    if numbers
        .windows(2)
        .map(|pair| (pair[0] - pair[1]).abs())
        .any(|diff| !(1..=3).contains(&diff))
    {
        return false;
    }
    true
}

fn to_lines_numbers(input: &str) -> Vec<Numbers> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Numbers>()
        })
        .collect()
}

type Numbers = Vec<i32>;

mod tests {
    use super::*;

    #[test]
    fn resolves_first() {
        let result = count_safe_lines(to_lines_numbers(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        ));
        assert_eq!(result, 2);
    }

    #[test]
    fn resolves_second() {
        let result = count_safe_lines_tolerating(to_lines_numbers(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        ));
        assert_eq!(result, 4);
    }
}
