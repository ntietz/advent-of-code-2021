use num::PrimInt;

pub fn run() {
    let input = puzzle_input();
    println!("day03.part1.solution = {}", solve_part1(input));
    println!("day03.part2.solution = {}", solve_part2(input));
}

fn solve_part1(input: &str) -> u32 {
    let bit_width = input.lines().next().unwrap().len();

    let nums: Vec<_> = input
        .lines()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect();

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for idx in 0..bit_width {
        let mask: u32 = 1 << idx;

        if count_mask(&nums, mask) > nums.len() / 2 {
            gamma |= mask;
        } else {
            epsilon |= mask;
        }
    }

    gamma * epsilon
}

fn solve_part2(input: &str) -> u32 {
    let bit_width = input.lines().next().unwrap().len();

    let nums: Vec<_> = input
        .lines()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect();

    let mut oxygen_ratings = nums.clone();
    let mut scrubber_ratings = nums;

    for idx in (0..bit_width).rev() {
        let mask: u32 = 1 << idx;

        if oxygen_ratings.len() > 1 {
            if count_mask(&oxygen_ratings, mask) * 2 >= oxygen_ratings.len() {
                oxygen_ratings.retain(|&n| n & mask > 0);
            } else {
                oxygen_ratings.retain(|&n| n & mask == 0);
            }
        }

        if scrubber_ratings.len() > 1 {
            if count_mask(&scrubber_ratings, mask) * 2 >= scrubber_ratings.len() {
                scrubber_ratings.retain(|&n| n & mask == 0);
            } else {
                scrubber_ratings.retain(|&n| n & mask > 0);
            }
        }
    }

    oxygen_ratings[0] * scrubber_ratings[0]
}

fn count_mask<T: PrimInt>(nums: &[T], mask: T) -> usize {
    nums.iter().filter(|&n| (*n) & mask > T::zero()).count()
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day03.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> &'static str {
        include_str!("../inputs/day03_example.txt")
    }

    #[test]
    fn verify_example_input_part1() {
        assert_eq!(198, solve_part1(example_input()));
    }

    #[test]
    fn verify_example_input_part2() {
        assert_eq!(230, solve_part2(example_input()));
    }
}
