use std::collections::HashMap;

pub fn run() {
    let input = puzzle_input();
    println!("day06.part1.solution = {}", solve_part1(input));
    println!("day06.part2.solution = {}", solve_part2(input));
}

fn solve_part1(input: &str) -> usize {
    let initial_fish = parse_input(input);
    simulate(initial_fish, 80)
}

fn solve_part2(input: &str) -> usize {
    let initial_fish = parse_input(input);
    simulate(initial_fish, 256)
}

fn parse_input(input: &str) -> Vec<u8> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect()
}

fn simulate(initial_fish: Vec<u8>, days: usize) -> usize {
    let mut cycle_counts: HashMap<u8, usize> = HashMap::new();

    for cycle in 0..=8 {
        cycle_counts.insert(cycle, 0);
    }

    for &fish in &initial_fish {
        cycle_counts.entry(fish).and_modify(|e| *e += 1);
    }

    for _ in 0..days {
        let mut next_counts = cycle_counts.clone();

        for cycle in (1..=8).rev() {
            next_counts
                .entry(cycle - 1)
                .and_modify(|e| *e = cycle_counts[&cycle]);
        }

        next_counts.entry(8).and_modify(|e| *e = cycle_counts[&0]);
        next_counts.entry(6).and_modify(|e| *e += cycle_counts[&0]);

        cycle_counts = next_counts;
    }

    cycle_counts.values().sum()
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day06.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> &'static str {
        include_str!("../inputs/day06_example.txt")
    }

    #[test]
    pub fn can_parse_input() {
        let nums = parse_input(example_input());
        assert_eq!(5, nums.len());
    }

    #[test]
    fn verify_small_simulation() {
        let fish: Vec<u8> = vec![3, 4, 3, 1, 2];
        assert_eq!(7, simulate(fish.clone(), 3));
        assert_eq!(26, simulate(fish.clone(), 18));
    }

    #[test]
    fn verify_example_input_part1() {
        assert_eq!(5934, solve_part1(example_input()));
    }

    #[test]
    fn verify_example_input_part2() {
        assert_eq!(26984457539, solve_part2(example_input()));
    }
}
