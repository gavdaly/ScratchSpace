//! Just as you're about to report your findings to the Elf, one of you realizes that the rules have actually been printed on the back of every card this whole time.
//!
//! There's no such thing as "points". Instead, scratchcards only cause you to win more scratchcards equal to the number of winning numbers you have.
//!
//! Specifically, you win copies of the scratchcards below the winning card equal to the number of matches. So, if card 10 were to have 5 matching numbers, you would win one copy each of cards 11, 12, 13, 14, and 15.
//!
//! Copies of scratchcards are scored like normal scratchcards and have the same card number as the card they copied. So, if you win a copy of card 10 and it has 5 matching numbers, it would then win a copy of the same cards that the original card 10 won: cards 11, 12, 13, 14, and 15. This process repeats until none of the copies cause you to win any more cards. (Cards will never make you copy a card past the end of the table.)
//!
//! This time, the above example goes differently:
//!
//! ```text
//! Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
//! Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
//! Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
//! Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
//! Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
//! Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
//! ```
//!
//! Card 1 has four matching numbers, so you win one copy each of the next four cards: cards 2, 3, 4, and 5.
//! Your original card 2 has two matching numbers, so you win one copy each of cards 3 and 4.
//! Your copy of card 2 also wins one copy each of cards 3 and 4.
//! Your four instances of card 3 (one original and three copies) have two matching numbers, so you win four copies each of cards 4 and 5.
//! Your eight instances of card 4 (one original and seven copies) have one matching number, so you win eight copies of card 5.
//! Your fourteen instances of card 5 (one original and thirteen copies) have no matching numbers and win no more cards.
//! Once all of the originals and copies have been processed, you end up with 1 instance of card 1, 2 instances of card 2, 4 instances of card 3, 8 instances of card 4, 14 instances of card 5, and 1 instance of card 6. In total, this example pile of scratchcards causes you to ultimately have 30 scratchcards!
//!
//! Process all of the original and copied scratchcards until no more scratchcards are won. Including the original set of scratchcards, how many total scratchcards do you end up with?

use std::collections::{BTreeMap, HashSet};

pub fn calculate(input: &str) -> i32 {
    let mut tickets = Tickets::parse(input);
    tickets.calculate()
}

#[derive(Debug)]
struct Tickets {
    value: BTreeMap<usize, Ticket>,
}

#[derive(Debug)]
struct Ticket {
    quantity: i32,
    wins: u32,
}

impl Tickets {
    pub fn parse(input: &str) -> Self {
        let mut value = BTreeMap::new();
        input.lines().enumerate().for_each(|(index, line)| {
            let (numbers, ticket) = parse_line(line);
            let wins = ticket.intersection(&numbers).count() as u32;
            value.insert(index, Ticket { quantity: 1, wins });
        });
        Self { value }
    }
    pub fn calculate(&mut self) -> i32 {
        let count = self.value.len();
         for ind in 0..count {
            if let Some(Ticket { quantity, wins }) = self.value.get(&ind) {
                let q = quantity.clone();
                for i in 1..=*wins {
                    self.inc(&(ind + i as usize), q as u32);
                }
            }
        }
        self.value
            .iter()
            .fold(0, |total, (_, Ticket { quantity, wins: _ })| {
                total + quantity
            })
    }
    fn inc(&mut self, index: &usize, amount: u32) {
        if let Some(Ticket { quantity, wins: _ }) = self.value.get_mut(index) {
            *quantity += amount as i32;
        };
    }
}

fn parse_line(input: &str) -> (HashSet<u32>, HashSet<u32>) {
    let (_, rest) = input.split_once(": ").expect("to have correct formatting");
    let (numbers, card) = rest.split_once(" | ").expect("To have correct formatting");
    let numbers = numbers
        .split_whitespace()
        .map(|n| n.parse::<u32>().expect("To only have numbers"))
        .collect::<HashSet<u32>>();
    let ticket = card
        .split_whitespace()
        .map(|n| n.parse::<u32>().expect("to only have numbers"))
        .collect::<HashSet<u32>>();
    (numbers, ticket)
}

#[cfg(test)]
mod day4_part_2_tests {
    use super::*;
    #[test]
    fn calculate_test() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
       let result = calculate(input);
        assert_eq!(result, 30)
    }
}
