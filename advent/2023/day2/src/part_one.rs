//! You're launched high into the atmosphere! The apex of your trajectory just barely reaches the surface of a large island floating in the sky. You gently land in a fluffy pile of leaves. It's quite cold, but you don't see much snow. An Elf runs over to greet you.
//!
//! The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll be happy to explain the situation, but it's a bit of a walk, so you have some time. They don't get many visitors up here; would you like to play a game in the meantime?
//!
//! As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue. Each time you play this game, he will hide a secret number of cubes of each color in the bag, and your goal is to figure out information about the number of cubes.
//!
//! To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.
//!
//! You play several games and record the information from each game (your puzzle input). Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

#[derive(PartialEq, Debug)]
pub struct Game {
    pub number: usize,
    pub hands: Vec<Hand>,
}

#[derive(PartialEq, Debug)]
pub struct Hand {
    pub red: usize,
    pub blue: usize,
    pub green: usize,
}

pub fn calculate(input: &str) -> usize {
    let hands = input.lines().map(parse);
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    hands.fold(0, |mut total, game| {
        let result = game.hands.into_iter().fold(true, |is_valid, hand| {
            if hand.red <= 12 && hand.green <= 13 && hand.blue <= 14 {
                is_valid
            } else {
                false
            }
        });
        if result {
            total += game.number;
        }
        total
    })
}

pub fn parse(input: &str) -> Game {
    let (game_number, hands_str) = input.split_once(':').unwrap();
    let (_, game_number) = game_number.split_once(" ").unwrap();
    let number = game_number.parse::<usize>().unwrap();
    let hands = hands_str
        .split("; ")
        .map(|hand| {
            hand.split(", ").fold(
                Hand {
                    red: 0,
                    blue: 0,
                    green: 0,
                },
                |hand, string| {
                    let (count, color) = string.trim().split_once(" ").unwrap();
                    let amount = count.parse::<usize>().unwrap();
                    match color {
                        "red" => Hand {
                            red: amount,
                            ..hand
                        },
                        "blue" => Hand {
                            blue: amount,
                            ..hand
                        },
                        "green" => Hand {
                            green: amount,
                            ..hand
                        },
                        _ => panic!(),
                    }
                },
            )
        })
        .collect::<Vec<_>>();

    Game { number, hands }
}

#[cfg(test)]
mod day_two_part_one_test {
    use super::*;
    #[test]
    fn test_calculate() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let result = calculate(input);
        assert_eq!(result, 8);
    }
    #[test]
    fn test_parse1() {
        let parsed = parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let game = Game {
            number: 1,
            hands: vec![
                Hand {
                    blue: 3,
                    red: 4,
                    green: 0,
                },
                Hand {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Hand {
                    red: 0,
                    blue: 0,
                    green: 2,
                },
            ],
        };
        assert_eq!(parsed, game);
    }
    #[test]
    fn test_parse2() {
        let parsed = parse("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
        let game = Game {
            number: 2,
            hands: vec![
                Hand {
                    blue: 1,
                    red: 0,
                    green: 2,
                },
                Hand {
                    red: 1,
                    green: 3,
                    blue: 4,
                },
                Hand {
                    red: 0,
                    blue: 1,
                    green: 1,
                },
            ],
        };
        assert_eq!(parsed, game);
    }
    #[test]
    fn test_parse3() {
        let parsed =
            parse("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
        let game = Game {
            number: 3,
            hands: vec![
                Hand {
                    blue: 6,
                    red: 20,
                    green: 8,
                },
                Hand {
                    red: 4,
                    green: 13,
                    blue: 5,
                },
                Hand {
                    red: 1,
                    blue: 0,
                    green: 5,
                },
            ],
        };
        assert_eq!(parsed, game);
    }
    #[test]
    fn test_parse4() {
        let parsed =
            parse("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
        let game = Game {
            number: 4,
            hands: vec![
                Hand {
                    blue: 6,
                    red: 3,
                    green: 1,
                },
                Hand {
                    red: 6,
                    green: 3,
                    blue: 0,
                },
                Hand {
                    red: 14,
                    blue: 15,
                    green: 3,
                },
            ],
        };
        assert_eq!(parsed, game);
    }
    #[test]
    fn test_parse5() {
        let parsed = parse("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        let game = Game {
            number: 5,
            hands: vec![
                Hand {
                    blue: 1,
                    red: 6,
                    green: 3,
                },
                Hand {
                    red: 1,
                    green: 2,
                    blue: 2,
                },
            ],
        };
        assert_eq!(parsed, game);
    }
}
