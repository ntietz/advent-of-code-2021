pub fn run() {
    part1();
    part2();
}

pub fn part1() {
    let solution = solve_part1(puzzle_input());
    println!("day01.part1.solution = {}", solution);
}

pub fn part2() {
    let solution = solve_part2(puzzle_input());
    println!("day01.part2.solution = {}", solution);
}

fn solve_part1(input: &str) -> usize {
    let depths = parse_input(input);

    depths.windows(2).filter(|&pair| pair[1] > pair[0]).count()
}

fn solve_part2(input: &str) -> usize {
    let depths = parse_input(input);

    let sums: Vec<u64> = depths.windows(3).map(|w| w.iter().sum()).collect();
    sums.windows(2).filter(|&pair| pair[1] > pair[0]).count()
}

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|s| s.parse::<u64>().unwrap()).collect()
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day01.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> &'static str {
        include_str!("../inputs/day01_example.txt")
    }

    #[test]
    fn verify_example_input_part1() {
        assert_eq!(7, solve_part1(example_input()));
    }

    #[test]
    fn verify_example_input_part2() {
        assert_eq!(5, solve_part2(example_input()));
    }
}
