use std::collections::HashMap;

fn main() {
    println!(
        "{}",
        resolve_second(
            "3   4
                    4   3
                    2   5
                    1   3
                    3   9
                    3   3"
        )
    );
}

fn resolve_second(input: &str) -> i32 {
    let (left, right) = split_left_and_right(input);
    let num_to_occurrences = count_occurrences(right);
    let mut result = 0;
    for x in left {
        result += num_to_occurrences.get(&x).unwrap_or(&0) * x;
    }
    result
}

fn count_occurrences(vector: Vec<i32>) -> HashMap<i32, i32> {
    vector.iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(*x).or_insert(0) += 1;
        acc
    })
}

fn resolve_first(input: &str) -> i64 {
    let (mut left, mut right) = split_left_and_right(input);
    left.sort();
    right.sort();
    let mut result = 0;
    for i in 0..left.len() {
        result += (left[i] - right[i]).abs() as i64;
    }
    result
}

fn split_left_and_right(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    input.split_whitespace().enumerate().for_each(|(i, x)| {
        if i % 2 == 0 {
            left.push(x.parse::<i32>().unwrap());
            return;
        }
        right.push(x.parse::<i32>().unwrap());
    });
    (left, right)
}

mod tests {
    use super::*;

    #[test]
    fn resolves() {
        let result = resolve_first(
            "3   4
                    4   3
                    2   5
                    1   3
                    3   9
                    3   3",
        );
        assert_eq!(result, 11);
    }

    #[test]
    fn resolves_second() {
        let result = resolve_second(
            "3   4
                    4   3
                    2   5
                    1   3
                    3   9
                    3   3",
        );
        assert_eq!(result, 31);
    }

    #[test]
    fn counts() {
        let result = count_occurrences(vec![4, 3, 5, 3, 9, 3]);
        assert_eq!(result, [(4, 1), (3, 3), (5, 1), (9, 1)].iter().cloned().collect());
    }
}
