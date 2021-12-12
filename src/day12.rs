use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = puzzle_input();
    println!("day12.part1.solution = {}", solve_part1(input));
    println!("day12.part2.solution = {}", solve_part2(input));
}

fn solve_part1(input: &str) -> usize {
    let g = parse_input(input);
    let paths = all_paths(
        &g,
        "start".to_string(),
        "end".to_string(),
        vec![vec!["start".to_string()]],
        false,
    );
    paths.len()
}

fn solve_part2(input: &str) -> usize {
    let g = parse_input(input);
    let paths = all_paths(
        &g,
        "start".to_string(),
        "end".to_string(),
        vec![vec!["start".to_string()]],
        true,
    );
    paths.len()
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day12.txt")
}

fn parse_input(input: &str) -> Graph {
    let lines: Vec<_> = input.lines().collect();

    let node_ids: HashSet<_> = lines.iter().flat_map(|&line| line.split('-')).collect();

    let adjacent_ids: HashSet<(&str, &str)> = lines
        .iter()
        .flat_map(|&line| {
            let (left, right) = line.split_once('-').unwrap();
            [(left, right), (right, left)]
        })
        .collect();

    let mut graph = Graph::new();

    for id in node_ids.iter() {
        graph.insert(id.to_string(), Neighbors::new());
    }

    for (left, right) in adjacent_ids {
        graph
            .get_mut(&left.to_string())
            .unwrap()
            .insert(right.to_string());
    }

    graph
}

fn all_paths(g: &Graph, from: Node, to: Node, paths: Vec<Path>, allow_double: bool) -> Vec<Path> {
    if paths.is_empty() || from == to {
        return paths;
    }

    g.get(&from)
        .unwrap()
        .iter()
        .flat_map(|neighbor| {
            let paths: Vec<Path> = paths
                .iter()
                .filter(|&path| {
                    if !allow_double {
                        is_big(neighbor) || !path.contains(neighbor)
                    } else {
                        (is_big(neighbor) || !has_double(path) || !path.contains(neighbor))
                            && neighbor != "start"
                    }
                })
                .map(|path| {
                    let mut path = path.clone();
                    path.push(neighbor.clone());
                    path
                })
                .collect();

            all_paths(g, neighbor.clone(), to.clone(), paths, allow_double)
        })
        .collect()
}

fn is_big(node: &Node) -> bool {
    node.chars().next().unwrap().is_uppercase()
}

fn has_double(path: &Path) -> bool {
    let small_ids: Vec<Node> = path.iter().cloned().filter(|n| !is_big(n)).collect();
    let dedup_ids: HashSet<Node> = small_ids.iter().cloned().collect();
    small_ids.len() != dedup_ids.len()
}

type Node = String;
type Neighbors = HashSet<Node>;
type Graph = HashMap<Node, Neighbors>;

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
