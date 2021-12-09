use std::collections::HashSet;

pub fn run() {
    let input = puzzle_input();
    println!("day08.part1.solution = {}", solve_part1(input));
    println!("day08.part2.solution = {}", solve_part2(input));
}

fn solve_part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let (_, outputs) = line.split_once('|').unwrap();
            outputs
                .split_ascii_whitespace()
                .filter(|&r| is_easy_digit(r))
                .count()
        })
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    input.trim().lines().map(decode_line).sum()
}

/// Decodes a line and returns its output value.
fn decode_line(line: &str) -> u32 {
    let signals: Vec<_> = line
        .split_ascii_whitespace()
        .filter(|&s| s != "|")
        .collect();
    let digits = decode(&signals);

    let idx = digits.len() - 4;

    digits[idx] * 1000 + digits[idx + 1] * 100 + digits[idx + 2] * 10 + digits[idx + 3]
}

fn decode(signals: &[&str]) -> Vec<u32> {
    let mut infos: Vec<_> = signals.iter().map(|&s| DecodeInfo::new(s)).collect();

    while !infos.iter().all(|x| x.decoded()) {
        for i in 0..infos.len() {
            if infos[i].decoded() {
                continue;
            }

            let mut info = infos[i].clone();

            for other in infos.iter().filter(|&info| info.decoded()) {
                info.try_deduce(other);
            }

            infos[i] = info;
        }
    }

    infos.iter().map(|i| i.digit()).collect()
}

#[derive(Clone)]
pub struct DecodeInfo {
    pub segments: HashSet<char>,
    pub possibilities: HashSet<u32>,
}

impl DecodeInfo {
    pub fn new(s: &str) -> DecodeInfo {
        let segments: HashSet<char> = s.chars().collect();

        let possibilities: HashSet<u32> = match segments.len() {
            2 => HashSet::from([1]),
            4 => HashSet::from([4]),
            3 => HashSet::from([7]),
            7 => HashSet::from([8]),
            5 => HashSet::from([2, 3, 5]),
            6 => HashSet::from([0, 6, 9]),
            _ => panic!("What the heck digit is this?"),
        };

        DecodeInfo {
            segments,
            possibilities,
        }
    }

    pub fn digit(&self) -> u32 {
        assert!(self.possibilities.len() == 1);
        *self.possibilities.iter().next().unwrap()
    }

    pub fn decoded(&self) -> bool {
        self.possibilities.len() == 1
    }

    pub fn try_deduce(&mut self, info: &DecodeInfo) {
        self.possibilities.drain_filter(|p| {
            let expected = expected_intersections(*p, info.digit());
            let actual = self.segments.intersection(&info.segments).count();
            expected != actual
        });
    }
}

/// The digits 1, 4, 7, and 8 all have a unique number of segments used in
/// their display: no other digit uses 2 segments, for example. As a result,
/// we consider these "easy digits" which we can always guess.
fn is_easy_digit(s: &str) -> bool {
    matches!(s.len(), 2 | 3 | 4 | 7)
}

/// Converts a digit back to the (good, non-garbled) segments.
fn digit_to_segments(a: u32) -> HashSet<char> {
    match a {
        0 => "abcefg".chars().collect(),
        1 => "cf".chars().collect(),
        2 => "acdeg".chars().collect(),
        3 => "acdfg".chars().collect(),
        4 => "bcdf".chars().collect(),
        5 => "abdfg".chars().collect(),
        6 => "abdefg".chars().collect(),
        7 => "acf".chars().collect(),
        8 => "abcdefg".chars().collect(),
        _ => "abcdfg".chars().collect(),
    }
}

fn expected_intersections(a: u32, b: u32) -> usize {
    digit_to_segments(a)
        .intersection(&digit_to_segments(b))
        .count()
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day08.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> &'static str {
        include_str!("../inputs/day08_example.txt")
    }

    #[test]
    fn verify_example_input_part1() {
        assert_eq!(26, solve_part1(example_input()));
    }

    #[test]
    fn verify_decodes_example_line() {
        let line =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab cdfeb fcadb cdfeb cdbaf";
        let signals: Vec<_> = line.split(' ').collect();

        assert_eq!(
            vec![8, 5, 2, 3, 7, 9, 6, 4, 0, 1, 5, 3, 5, 3],
            decode(&signals)
        );
        assert_eq!(5353, decode_line(line));
    }
}
