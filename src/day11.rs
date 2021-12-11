pub fn run() {
    let input = puzzle_input();
    println!("day11.part1.solution = {}", solve_part1(input));
    println!("day11.part2.solution = {}", solve_part2(input));
}

fn solve_part1(input: &str) -> usize {
    let mut grid = OctoGrid::parse(input);

    let mut total = 0;

    for _ in 0..100 {
        total += grid.step();
    }

    total
}

fn solve_part2(input: &str) -> usize {
    let mut grid = OctoGrid::parse(input);
    let mut round = 1;

    while grid.step() != 100 {
        round += 1;
    }

    return round;
}

/// Represents all the octopi in the cavern.
struct OctoGrid {
    energy_levels: Vec<u32>,
    rows: usize,
    cols: usize,
}

impl OctoGrid {
    pub fn parse(input: &str) -> Self {
        let energy_levels: Vec<_> = input.chars().flat_map(|c| c.to_digit(10)).collect();
        let size = (energy_levels.len() as f64).sqrt() as usize;
        assert_eq!(size*size, energy_levels.len());
        OctoGrid {
            energy_levels,
            rows: size,
            cols: size,
        }
    }

    /// Performs one time-step and returns the number of flashes from this step.
    pub fn step(&mut self) -> usize {
        for level in self.energy_levels.iter_mut() {
            *level += 1;
        }

        let mut changed = true;
        let mut flashed: Vec<bool> = vec![false; self.energy_levels.len()];

        while changed {
            changed = false;

            for idx in 0..(self.energy_levels.len()) {
                if self.energy_levels[idx] > 9 && !flashed[idx] {
                    changed = true;
                    flashed[idx] = true;

                    for neighbor in self.neighbors(idx) {
                        self.energy_levels[neighbor] += 1;
                    }
                }
            }
        }

        for level in self.energy_levels.iter_mut() {
            if *level > 9 {
                *level = 0;
            }
        }

        flashed.iter().filter(|&b| *b).count()
    }

    fn neighbors(&self, idx: usize) -> Vec<usize> {
        let col = (idx % self.cols) as i32;
        let row = ((idx - col as usize) / self.rows) as i32;

        let mut neighbors = vec![];

        for d_row in -1..=1 {
            for d_col in -1..=1 {
                let (n_row, n_col) = (d_row + row, d_col + col);
                if 0 <= n_row && n_row < self.rows as i32 && 0 <= n_col && n_col < self.cols as i32 && (n_row, n_col) != (row, col) {
                    neighbors.push((n_row as usize * self.rows) + n_col as usize);
                }
            }
        }

        neighbors
    }
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day11.txt")
}


#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> &'static str {
        include_str!("../inputs/day11_example.txt")
    }

    #[test]
    fn verify_step_behavior() {
        let input = "
            11111
            19991
            19191
            19991
            11111
            ";
        let mut grid = OctoGrid::parse(input);

        assert_eq!(vec![1,5,6], grid.neighbors(0));
        assert_eq!(vec![0, 1, 2, 5, 7, 10, 11, 12], grid.neighbors(6));

        assert_eq!(9, grid.step());
        assert_eq!(0, grid.step());
    }

    #[test]
    fn verify_example_input_part1() {
        assert_eq!(1656, solve_part1(example_input()));
    }

    #[test]
    fn verify_example_input_part2() {
        assert_eq!(195, solve_part2(example_input()));
    }
}
