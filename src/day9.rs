fn main() {
    dbg!(checksum(&compact(&map("2333133121414131402"))));
}

const SPACE_CHAR: &'static str = ".";

fn map(input: &str) -> Vec<String> {
    input.chars().enumerate().fold(vec![], |mut acc, (i, c)| {
        let mut char_to_repeat = (i / 2).to_string();
        if i % 2 == 1 {
            char_to_repeat = SPACE_CHAR.into();
        }
        for _ in 0..c.to_digit(10).unwrap() {
            acc.push(char_to_repeat.clone());
        }
        acc
    })
}

fn compact(input: &Vec<String>) -> Vec<&str> {
    let mut result = vec![];
    for i in 0..input.len() {
        result.push(input[i].as_str());
    }
    let mut last_swap = 0;
    for i in (last_swap..input.len()).rev() {
        if input[i] == SPACE_CHAR {
            continue;
        }
        for j in last_swap..i {
            if result[j] != SPACE_CHAR {
                continue;
            }
            result[j] = &input[i];
            result[i] = SPACE_CHAR;
            last_swap = j;
            break;
        }
    }
    result
}

fn checksum(input: &Vec<&str>) -> usize {
    input.iter().enumerate().fold(0, |acc, (i, &c)| {
        if c == SPACE_CHAR {
            return acc;
        }
        acc + (i * c.parse::<usize>().unwrap())
    })
}

mod tests {
    use super::*;

    #[test]
    fn checksums() {
        assert_eq!(checksum(&compact(&map("2333133121414131402"))), 1928);
    }

    #[test]
    fn compacts() {
        assert_eq!(
            compact(&map("12345")),
            "022111222......"
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        );

        assert_eq!(
            compact(&map("2333133121414131402")),
            "0099811188827773336446555566.............."
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn maps() {
        assert_eq!(
            map("12345"),
            vec!["0", ".", ".", "1", "1", "1", ".", ".", ".", ".", "2", "2", "2", "2", "2"]
        );

        assert_eq!(
            map("2333133121414131402"),
            "00...111...2...333.44.5555.6666.777.888899"
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        );
    }
}
