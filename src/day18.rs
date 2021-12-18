pub fn run() {
    let input = puzzle_input();
    println!("day18.part1.solution = {}", solve_part1(input));
    println!("day18.part2.solution = {}", solve_part2(input));
}

fn solve_part1(input: &str) -> i64 {
    let sum = input
        .lines()
        .map(|line| parse_number(line.trim()))
        .reduce(|left, right| reduce(&add(&left, &right)))
        .unwrap();
    magnitude(&sum)
}

fn solve_part2(input: &str) -> i64 {
    let nums: Vec<_> = input
        .lines()
        .map(|line| parse_number(line.trim()))
        .collect();

    let mut max = 0;

    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }

            let m = magnitude(&reduce(&add(&nums[i], &nums[j])));
            if m > max {
                max = m;
            }
        }
    }

    max
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day18.txt").trim()
}

type Depth = u8;
type Value = i64;
type Element = (Value, Depth);
type Number = Vec<Element>;

fn parse_number(input: &str) -> Number {
    let mut i = 0;
    let mut depth = 0;

    let mut number: Number = vec![];

    while i < input.len() {
        match input.as_bytes()[i] as char {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' => {}
            _ => {
                let end_i = input[i..]
                    .find(|c: char| matches!(c as char, ',' | ']' | '['))
                    .unwrap()
                    + i;
                let n = input[i..end_i].parse::<Value>().unwrap();
                number.push((n, depth));
                i = end_i - 1;
            }
        }

        i += 1;
    }

    number
}

fn reduce(num: &Number) -> Number {
    let mut reduced = num.clone();
    let mut changed = true;

    while changed {
        changed = false;

        for i in 0..(reduced.len() - 1) {
            if reduced[i].1 == 5 && reduced[i + 1].1 == 5 {
                if i > 0 {
                    reduced[i - 1].0 += reduced[i].0;
                }
                if i < reduced.len() - 2 {
                    reduced[i + 2].0 += reduced[i + 1].0;
                }

                reduced.remove(i + 1);
                reduced[i].0 = 0;
                reduced[i].1 -= 1;

                changed = true;
                break;
            }
        }

        if !changed {
            for i in 0..(reduced.len()) {
                if reduced[i].0 >= 10 {
                    let left = reduced[i].0 / 2;
                    let right = reduced[i].0 / 2 + (reduced[i].0 % 2);

                    reduced[i].0 = left;
                    reduced[i].1 += 1;

                    reduced.insert(i + 1, (right, reduced[i].1));

                    changed = true;
                    break;
                }
            }
        }
    }

    reduced
}

fn magnitude(num: &Number) -> i64 {
    let mut m = num.clone();

    fn helper(num: &mut Number, depth: u8) -> bool {
        for i in 0..(num.len() - 1) {
            if num[i].1 == depth && num[i + 1].1 == depth {
                num[i].0 = 3 * num[i].0 + 2 * num[i + 1].0;
                num[i].1 -= 1;

                num.remove(i + 1);

                return true;
            }
        }
        false
    }

    let max_depth = num.iter().map(|(_, d)| *d).max().unwrap();

    for depth in (1..=max_depth).rev() {
        while helper(&mut m, depth) {}
    }

    m[0].0
}

fn add(left: &Number, right: &Number) -> Number {
    let mut sum = left.clone();
    sum.extend(right.iter());

    for (_, d) in sum.iter_mut() {
        *d += 1;
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> &'static str {
        include_str!("../inputs/day18_example.txt")
    }

    #[test]
    fn verify_magnitude() {
        assert_eq!(
            1384,
            magnitude(&parse_number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"))
        );
    }

    #[test]
    fn verify_example_input_part1() {
        assert_eq!(4140, solve_part1(example_input()));
    }

    #[test]
    fn verify_example_input_part2() {
        assert_eq!(3993, solve_part2(example_input()));
    }
}
