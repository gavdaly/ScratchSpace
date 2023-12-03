//! The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.
//!
//! You do not seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.
//!
//! Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.
//!
//! The missing part was not the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.
//!
//! This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.
//!
//! Consider the same engine schematic again:
//!
//! ```test
//! 467..114..
//! ...*......
//! ..35..633.
//! ......#...
//! 617*......
//! .....+.58.
//! ..592.....
//! ......755.
//! ...$.*....
//! .664.598..
//! ```
//!
//! In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.

use std::collections::BTreeMap;

#[must_use]
pub fn calculate(input: &str) -> usize {
    Schematic::parse(input).calc()
}

#[derive(Clone)]
struct Schematic {
    numbers: BTreeMap<(usize, usize, usize), usize>,
    symbols: BTreeMap<(usize, usize), char>,
}

impl Schematic {
    pub fn parse(input: &str) -> Self {
        let (numbers, symbols) = input.lines().enumerate().fold(
            (BTreeMap::new(), BTreeMap::new()),
            |(mut numbers, mut symbols), (y, line)| {
                let mut numbs = 0;
                let mut first_index = None;
                line.chars().enumerate().for_each(|(x, char)| match char {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        let digit = char.to_digit(10).expect("should be a number") as usize;
                        if first_index.is_none() {
                            first_index = Some(x + 1);
                        }
                        numbs = numbs * 10 + digit;
                    }
                    '.' => {
                        if first_index.is_some() {
                            let x1 = first_index.expect("checked before");
                            numbers.insert((y + 1, x1, x), numbs);
                            first_index = None;
                            numbs = 0;
                        }
                    }
                    c => {
                        if first_index.is_some() {
                            let x1 = first_index.expect("checked before");
                            numbers.insert((y + 1, x1, x), numbs);
                            first_index = None;
                            numbs = 0;
                        }
                        symbols.insert((y + 1, x + 1), c);
                    }
                });
                if first_index.is_some() {
                    let x1 = first_index.expect("checked before");
                    numbers.insert((y + 1, x1, line.len()), numbs);
                }
                (numbers, symbols)
            },
        );
        Self { numbers, symbols }
    }
    pub fn calc(&self) -> usize {
        let gears = self.symbols.clone().into_iter().filter(|(_, c)| c == &'*');
        gears.fold(0, |total, ((y, x), _)| {
            let numbs = self.surrounding_numbers(y, x);
            if numbs.len() == 2 {
                total + numbs.iter().product::<usize>()
            } else {
                total
            }
        })
    }
    fn surrounding_numbers(&self, y: usize, x: usize) -> Vec<usize> {
        let y_range = y - 1..=y + 1;
        let x_range = x - 1..=x + 1;

        self.numbers
            .clone()
            .into_iter()
            .filter_map(|((y, x1, x2), n)| {
                let contains_y = y_range.contains(&y);
                let mut range = x1..=x2;
                let contains_x = range.any(|item| x_range.contains(&item));

                if contains_y && contains_x {
                    Some(n)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod day3_part_two_tests {
    use super::*;
    #[test]
    fn test_part_two() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let result = Schematic::parse(input).calc();
        assert_eq!(result, 467835)
    }
}
