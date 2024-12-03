fn main() {
    dbg!(calculate(
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    ));
}

fn calculate(input: &str) -> i32 {
    regex::Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|cap| {
            let x = cap[1].parse::<i32>().unwrap();
            let y = cap[2].parse::<i32>().unwrap();
            x * y
        })
        .sum()
}

mod tests {
    use super::*;

    #[test]
    fn resolves_first() {
        assert_eq!(
            (calculate("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")),
            161
        );
    }

    macro_rules! do_nothing_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(calculate($value), 0);
                }
            )*
        }
    }

    do_nothing_tests!(
        single_param: "mul(4*",
        unclosed: "mul(6,9!",
        no_mul: "?(12,34)",
        whitespaces: "mul ( 2 , 4 )",
    );
}
