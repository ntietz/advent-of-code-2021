use std::collections::HashSet;

pub fn run() {
    let input = puzzle_input();
    println!("day13.part1.solution = {}", solve_part1(input));
    solve_part2(input);
    println!("day13.part2.solution = <see above>");
}

fn solve_part1(input: &str) -> usize {
    let instructions = Instructions::parse(input);
    apply_fold(instructions.points, instructions.folds[0]).len()
}

fn solve_part2(input: &str) {
    let instructions = Instructions::parse(input);

    let final_points = instructions
        .folds
        .iter()
        .fold(instructions.points, |points, fold| {
            apply_fold(points, *fold)
        });

    print_points(&final_points);
}

fn print_points(points: &HashSet<Point>) {
    let rows = points.iter().map(|&(_, y)| y).max().unwrap() + 1;
    let cols = points.iter().map(|&(x, _)| x).max().unwrap() + 1;

    for row in 0..rows {
        let s: String = (0..cols)
            .map(|col| {
                if points.contains(&(col, row)) {
                    '#'
                } else {
                    ' '
                }
            })
            .collect();
        println!("{}", s);
    }
}

fn apply_fold(points: HashSet<Point>, fold: Fold) -> HashSet<Point> {
    points
        .iter()
        .map(|&(x, y)| match fold {
            Fold::X(amt) => {
                if x > amt {
                    (amt - (x - amt), y)
                } else {
                    (x, y)
                }
            }
            Fold::Y(amt) => {
                if y > amt {
                    (x, amt - (y - amt))
                } else {
                    (x, y)
                }
            }
        })
        .collect()
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day13.txt")
}

type Point = (i64, i64);

#[derive(Debug, Clone, Copy)]
enum Fold {
    X(i64),
    Y(i64),
}

#[derive(Debug, Clone)]
struct Instructions {
    pub points: HashSet<Point>,
    pub folds: Vec<Fold>,
}

impl Instructions {
    pub fn parse(input: &str) -> Self {
        let points = input
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let (left, right) = line.split_once(',').unwrap();
                (left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap())
            })
            .collect();

        let folds = input
            .lines()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .map(|line| {
                let (left, right) = line.split_once('=').unwrap();
                let amt = right.parse::<i64>().unwrap();

                match left.chars().last().unwrap() {
                    'x' => Fold::X(amt),
                    'y' => Fold::Y(amt),
                    _ => panic!("Unrecognized fold direction"),
                }
            })
            .collect();

        Instructions { points, folds }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> &'static str {
        include_str!("../inputs/day13_example.txt")
    }

    #[test]
    fn verify_example_input_part1() {
        assert_eq!(17, solve_part1(example_input()));
    }

    #[test]
    fn verify_multiple_folds() {
        let instructions = Instructions::parse(example_input());
        let first_fold = apply_fold(instructions.points, instructions.folds[0]);
        let second_fold = apply_fold(first_fold, instructions.folds[1]);
        assert_eq!(16, second_fold.len());
    }
}
