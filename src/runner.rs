pub fn run(day: u32) {
    match day {
        1 => crate::day01::run(),
        2 => crate::day02::run(),
        3 => crate::day03::run(),
        4 => crate::day04::run(),
        5 => crate::day05::run(),
        6 => crate::day06::run(),
        7 => crate::day07::run(),
        8 => crate::day08::run(),
        9 => crate::day09::run(),
        10 => crate::day10::run(),
        11 => crate::day11::run(),
        12 => crate::day12::run(),
        13 => crate::day13::run(),
        14 => crate::day14::run(),
        15 => crate::day15::run(),
        16 => crate::day16::run(),
        _ => panic!("Unfinished day"),
    }
}
