//! Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.
//!
//! You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//!
//! You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").
//!
//! As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.
//!
//! The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

pub fn sum_lines(input: &str) -> Option<usize> {
    input.lines().map(parse_line).sum()
}

fn parse_line(line: &str) -> Option<usize> {
    let first = line.chars().find(|c| c.is_numeric())?;
    let last = line.chars().rev().find(|c| c.is_numeric())?;
    let string_number = format!("{first}{last}");
    string_number.parse().ok()
}

#[cfg(test)]
mod day_one_part_one_test {
    use super::*;
    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("1abc2"), Some(12));
        assert_eq!(parse_line("pqr3stu8vwx"), Some(38));
        assert_eq!(parse_line("a1b2c3d4e5f"), Some(15));
        assert_eq!(parse_line("treb7uchet"), Some(77));
    }
    #[test]
    fn test_sum_lines() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        assert_eq!(sum_lines(input), Some(142));
    }
}
