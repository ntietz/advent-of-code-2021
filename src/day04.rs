pub fn run() {
    let input = puzzle_input();
    println!("day04.part1.solution = {}", solve_part1(input));
    println!("day04.part2.solution = {}", solve_part2(input));
}

fn solve_part1(input: &str) -> u32 {
    let (nums, boards) = parse_input(input);
    let win = boards
        .iter()
        .filter_map(|board| play(&mut board.clone(), &nums))
        .min_by_key(|x| x.moves)
        .unwrap();

    win.score * nums[win.moves]
}

fn solve_part2(input: &str) -> u32 {
    let (nums, boards) = parse_input(input);
    let win = boards
        .iter()
        .filter_map(|board| play(&mut board.clone(), &nums))
        .max_by_key(|x| x.moves)
        .unwrap();

    win.score * nums[win.moves]
}

#[derive(Clone, Debug)]
struct BingoBoard {
    spaces: Vec<u32>,
    drawn: Vec<bool>,
}

impl BingoBoard {
    pub fn from_spaces(spaces: &[u32]) -> BingoBoard {
        assert_eq!(spaces.len(), 25);
        BingoBoard {
            spaces: spaces.into(),
            drawn: vec![false; 25],
        }
    }

    pub fn is_winning(&self) -> bool {
        (0..5).any(|i| complete(i, 5, &self.drawn) || complete(i * 5, 1, &self.drawn))
    }

    pub fn mark(&mut self, num: u32) {
        if let Some(i) = self.spaces.iter().position(|x| *x == num) {
            self.drawn[i] = true;
        }
    }

    pub fn score(&self) -> u32 {
        self.spaces
            .iter()
            .zip(self.drawn.iter())
            .filter(|(_, &d)| !d)
            .map(|(&s, _)| s)
            .sum()
    }
}

fn complete(idx: usize, inc: usize, drawn: &[bool]) -> bool {
    (idx..).step_by(inc).take(5).all(|i| drawn[i])
}

struct Win {
    pub moves: usize,
    pub score: u32,
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day04.txt")
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<BingoBoard>) {
    let (nums_line, cards_input) = input.split_once("\n").unwrap();

    let nums = nums_line
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    let board_nums: Vec<_> = cards_input
        .split_ascii_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    let boards = board_nums.chunks(25).map(BingoBoard::from_spaces).collect();

    (nums, boards)
}

fn play(board: &mut BingoBoard, nums: &[u32]) -> Option<Win> {
    for (i, &num) in nums.iter().enumerate() {
        board.mark(num);
        if board.is_winning() {
            return Some(Win {
                moves: i,
                score: board.score(),
            });
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> &'static str {
        include_str!("../inputs/day04_example.txt")
    }

    #[test]
    fn can_parse_input() {
        let (nums, boards) = parse_input(example_input());
        assert_eq!(27, nums.len());
        assert_eq!(3, boards.len());
    }

    #[test]
    fn verify_example_input_part1() {
        assert_eq!(4512, solve_part1(example_input()))
    }

    #[test]
    fn verify_example_input_part2() {
        assert_eq!(1924, solve_part2(example_input()))
    }
}
