pub fn run(day: u32) {
    match day {
        1 => crate::day01::run(),
        2 => crate::day02::run(),
        _ => panic!("Unfinished day"),
    }
}
