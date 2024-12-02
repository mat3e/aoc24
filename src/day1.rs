fn main() {
    println!(
        "{}",
        resolve_first(
            "3   4
                    4   3
                    2   5
                    1   3
                    3   9
                    3   3"
        )
    );
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
        if i % 2 == 1 {
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
}
