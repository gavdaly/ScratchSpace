//! Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

pub fn sum_lines(input: &str) -> Option<usize> {
    input
        .lines()
        .map(|l| {
            let vec = parse(l);
            Some((vec.first()? * 10) + vec.last()?)
        })
        .sum()
}

fn parse(line: &str) -> Vec<usize> {
    let mut result = Vec::with_capacity(line.len());
    for index in 0..line.len() {
        let reduced_line = &line[index..];

        if reduced_line.starts_with("one") || reduced_line.starts_with('1') {
            result.push(1);
        } else if reduced_line.starts_with("two") || reduced_line.starts_with('2') {
            result.push(2);
        } else if reduced_line.starts_with("three") || reduced_line.starts_with('3') {
            result.push(3);
        } else if reduced_line.starts_with("four") || reduced_line.starts_with('4') {
            result.push(4);
        } else if reduced_line.starts_with("five") || reduced_line.starts_with('5') {
            result.push(5);
        } else if reduced_line.starts_with("six") || reduced_line.starts_with('6') {
            result.push(6);
        } else if reduced_line.starts_with("seven") || reduced_line.starts_with('7') {
            result.push(7);
        } else if reduced_line.starts_with("eight") || reduced_line.starts_with('8') {
            result.push(8);
        } else if reduced_line.starts_with("nine") || reduced_line.starts_with('9') {
            result.push(9);
        };
    }
    result
}

#[cfg(test)]
mod day1_part2_tests {
    use super::*;

    #[test]
    fn test_parsing_one() {
        let first = format!("{:?}", parse("two1nine"));
        let second = format!("{:?}", vec![2, 1, 9]);
        assert_eq!(first, second);
    }
    #[test]
    fn test_parsing_two() {
        let first = format!("{:?}", parse("eightwothree"));
        let second = format!("{:?}", vec![8, 2, 3]);
        assert_eq!(first, second);
    }
    #[test]
    fn test_parsing_three() {
        let first = format!("{:?}", parse("abcone2threexyz"));
        let second = format!("{:?}", vec![1, 2, 3]);
        assert_eq!(first, second);
    }
    #[test]
    fn test_parsing_four() {
        let first = format!("{:?}", parse("xtwone3four"));
        let second = format!("{:?}", vec![2, 1, 3, 4]);
        assert_eq!(first, second);
    }
    #[test]
    fn test_parsing_five() {
        let first = format!("{:?}", parse("4nineeightseven2"));
        let second = format!("{:?}", vec![4, 9, 8, 7, 2]);
        assert_eq!(first, second);
    }
    #[test]
    fn test_parsing_six() {
        let first = format!("{:?}", parse("zoneight234"));
        let second = format!("{:?}", vec![1, 8, 2, 3, 4]);
        assert_eq!(first, second);
    }
    #[test]
    fn test_parsing_seven() {
        let first = format!("{:?}", parse("7pqrstsixteen"));
        let second = format!("{:?}", vec![7, 6]);
        assert_eq!(first, second);
    }

    #[test]
    fn sum_lines_test() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        assert_eq!(sum_lines(input), Some(281))
    }
}
