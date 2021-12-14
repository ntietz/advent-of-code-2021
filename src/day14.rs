use std::collections::BTreeMap;

pub fn run() {
    let input = puzzle_input();
    println!("day14.part1.solution = {}", solve_part1(input));
    println!("day14.part2.solution = {}", solve_part2(input));
}

fn solve_part1(input: &str) -> u64 {
    solve(input, 10)
}

fn solve_part2(input: &str) -> u64 {
    solve_pairs(input, 40)
}

fn solve(input: &str, steps: usize) -> u64 {
    let (template, rules) = parse_input(input);
    let mut template = template;

    for _ in 0..steps {
        template = step(template, &rules);
    }

    let mut counts: BTreeMap<char, u64> = BTreeMap::new();
    for c in template.chars() {
        counts.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }

    let highest = counts
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(_, v)| v)
        .unwrap();
    let lowest = counts
        .iter()
        .min_by(|a, b| a.1.cmp(b.1))
        .map(|(_, v)| v)
        .unwrap();

    highest - lowest
}

fn step(template: String, rules: &Rules) -> String {
    let mut result = String::from("");

    for pair @ (a, _) in template.chars().zip(template.chars().skip(1)) {
        result.push(a);
        if let Some(&x) = rules.get(&pair) {
            result.push(x);
        }
    }
    result.push(template.chars().last().unwrap());

    result
}

fn solve_pairs(input: &str, steps: usize) -> u64 {
    let (template, rules) = parse_input(input);
    let mut pairs = to_pairs(&template);

    for _ in 0..steps {
        pairs = step_pairs(pairs, &rules);
    }

    let mut counts: BTreeMap<char, u64> = BTreeMap::new();

    let mut pairs_iter = pairs.iter();
    let first = pairs_iter.next().unwrap();
    counts
        .entry(first.0 .0)
        .and_modify(|e| *e += first.1)
        .or_insert(*first.1);
    counts
        .entry(first.0 .1)
        .and_modify(|e| *e += first.1)
        .or_insert(*first.1);

    for (&(a, b), &count) in pairs_iter {
        counts.entry(a).and_modify(|e| *e += count).or_insert(count);
        counts.entry(b).and_modify(|e| *e += count).or_insert(count);
    }
    counts
        .entry(template.chars().next().unwrap())
        .and_modify(|e| *e += 1);
    counts
        .entry(template.chars().last().unwrap())
        .and_modify(|e| *e += 1);

    let highest = counts
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(_, v)| v)
        .unwrap()
        / 2;
    let lowest = counts
        .iter()
        .min_by(|a, b| a.1.cmp(b.1))
        .map(|(_, v)| v)
        .unwrap()
        / 2;

    highest - lowest
}

fn step_pairs(pairs: BTreeMap<(char, char), u64>, rules: &Rules) -> BTreeMap<(char, char), u64> {
    let mut result = BTreeMap::new();

    for (pair @ (a, b), count) in pairs {
        if let Some(&c) = rules.get(&pair) {
            result
                .entry((a, c))
                .and_modify(|e| *e += count)
                .or_insert(count);
            result
                .entry((c, b))
                .and_modify(|e| *e += count)
                .or_insert(count);
        } else {
            result
                .entry(pair)
                .and_modify(|e| *e += count)
                .or_insert(count);
        }
    }

    result
}

fn to_pairs(template: &str) -> BTreeMap<(char, char), u64> {
    let mut counts: BTreeMap<(char, char), u64> = BTreeMap::new();

    for pair in template.chars().zip(template.chars().skip(1)) {
        counts.entry(pair).and_modify(|e| *e += 1).or_insert(1);
    }

    counts
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day14.txt")
}

fn parse_input(input: &str) -> (String, Rules) {
    let template = input.lines().next().unwrap();

    let rules = input
        .lines()
        .skip(2)
        .map(|line| {
            let (left, right) = line.split_once(" -> ").unwrap();
            let mut chars = left.chars();
            (
                (chars.next().unwrap(), chars.next().unwrap()),
                right.chars().next().unwrap(),
            )
        })
        .collect();

    (template.to_string(), rules)
}

type Rules = BTreeMap<(char, char), char>;

#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> &'static str {
        include_str!("../inputs/day14_example.txt")
    }

    #[test]
    fn verify_one_step() {
        let (template, rules) = parse_input(example_input());
        assert_eq!("NCNBCHB", step(template, &rules));
    }

    #[test]
    fn verify_pairwise_is_equivalent() {
        let input = example_input();
        assert_eq!(solve(input, 1), solve_pairs(input, 1));
        assert_eq!(solve(input, 5), solve_pairs(input, 5));
    }

    #[test]
    fn verify_example_input_part1() {
        assert_eq!(1588, solve_part1(example_input()));
    }

    #[test]
    fn verify_example_input_part2() {
        assert_eq!(2188189693529, solve_part2(example_input()));
    }
}
