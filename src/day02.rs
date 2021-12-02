pub fn run() {
    let input = puzzle_input();
    println!("day02.part1.solution = {}", solve_part1(input));
    println!("day02.part2.solution = {}", solve_part2(input));
}

fn solve_part1(input: &str) -> i32 {
    let commands = parse_input(input);

    let final_pos = commands.iter().fold((0, 0), |(x, y), cmd| match cmd {
        Command::Forward(n) => (x + n, y),
        Command::Down(n) => (x, y + n),
        Command::Up(n) => (x, y - n),
    });

    final_pos.0 * final_pos.1
}

fn solve_part2(input: &str) -> i32 {
    let commands = parse_input(input);

    let final_pos = commands
        .iter()
        .fold((0, 0, 0), |(x, y, aim), cmd| match cmd {
            Command::Forward(n) => (x + n, y + aim * n, aim),
            Command::Down(n) => (x, y, aim + n),
            Command::Up(n) => (x, y, aim - n),
        });

    final_pos.0 * final_pos.1
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day02.txt")
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Command {
    /// Parse takes in a string, such as "forward 3", and returns an Option
    /// of a parsed Command. If parsing fails, it will return None.
    ///
    /// ```
    /// use aoc::day02::Command;
    /// assert_eq!(Command::parse("forward 3"), Some(Command::Forward(3)));
    /// assert_eq!(Command::parse("blah blah"), None);
    /// assert_eq!(Command::parse("forward three"), None);
    /// ```
    pub fn parse(cmd: &str) -> Option<Command> {
        match cmd.split_once(" ") {
            Some((dir, amt)) => {
                let amt = amt.parse::<i32>();
                match (dir, amt) {
                    ("forward", Ok(x)) => Some(Command::Forward(x)),
                    ("down", Ok(x)) => Some(Command::Down(x)),
                    ("up", Ok(x)) => Some(Command::Up(x)),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

fn parse_input(input: &str) -> Vec<Command> {
    input.lines().filter_map(Command::parse).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> &'static str {
        include_str!("../inputs/day02_example.txt")
    }

    #[test]
    fn verify_example_input_part1() {
        assert_eq!(150, solve_part1(example_input()));
    }

    #[test]
    fn verify_example_input_part2() {
        assert_eq!(900, solve_part2(example_input()));
    }
}
