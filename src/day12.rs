use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};

pub fn run() {
    let input = puzzle_input();
    println!("day12.part1.solution = {}", solve_part1(input));
    println!("day12.part2.solution = {}", solve_part2(input));
}

fn solve_part1(input: &'static str) -> usize {
    let g = parse_input(input);
    let start = compute_hash("start");
    let end = compute_hash("end");
    path_count(&g, start, end, vec![start], false)
}

fn solve_part2(input: &'static str) -> usize {
    let g = parse_input(input);
    let start = compute_hash("start");
    let end = compute_hash("end");
    path_count(&g, start, end, vec![start], true)
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day12.txt")
}

fn parse_input(input: &'static str) -> Graph {
    let lines: Vec<_> = input.lines().collect();

    let node_ids: BTreeSet<Node> = lines
        .iter()
        .flat_map(|&line| line.split('-'))
        .map(|s| compute_hash(s))
        .collect();

    let adjacent_ids: BTreeSet<(Node, Node)> = lines
        .iter()
        .flat_map(|&line| {
            let (left, right) = line.split_once('-').unwrap();
            [
                (compute_hash(left), compute_hash(right)),
                (compute_hash(right), compute_hash(left)),
            ]
        })
        .collect();

    let mut graph = Graph::new();

    for &id in node_ids.iter() {
        graph.insert(id, Neighbors::new());
    }

    for (left, right) in adjacent_ids {
        graph.get_mut(&left).unwrap().insert(right);
    }

    graph
}

fn compute_hash(s: &str) -> u64 {
    let mut h = DefaultHasher::new();
    let big = is_big(s);
    s.hash(&mut h);
    (h.finish() << 1) | (if big { 1 } else { 0 })
}

fn path_count(g: &Graph, from: Node, to: Node, path: Path, allow_double: bool) -> usize {
    if from == to {
        return 1;
    }

    let neighbors = g.get(&from).unwrap();

    neighbors
        .iter()
        .filter(|&neighbor| {
            if !allow_double {
                big_bit_set(*neighbor) || !path.contains(neighbor)
            } else {
                (big_bit_set(*neighbor) || !has_double(&path) || !path.contains(neighbor))
                    && *neighbor != compute_hash("start")
            }
        })
        .map(|neighbor| {
            let mut path = path.clone();
            path.push(*neighbor);
            path_count(g, *neighbor, to, path, allow_double)
        })
        .sum()
}

fn is_big(node: &str) -> bool {
    node.chars().next().unwrap().is_uppercase()
}

fn big_bit_set(n: Node) -> bool {
    n % 2 == 1
}

fn has_double(path: &Path) -> bool {
    let small_ids: Vec<Node> = path.iter().cloned().filter(|n| !big_bit_set(*n)).collect();
    let dedup_ids: Neighbors = small_ids.iter().cloned().collect();
    small_ids.len() != dedup_ids.len()
}

type Node = u64;
type Neighbors = BTreeSet<Node>;
type Graph = BTreeMap<Node, Neighbors>;

type Path = Vec<Node>;

#[cfg(test)]
mod test {
    use super::*;

    fn example_input1() -> &'static str {
        include_str!("../inputs/day12_example1.txt")
    }

    fn example_input2() -> &'static str {
        include_str!("../inputs/day12_example2.txt")
    }

    fn example_input3() -> &'static str {
        include_str!("../inputs/day12_example3.txt")
    }

    #[test]
    fn verify_example_input_part1() {
        assert_eq!(10, solve_part1(example_input1()));
        assert_eq!(19, solve_part1(example_input2()));
        assert_eq!(226, solve_part1(example_input3()));
    }

    #[test]
    fn verify_example_input_part2() {
        assert_eq!(36, solve_part2(example_input1()));
        assert_eq!(103, solve_part2(example_input2()));
        assert_eq!(3509, solve_part2(example_input3()));
    }
}
