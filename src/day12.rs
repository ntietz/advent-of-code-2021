use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub fn run() {
    let input = puzzle_input();
    println!("day12.part1.solution = {}", solve_part1(input));
    println!("day12.part2.solution = {}", solve_part2(input));
}

fn solve_part1(input: &'static str) -> usize {
    let g = parse_input(input);
    let start = Rc::new("start");
    let end = Rc::new("end");
    path_count(&g, start.clone(), end, vec![start], false)
}

fn solve_part2(input: &'static str) -> usize {
    let g = parse_input(input);
    let start = Rc::new("start");
    let end = Rc::new("end");
    path_count(&g, start.clone(), end, vec![start], true)
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day12.txt")
}

fn parse_input(input: &'static str) -> Graph {
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
        graph.insert(Rc::new(id), Neighbors::new());
    }

    for (left, right) in adjacent_ids {
        graph.get_mut(&Rc::new(left)).unwrap().insert(Rc::new(right));
    }

    graph
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
                is_big(neighbor) || !path.contains(neighbor)
            } else {
                (is_big(neighbor) || !has_double(&path) || !path.contains(neighbor))
                    && **neighbor != "start"
            }
        })
        .map(|neighbor| {
            let mut path = path.clone();
            path.push(neighbor.clone());
            path_count(g, neighbor.clone(), to.clone(), path, allow_double)
        })
        .sum()
}

fn is_big(node: &Node) -> bool {
    node.chars().next().unwrap().is_uppercase()
}

fn has_double(path: &Path) -> bool {
    let small_ids: Vec<Node> = path.iter().cloned().filter(|n| !is_big(n)).collect();
    let dedup_ids: HashSet<Node> = small_ids.iter().cloned().collect();
    small_ids.len() != dedup_ids.len()
}

type Node = Rc<&'static str>;
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
