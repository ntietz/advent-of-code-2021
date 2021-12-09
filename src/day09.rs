use std::collections::HashSet;

pub fn run() {
    let input = puzzle_input();
    println!("day09.part1.solution = {}", solve_part1(input));
    println!("day09.part2.solution = {}", solve_part2(input));
}

fn solve_part1(input: &str) -> u32 {
    let heightmap = Heightmap::from_input(input);
    heightmap
        .low_points()
        .iter()
        .map(|&(row, col)| (heightmap.get(row, col) + 1) as u32)
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    let heightmap = Heightmap::from_input(input);
    let mut basin_sizes: Vec<_> = heightmap
        .low_points()
        .iter()
        .map(|&(row, col)| heightmap.basin_size(row, col))
        .collect();
    basin_sizes.sort_unstable_by(|a, b| b.cmp(a));

    basin_sizes.iter().take(3).product()
}

struct Heightmap {
    heights: Vec<u8>,
    rows: usize,
    columns: usize,
}

impl Heightmap {
    pub fn from_input(input: &str) -> Self {
        let heights: Vec<_> = input
            .trim()
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8))
            .collect();

        let columns = input.lines().next().unwrap().len();
        let rows = heights.len() / columns;

        Heightmap {
            heights,
            rows,
            columns,
        }
    }

    pub fn low_points(&self) -> Vec<(usize, usize)> {
        (0..self.rows)
            .flat_map(|row| -> Vec<(usize, usize)> {
                (0..self.columns)
                    .filter_map(|col| {
                        if self.is_low_point(row, col) {
                            Some((row, col))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect()
    }

    /// Computes the basin size for a provided low point. This relies on the
    /// assumption that each basin is separated by a ridge of 9s, so we can do
    /// a breadth-first search outward until we find the ridge.
    pub fn basin_size(&self, row: usize, col: usize) -> u32 {
        let mut prev_fringe: HashSet<(usize, usize)> = HashSet::new();
        let mut fringe: HashSet<_> = HashSet::from([(row, col)]);

        while fringe != prev_fringe {
            prev_fringe = fringe.clone();

            for &(row, col) in &prev_fringe {
                if (row > 0) && self.get(row - 1, col) != 9 {
                    fringe.insert((row - 1, col));
                }
                if (row < self.rows - 1) && self.get(row + 1, col) != 9 {
                    fringe.insert((row + 1, col));
                }
                if (col > 0) && self.get(row, col - 1) != 9 {
                    fringe.insert((row, col - 1));
                }
                if (col < self.columns - 1) && self.get(row, col + 1) != 9 {
                    fringe.insert((row, col + 1));
                }
            }
        }

        fringe.len() as u32
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.heights[col + row * self.columns]
    }

    pub fn is_low_point(&self, row: usize, col: usize) -> bool {
        let height = self.get(row, col);
        (row == 0 || height < self.get(row - 1, col))
            && (row == self.rows - 1 || height < self.get(row + 1, col))
            && (col == 0 || height < self.get(row, col - 1))
            && (col == self.columns + 1 || height < self.get(row, col + 1))
    }
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day09.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> &'static str {
        include_str!("../inputs/day09_example.txt")
    }

    #[test]
    fn parses_input_correctly() {
        let heightmap = Heightmap::from_input(example_input());
        assert_eq!(5, heightmap.rows);
        assert_eq!(10, heightmap.columns);
        assert_eq!(2, heightmap.get(0, 0));
        assert_eq!(8, heightmap.get(3, 0));
        assert_eq!(1, heightmap.get(0, 1));
    }

    #[test]
    fn checks_low_points_correctly() {
        let heightmap = Heightmap::from_input(example_input());
        assert_eq!(true, heightmap.is_low_point(0, 1));
        assert_eq!(true, heightmap.is_low_point(2, 2));
        assert_eq!(false, heightmap.is_low_point(0, 0));
        assert_eq!(false, heightmap.is_low_point(4, 9));
    }

    #[test]
    fn finds_basin_sizes() {
        let heightmap = Heightmap::from_input(example_input());
        assert_eq!(3, heightmap.basin_size(0, 1));
        assert_eq!(14, heightmap.basin_size(2, 2));
    }

    #[test]
    fn verify_example_input_part1() {
        assert_eq!(15, solve_part1(example_input()));
    }

    #[test]
    fn verify_example_input_part2() {
        assert_eq!(1134, solve_part2(example_input()));
    }
}
