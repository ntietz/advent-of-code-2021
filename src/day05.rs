use std::collections::HashSet;

pub fn run() {
    let input = puzzle_input();
    println!("day05.part1.solution = {}", solve_part1(input));
    println!("day05.part2.solution = {}", solve_part2(input));
}

fn solve_part1(input: &str) -> usize {
    let line_segments = parse_input(input);

    let mut points: Vec<_> = line_segments
        .iter()
        .map(|x| x.points_covered_hv())
        .flatten()
        .collect();
    points.sort_unstable();

    let intersections: HashSet<_> = points.windows(2).filter(|w| w[0] == w[1]).collect();

    intersections.len()
}

fn solve_part2(input: &str) -> usize {
    let line_segments = parse_input(input);

    let mut points: Vec<_> = line_segments
        .iter()
        .map(|x| x.points_covered())
        .flatten()
        .collect();
    points.sort_unstable();

    let intersections: HashSet<_> = points.windows(2).filter(|w| w[0] == w[1]).collect();

    intersections.len()
}

#[derive(Debug)]
struct LineSegment {
    a: (i32, i32),
    b: (i32, i32),
}

impl LineSegment {
    pub fn parse(input: &str) -> LineSegment {
        input
            .split_once(" -> ")
            .map(|(left, right)| LineSegment {
                a: parse_tuple(left),
                b: parse_tuple(right),
            })
            .unwrap()
    }

    pub fn points_covered(&self) -> Vec<(i32, i32)> {
        let mut points = self.points_covered_hv();
        points.extend(self.points_covered_diag());
        points
    }

    pub fn points_covered_diag(&self) -> Vec<(i32, i32)> {
        let a = std::cmp::min(self.a, self.b);
        let b = std::cmp::max(self.a, self.b);

        let dx = b.0 - a.0;
        let dy = b.1 - a.1;

        if dx.abs() == dy.abs() {
            let xs: Vec<_> = (a.0..=b.0).collect();
            let ys: Vec<_> = if dy > 0 {
                (a.1..=b.1).collect()
            } else {
                (b.1..=a.1).rev().collect()
            };
            xs.iter().cloned().zip(ys.iter().cloned()).collect()
        } else {
            vec![]
        }
    }

    pub fn points_covered_hv(&self) -> Vec<(i32, i32)> {
        let a = std::cmp::min(self.a, self.b);
        let b = std::cmp::max(self.a, self.b);

        if a.0 == b.0 {
            (a.1..=b.1).map(|y| (a.0, y)).collect()
        } else if a.1 == b.1 {
            (a.0..=b.0).map(|x| (x, a.1)).collect()
        } else {
            vec![]
        }
    }
}

fn parse_tuple(input: &str) -> (i32, i32) {
    // No error handling because we're just covering the provided input.
    // In practice, let's not assume that everything works out.
    let (left, right) = input.split_once(",").unwrap();
    (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
}

fn parse_input(input: &str) -> Vec<LineSegment> {
    input.lines().map(LineSegment::parse).collect()
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day05.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> &'static str {
        include_str!("../inputs/day05_example.txt")
    }

    #[test]
    fn can_parse_input() {
        let line_segments = parse_input(example_input());

        assert_eq!(10, line_segments.len());
        assert_eq!((0, 9), line_segments[0].a);
        assert_eq!((5, 9), line_segments[0].b);
    }

    #[test]
    fn verify_example_input_part1() {
        assert_eq!(5, solve_part1(example_input()));
    }

    #[test]
    fn verify_example_input_part2() {
        assert_eq!(12, solve_part2(example_input()));
    }
}
