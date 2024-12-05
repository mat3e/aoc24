fn main() {
    dbg!(calculate_xmas(
        ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."
    ));
}

fn calculate_xmas(input: &str) -> usize {
    calculate(input, "XMAS") + calculate(input, "SAMX")
}

fn calculate(input: &str, pattern: &str) -> usize {
    let expected_char = pattern.chars().next().unwrap();
    let mut result: usize = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, curr_char)| {
            if curr_char == expected_char {
                let south =
                    count_single_in_direction(input, &pattern[1..], Start::from((x, y)), SOUTH);
                let east =
                    count_single_in_direction(input, &pattern[1..], Start::from((x, y)), EAST);
                let sw = count_single_in_direction(
                    input,
                    &pattern[1..],
                    Start::from((x, y)),
                    SOUTH_WEST,
                );
                let se = count_single_in_direction(
                    input,
                    &pattern[1..],
                    Start::from((x, y)),
                    SOUTH_EAST,
                );
                result = result + south + east + sw + se;
            }
        });
    });
    result
}

fn count_single_in_direction(
    input: &str,
    pattern: &str,
    start: Start,
    coordinates: (i8, i8),
) -> usize {
    if start.x.is_none() || start.y.is_none() {
        return 0;
    }
    let mut chars = pattern.chars();
    let mut next_x = start.x;
    let mut next_y = start.y;
    for _ in 0..pattern.len() {
        let expected_char = chars.next().unwrap();
        next_x = next_x_forward(next_x.unwrap(), coordinates, input);
        next_y = next_y_forward(next_y.unwrap(), coordinates, input);
        if next_x.is_none() || next_y.is_none() {
            return 0;
        }
        let curr_char = input
            .lines()
            .nth(next_y.unwrap())
            .unwrap()
            .chars()
            .nth(next_x.unwrap())
            .unwrap();
        if curr_char != expected_char {
            return 0;
        }
    }
    1
}

fn next_x_forward(current: usize, coordinates: (i8, i8), input: &str) -> Option<usize> {
    if coordinates.0 >= 0 {
        current.checked_add(coordinates.0 as usize)
    } else {
        current.checked_sub(-coordinates.0 as usize)
    }
    .filter(|val| *val < input.lines().next().unwrap().len() && *val >= 0)
}

fn next_y_forward(current: usize, coordinates: (i8, i8), input: &str) -> Option<usize> {
    current
        .checked_add(coordinates.1 as usize)
        .filter(|val| *val < input.lines().count())
}

struct Start {
    x: Option<usize>,
    y: Option<usize>,
}

impl Default for Start {
    fn default() -> Self {
        Self {
            x: Some(0),
            y: Some(0),
        }
    }
}

impl From<(usize, usize)> for Start {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: Some(x),
            y: Some(y),
            ..Self::default()
        }
    }
}

const SOUTH: (i8, i8) = (0, 1);
const EAST: (i8, i8) = (1, 0);
const SOUTH_EAST: (i8, i8) = (1, 1);
const SOUTH_WEST: (i8, i8) = (-1, 1);

mod tests {
    use super::*;

    #[test]
    fn resolves_first() {
        assert_eq!(
                    (calculate_xmas(
                        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
                    )),
                    18
                );
        assert_eq!(
            (calculate_xmas(
                "..X...
.SAMX.
.A..A.
XMAS.S
.X...."
            )),
            4
        );
    }

    #[test]
    fn calculates_for_given_string() {
        assert_eq!(
            calculate(
                "...XMAS
..MMM..
.A.A.A.
S..S..S",
                "XMAS"
            ),
            4
        );
        assert_eq!(
            calculate(
                "..X...
.SAMX.
.A..A.
XMAS.S
.X....",
                "SAMX"
            ),
            2
        )
    }

    #[test]
    fn should_match_in_direction() {
        assert_eq!(
            count_single_in_direction(
                "...XMAS...XMAS
..MMM....MMM..
.A.A.A..A.A.A.
S..S..SS..S..S",
                &"XMAS"[1..],
                Start {
                    x: Some(3),
                    ..Start::default()
                },
                EAST
            ),
            1
        );
        assert_eq!(
            count_single_in_direction(
                "...XMAS...XMAS
..MMM....MMM..
.A.A.A..A.A.A.
S..S..SS..S..S",
                &"XMAS"[1..],
                Start {
                    x: Some(10),
                    ..Start::default()
                },
                EAST
            ),
            1
        );
        assert_eq!(
            count_single_in_direction(
                "...XMAS
..MMM..
.A.A.A.
S..S..S",
                &"XMAS"[1..],
                Start {
                    x: Some(3),
                    ..Start::default()
                },
                SOUTH_WEST
            ),
            1
        );
        assert_eq!(
            count_single_in_direction(
                "..X...
.SAMX.
.A..A.
XMAS.S
.X....",
                &"SAMX"[1..],
                Start {
                    x: Some(1),
                    y: Some(1),
                    ..Start::default()
                },
                SOUTH
            ),
            1
        );
        assert_eq!(
            count_single_in_direction(
                "..X...
.SAMX.
.A..A.
XMAS.S
.X....",
                &"SAMX"[1..],
                Start {
                    x: Some(1),
                    y: Some(1),
                    ..Start::default()
                },
                EAST
            ),
            1
        );
    }
}