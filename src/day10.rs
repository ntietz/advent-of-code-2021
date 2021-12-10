pub fn run() {
    let input = puzzle_input();
    println!("day10.part1.solution = {}", solve_part1(input));
    println!("day10.part2.solution = {}", solve_part2(input));
}

fn solve_part1(input: &str) -> u64 {
    input
        .lines()
        .filter_map(check_syntax)
        .map(|se| match se {
            SyntaxError::Incomplete(_) => 0,
            SyntaxError::MismatchedChunk(_, c) => score_mismatch(c),
            SyntaxError::CloseBeforeOpen(c) => score_mismatch(c),
        })
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    let mut scores: Vec<_> = input
        .lines()
        .filter_map(check_syntax)
        .filter_map(|se| match se {
            SyntaxError::Incomplete(s) => Some(score_incomplete(s)),
            _ => None,
        })
        .collect();
    scores.sort_unstable();

    scores[scores.len()/2]
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day10.txt")
}

#[derive(Debug, PartialEq)]
enum SyntaxError {
    Incomplete(String),
    MismatchedChunk(char, char),
    CloseBeforeOpen(char),
}

fn check_syntax(line: &str) -> Option<SyntaxError> {
    let mut stack: Vec<char> = vec![];

    for c in line.chars() {
        if matches!(c, '(' | '[' | '{' | '<') {
            stack.push(c);
        } else if let Some(prev) = stack.pop() {
            if !valid_pair(prev, c) {
                return Some(SyntaxError::MismatchedChunk(prev, c));
            }
        } else {
            return Some(SyntaxError::CloseBeforeOpen(c));
        }
    }

    if !stack.is_empty() {
        return Some(SyntaxError::Incomplete(stack.into_iter().collect()));
    }

    None
}

fn valid_pair(left: char, right: char) -> bool {
    matches!((left, right), ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>'))
}

fn score_mismatch(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn score_incomplete(s: String) -> u64 {
    let mut score = 0;

    for c in s.chars().rev() {
        score *= 5;

        score += match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("Unexpected char"),
        };
    }

    score
}


#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> &'static str {
        include_str!("../inputs/day10_example.txt")
    }

    #[test]
    fn determines_syntax_errors() {
        assert_eq!(
            check_syntax("(]"),
            Some(SyntaxError::MismatchedChunk('(', ']'))
        );
        assert_eq!(check_syntax("()"), None);
        assert_eq!(check_syntax("("), Some(SyntaxError::Incomplete("(".to_string())));
        assert_eq!(check_syntax("[{()"), Some(SyntaxError::Incomplete("[{".to_string())));
        assert_eq!(check_syntax("[])"), Some(SyntaxError::CloseBeforeOpen(')')));
    }

    #[test]
    fn verify_example_input_part1() {
        assert_eq!(26397, solve_part1(example_input()));
    }

    #[test]
    fn verify_example_input_part2() {
        assert_eq!(288957, solve_part2(example_input()));
    }
}
