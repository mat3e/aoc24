fn main() {
    dbg!(checksum(
        &full_compact(&map("2333133121414131402"))
            .join("")
            .split("")
            .collect()
    ));
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

fn full_compact(input: &Vec<String>) -> Vec<String> {
    let mut result = vec![];
    let mut i = 0;
    while i < input.len() {
        let current_char = input[i].as_str();
        let mut count = 1;
        while i + 1 < input.len() && input[i + 1] == current_char {
            i += 1;
            count += 1;
        }
        result.push(current_char.repeat(count));
        i += 1;
    }
    compact(&result).iter().map(|c| c.to_string()).collect()
}

fn compact(input: &Vec<String>) -> Vec<&str> {
    let mut result = vec![];
    for i in 0..input.len() {
        result.push(input[i].as_str());
    }
    let mut new_indexes: Vec<usize> = vec![];
    for i in (0..input.len()).rev() {
        if input[i] == SPACE_CHAR {
            continue;
        }
        for j in 0..i + new_indexes.len() {
            new_indexes.iter().filter(|&&x| x < j).count();
            let calibration_i = new_indexes.iter().filter(|&&x| x < i).count();
            let swap_candidate = result[j];
            if swap_candidate.len() < input[i].len()
                || swap_candidate.chars().any(|c| c.to_string() != SPACE_CHAR)
            {
                continue;
            }
            let temp = result[j];
            result[j] = &input[i];
            dbg!(&input[i]);
            let size_diff = swap_candidate.len() - input[i].len();
            if size_diff == 0 {
                result[i + calibration_i] = dbg!(temp);
            } else {
                let at_end = &input[j - new_indexes.len()][0..input[i].len()];
                dbg!(&at_end);
                result[i + calibration_i] = at_end;
                let in_between = &input[j - new_indexes.len()][0..size_diff];
                dbg!(&in_between);
                result.insert(j + 1, in_between);
                new_indexes.push(j + 1);
                new_indexes.sort()
            }
            break;
        }
    }
    result
}

fn checksum(input: &Vec<&str>) -> usize {
    input.iter().filter(|element| !element.is_empty()).enumerate().fold(0, |acc, (i, &c)| {
        if c == SPACE_CHAR || c == "" {
            return acc;
        }
        acc + (i * c.parse::<usize>().unwrap())
    })
}

mod tests {
    use super::*;

    #[test]
    fn resolves_second() {
        assert_eq!(
            checksum(
                &full_compact(&map("2333133121414131402"))
                    .join("")
                    .split("")
                    .collect()
            ),
            2858
        );
    }

    #[test]
    fn full_compacts() {
        assert_eq!(
            full_compact(&map("2333133121414131402")).join(""),
            "00992111777.44.333....5555.6666.....8888.."
        );
    }

    #[test]
    fn resolves_first() {
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
