pub fn run(day: u32) {
    match day {
        1 => crate::day01::run(),
        2 => crate::day02::run(),
        3 => crate::day03::run(),
        4 => crate::day04::run(),
        5 => crate::day05::run(),
        6 => crate::day06::run(),
        _ => panic!("Unfinished day"),
    }
}
