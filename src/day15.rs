use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn run() {
    let input = puzzle_input();
    println!("day15.part1.solution = {}", solve_part1(input));
    println!("day15.part2.solution = {}", solve_part2(input));
}

fn solve_part1(input: &str) -> u32 {
    let grid = parse_input(input);
    find_cheapest_path(&grid)
}

fn solve_part2(input: &str) -> u32 {
    let grid = parse_input(input);
    let grid = embiggen_grid(grid);
    find_cheapest_path(&grid)
}

fn find_cheapest_path(grid: &[Vec<u32>]) -> u32 {
    let mut visited: Vec<Vec<bool>> = (0..grid.len()).map(|_| vec![false; grid.len()]).collect();
    let mut costs: Vec<Vec<Option<u32>>> =
        (0..grid.len()).map(|_| vec![None; grid.len()]).collect();
    costs[0][0] = Some(0);
    visited[0][0] = true;

    let mut fringe: BinaryHeap<Reverse<(u32, (usize, usize))>> = BinaryHeap::new();
    fringe.push(Reverse((0, (0, 0))));
    while let Some(Reverse((_, coord @ (x, y)))) = fringe.pop() {
        visited[x][y] = true;
        for neighbor @ (a, b) in neighboring_points(coord, grid.len()) {
            let path_cost = costs[x][y].unwrap() + grid[a][b];
            if !visited[a][b] || path_cost < costs[a][b].unwrap() {
                visited[a][b] = true;
                costs[a][b] = Some(path_cost);
                fringe.push(Reverse((path_cost, neighbor)));
            }
        }
    }

    costs[grid.len() - 1][grid.len() - 1].unwrap()
}

fn embiggen_grid(grid: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut big_grid = vec![vec![0; grid.len() * 5]; grid.len() * 5];

    let size = grid.len();

    for inc_row in 0..5 {
        for inc_col in 0..5 {
            for row in 0..size {
                for col in 0..size {
                    big_grid[row + size * inc_row][col + size * inc_col] =
                        inc_by(grid[row][col], (inc_row + inc_col) as u32);
                }
            }
        }
    }

    big_grid
}

fn inc_by(n: u32, inc: u32) -> u32 {
    let n = n + inc;
    if n > 9 {
        n - 9
    } else {
        n
    }
}

fn neighboring_points((x, y): (usize, usize), max: usize) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];

    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x < max - 1 {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < max - 1 {
        neighbors.push((x, y + 1));
    }

    neighbors
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day15.txt")
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u32)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> &'static str {
        include_str!("../inputs/day15_example.txt")
    }

    #[test]
    fn verify_example_input_part1() {
        assert_eq!(40, solve_part1(example_input()));
    }

    #[test]
    fn verify_example_input_part2() {
        assert_eq!(315, solve_part2(example_input()));
    }
}
