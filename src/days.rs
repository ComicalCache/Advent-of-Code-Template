pub mod day0;

/// Reads input of respective day. Files must be named the same as
/// the respective day in `DayOpt` and `DayImpl` struct
pub fn read_file(day: usize) -> String {
    std::fs::read_to_string(format!("inputs/day{day}.txt"))
        .unwrap_or_else(|_| panic!("Failed to open input for day {day}"))
}
