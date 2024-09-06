//! You take the boat and find the gardener right where you were told he would be: managing a giant "garden" that looks more to you like a farm.
//!
//! "A water source? Island Island is the water source!" You point out that Snow Island isn't receiving any water.
//!
//! "Oh, we had to stop the water because we ran out of sand to filter it with! Can't make snow with dirty water. Don't worry, I'm sure we'll get more sand soon; we only turned off the water a few days... weeks... oh no." His face sinks into a look of horrified realization.
//!
//! "I've been so busy making sure everyone here has food that I completely forgot to check why we stopped getting more sand! There's a ferry leaving soon that is headed over in that direction - it's much faster than your boat. Could you please go check it out?"
//!
//! You barely have time to agree to this request when he brings up another. "While you wait for the ferry, maybe you can help us with our food production problem. The latest Island Island Almanac just arrived and we're having trouble making sense of it."
//!
//! The almanac (your puzzle input) lists all of the seeds that need to be planted. It also lists what type of soil to use with each kind of seed, what type of fertilizer to use with each kind of soil, what type of water to use with each kind of fertilizer, and so on. Every type of seed, soil, fertilizer and so on is identified with a number, but numbers are reused by each category - that is, soil 123 and fertilizer 123 aren't necessarily related to each other.
//!
//! For example:
//!
//! ```text
//! seeds: 79 14 55 13
//!
//! seed-to-soil map:
//! 50 98 2
//! 52 50 48
//!
//! soil-to-fertilizer map:
//! 0 15 37
//! 37 52 2
//! 39 0 15
//!
//! fertilizer-to-water map:
//! 49 53 8
//! 0 11 42
//! 42 0 7
//! 57 7 4
//!
//! water-to-light map:
//! 88 18 7
//! 18 25 70
//!
//! light-to-temperature map:
//! 45 77 23
//! 81 45 19
//! 68 64 13
//!
//! temperature-to-humidity map:
//! 0 69 1
//! 1 0 69
//!
//! humidity-to-location map:
//! 60 56 37
//! 56 93 4
//! ```
//!
//! The almanac starts by listing which seeds need to be planted: seeds 79, 14, 55, and 13.
//!
//! The rest of the almanac contains a list of maps which describe how to convert numbers from a source category into numbers in a destination category. That is, the section that starts with seed-to-soil map: describes how to convert a seed number (the source) to a soil number (the destination). This lets the gardener and his team know which soil to use with which seeds, which water to use with which fertilizer, and so on.
//!
//! Rather than list every source number and its corresponding destination number one by one, the maps describe entire ranges of numbers that can be converted. Each line within a map contains three numbers: the destination range start, the source range start, and the range length.
//!
//! Consider again the example seed-to-soil map:
//!
//! ```text
//! 50 98 2
//! 52 50 48
//! ```
//!
//! The first line has a destination range start of 50, a source range start of 98, and a range length of 2. This line means that the source range starts at 98 and contains two values: 98 and 99. The destination range is the same length, but it starts at 50, so its two values are 50 and 51. With this information, you know that seed number 98 corresponds to soil number 50 and that seed number 99 corresponds to soil number 51.
//!
//! The second line means that the source range starts at 50 and contains 48 values: 50, 51, ..., 96, 97. This corresponds to a destination range starting at 52 and also containing 48 values: 52, 53, ..., 98, 99. So, seed number 53 corresponds to soil number 55.
//!
//! Any source numbers that aren't mapped correspond to the same destination number. So, seed number 10 corresponds to soil number 10.
//!
//! So, the entire list of seed numbers and their corresponding soil numbers looks like this:
//!
//! ```text
//! seed  soil
//! 0     0
//! 1     1
//! ...   ...
//! 48    48
//! 49    49
//! 50    52
//! 51    53
//! ...   ...
//! 96    98
//! 97    99
//! 98    50
//! 99    51
//! ```
//!
//! With this map, you can look up the soil number required for each initial seed number:
//!
//! - Seed number 79 corresponds to soil number 81.
//! - Seed number 14 corresponds to soil number 14.
//! - Seed number 55 corresponds to soil number 57.
//! - Seed number 13 corresponds to soil number 13.
//!
//! The gardener and his team want to get started as soon as possible, so they'd like to know the closest location that needs a seed. Using these maps, find the lowest location number that corresponds to any of the initial seeds. To do this, you'll need to convert each seed number through other categories until you can find its corresponding location number. In this example, the corresponding types are:
//!
//! - Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
//! - Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
//! - Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
//! - Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.
//!
//! So, the lowest location number in this example is 35.
//!
//! What is the lowest location number that corresponds to any of the initial seed numbers?

use winnow::{
    ascii::{alpha1, dec_uint, newline},
    combinator::{preceded, separated, separated_pair, terminated},
    PResult, Parser,
};

pub fn calculate(input: &str) -> u32 {
    Almanac::parse(input).get_lowest_location()
}

#[derive(Clone, Debug, PartialEq)]
struct Almanac {
    seeds: Vec<u32>,
    mappings: Vec<Mapping>,
}

impl Almanac {
    pub fn parse(input: &str) -> Self {
        parse_almanac.parse(input).unwrap()
    }
    pub fn get_lowest_location(&self) -> u32 {
        self.seeds
            .clone()
            .into_iter()
            .map(|s| {
                let mut current_mapping = self.get_mapping_from("seed").unwrap();
                let mut mapping_number = s;
                while current_mapping.to != "location" {
                    mapping_number = current_mapping.find_between(mapping_number);
                    current_mapping = self.get_mapping_from(&current_mapping.to()).unwrap();
                }
                dbg!(&mapping_number, current_mapping);
                mapping_number
            })
            .min()
            .unwrap_or_default()
    }
    fn get_mapping_from(&self, location: &str) -> Option<Mapping> {
        self.mappings
            .clone()
            .into_iter()
            .find(|m| m.from == location)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Mapping {
    from: String,
    to: String,
    plots: Vec<Plot>,
}

impl Mapping {
    fn find_between(&self, location: u32) -> u32 {
        self.plots
            .iter()
            .filter_map(|p| p.get_to(location))
            .min()
            .unwrap_or(location)
    }
    fn to(&self) -> String {
        self.to.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Plot {
    from: u32,
    to: u32,
    distance: u32,
}

impl Plot {
    fn new(from: u32, to: u32, distance: u32) -> Self {
        Self { from, to, distance }
    }
    fn get_to(&self, input: u32) -> Option<u32> {
        if input < self.from || input >= (self.from + self.distance) {
            return None;
        }
        let dist = self.to as i32 - self.from as i32;
        Some((input as i32 - dist) as u32)
    }
}

fn parse_plot(input: &mut &str) -> PResult<Plot> {
    let (from, _, to, _, distance) = (dec_uint, " ", dec_uint, " ", dec_uint).parse_next(input)?;
    Ok(Plot::new(from, to, distance))
}

fn parse_mapping(input: &mut &str) -> PResult<Mapping> {
    let ((from, to), _, plots) =
        (parse_header, newline, separated(1.., parse_plot, newline)).parse_next(input)?;
    Ok(Mapping {
        from: from.into(),
        to: to.into(),
        plots,
    })
}

fn parse_header<'a>(input: &mut &'a str) -> PResult<(&'a str, &'a str)> {
    terminated(separated_pair(alpha1, "-to-", alpha1), " map:").parse_next(input)
}

fn parse_seeds(input: &mut &str) -> PResult<Vec<u32>> {
    preceded("seeds: ", parse_number_seq).parse_next(input)
}

fn parse_number_seq(input: &mut &str) -> PResult<Vec<u32>> {
    separated(1.., dec_uint::<&str, u32, _>, ' ').parse_next(input)
}

fn parse_almanac<'a>(input: &mut &'a str) -> PResult<Almanac> {
    let (seeds, _, _, mappings) = (
        parse_seeds,
        newline,
        newline,
        separated(1.., parse_mapping, (newline, newline)),
    )
        .parse_next(input)?;
    Ok(Almanac { seeds, mappings })
}

#[cfg(test)]
mod day5_part_one_tests {
    use super::*;

    #[test]
    fn test_parse_seeds() {
        let input = "seeds: 79 14 55 13";
        let result = parse_seeds.parse(input).unwrap();
        let expected = vec![79, 14, 55, 13];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_plot() {
        let input = "49 53 8";
        let result = parse_plot.parse(input).unwrap();
        let expected = Plot::new(49, 53, 8);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_header_test() {
        let input = "water-to-light map:";
        let result = parse_header.parse(input).unwrap();
        let expected = ("water", "light");
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_mapping_test() {
        let input = r#"seed-to-soil map:
50 98 2
52 50 48"#;
        let result = parse_mapping.parse(input).unwrap();
        let expected = Mapping {
            from: "seed".into(),
            to: "soil".into(),
            plots: vec![Plot::new(50, 98, 2), Plot::new(52, 50, 48)],
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn test_plots_get_to() {
        let plots = Plot::new(52, 50, 48);
        assert_eq!(plots.get_to(55), Some(57));
        assert_eq!(plots.get_to(52), Some(54));
        assert_eq!(plots.get_to(51), None);
        assert_eq!(plots.get_to(99), Some(101));
        assert_eq!(plots.get_to(100), None);
    }

    #[test]
    fn test_mapping_find_between() {
        let mapping = Mapping {
            from: "seed".into(),
            to: "soil".into(),
            plots: vec![Plot::new(50, 98, 2), Plot::new(52, 50, 48)],
        };
        assert_eq!(mapping.find_between(79), 81);
        assert_eq!(mapping.find_between(14), 14);
        assert_eq!(mapping.find_between(55), 57);
        assert_eq!(mapping.find_between(13), 13);
    }

    const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
    #[test]
    fn parse_almanic_test() {
        let result = parse_almanac.parse(INPUT).unwrap();
        assert_eq!(result.mappings.len(), 7)
    }

    #[test]
    fn calculate_result() {
        let result = calculate(INPUT);
        assert_eq!(result, 35);
    }
}
