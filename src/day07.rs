pub fn run() {
    let input = puzzle_input();
    println!("day07.part1.solution = {}", solve_part1(input));
    println!("day07.part2.solution = {}", solve_part2(input));
}

fn solve_part1(input: &str) -> i32 {
    let positions = parse_input(input);

    (0..*(positions.iter().max().unwrap()))
        .map(|p| positions.iter().map(|x| (x - p).abs()).sum())
        .min()
        .unwrap()
}

fn solve_part2(input: &str) -> i32 {
    let positions = parse_input(input);

    (0..*(positions.iter().max().unwrap()))
        .map(|p| positions.iter().map(|x| cost((x - p).abs())).sum())
        .min()
        .unwrap()
}

fn cost(steps: i32) -> i32 {
    (steps * (steps + 1)) / 2
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day07.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> &'static str {
        include_str!("../inputs/day07_example.txt")
    }

    #[test]
    fn verify_example_input_part1() {
        assert_eq!(37, solve_part1(example_input()));
    }

    #[test]
    fn verify_example_input_part2() {
        assert_eq!(168, solve_part2(example_input()));
    }

    #[test]
    fn verify_costs() {
        assert_eq!(1, cost(1));
        assert_eq!(3, cost(2));
        assert_eq!(6, cost(3));
        assert_eq!(10, cost(4));
    }
}
