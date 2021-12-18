pub fn run() {
    let input = puzzle_input();
    let target = parse_input(input);
    let trajectories = all_trajectories(&target);
    println!("day17.part1.solution = {}", solve_part1(&trajectories));
    println!("day17.part2.solution = {}", solve_part2(&trajectories));
}

fn solve_part1(trajectories: &[(i32, i32)]) -> i32 {
    trajectories
        .iter()
        .map(|(_, dy)| max_height(*dy))
        .max()
        .unwrap()
}

fn solve_part2(trajectories: &[(i32, i32)]) -> usize {
    trajectories.len()
}

fn hits_target(dx: i32, dy: i32, target: &TargetArea) -> (bool, (i32, i32)) {
    let (mut dx, mut dy) = (dx, dy);
    let (mut x, mut y) = (0, 0);

    while x < target.x.end
        && y > target.y.start
        && !(target.x.contains(&x) && target.y.contains(&y))
    {
        x += dx;
        y += dy;

        if dx > 0 {
            dx -= 1;
        }
        dy -= 1;
    }

    (target.x.contains(&x) && target.y.contains(&y), (x, y))
}

fn max_height(dy: i32) -> i32 {
    let mut dy = dy;
    let mut y = 0;

    while dy > 0 {
        y += dy;
        dy -= 1;
    }

    y
}

fn all_trajectories(target: &TargetArea) -> Vec<(i32, i32)> {
    let dxs = valid_x_velocities(target);

    let mut trajectories = vec![];

    for &dx in &dxs {
        let min_dy = target.y.start;
        let max_dy = -target.y.start;
        for dy in (min_dy..=max_dy).rev() {
            let (hit, (_, _)) = hits_target(dx, dy, target);
            if hit {
                trajectories.push((dx, dy));
            }
        }
    }

    trajectories
}

fn valid_x_velocities(target: &TargetArea) -> Vec<i32> {
    (0..target.x.end)
        .filter(|&dx| intersects_x_target(dx, target))
        .collect()
}

fn x_steps(dx: i32, target: &TargetArea) -> Option<i32> {
    let mut dx = dx;
    let mut n = 0;
    let mut steps = 0;
    while n < target.x.end && dx > 0 {
        steps += 1;
        if target.x.contains(&n) {
            return Some(steps);
        }
        n += dx;
        dx -= 1;
    }

    None
}

fn intersects_x_target(dx: i32, target: &TargetArea) -> bool {
    x_steps(dx, target).is_some()
}

#[derive(Debug, PartialEq)]
struct TargetArea {
    pub x: std::ops::Range<i32>,
    pub y: std::ops::Range<i32>,
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day17.txt")
}

fn parse_input(input: &str) -> TargetArea {
    let filtered_input = input.replace(|c: char| !c.is_digit(10) && c != '-', " ");
    let mut nums = filtered_input
        .split_ascii_whitespace()
        .map(|token| token.parse::<i32>().unwrap());

    TargetArea {
        x: nums.next().unwrap()..nums.next().unwrap() + 1,
        y: nums.next().unwrap()..nums.next().unwrap() + 1,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> &'static str {
        include_str!("../inputs/day17_example.txt")
    }

    #[test]
    fn parses_target_areas() {
        let expected = TargetArea {
            x: 20..31,
            y: -10..-4,
        };

        assert_eq!(expected, parse_input(example_input()));
    }

    #[test]
    fn checks_intersections() {
        let target = parse_input(example_input());

        assert_eq!((true, (30, -6)), hits_target(9, 0, &target));
        assert_eq!((true, (21, -9)), hits_target(6, 3, &target));
        assert_eq!((false, (33, -9)), hits_target(17, -4, &target));
    }

    #[test]
    fn verify_example_input_part1() {
        let target = parse_input(example_input());
        let trajectories = all_trajectories(&target);
        assert_eq!(45, solve_part1(&trajectories));
    }

    #[test]
    fn verify_example_input_part2() {
        let target = parse_input(example_input());
        let trajectories = all_trajectories(&target);
        assert_eq!(112, solve_part2(&trajectories));
    }
}
