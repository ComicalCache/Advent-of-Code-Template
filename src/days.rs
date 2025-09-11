macro_rules! days {
    ($($p:ident, $d:ident),*) => {
        $(
            mod $p;
            pub use $p::$d;
        )*
    }
}

days!(day0, Day0);

/// Reads input of respective day. Files must be named the same as
/// the respective day in DayOpt and DayImpl struct
pub fn read_file(day: usize) -> String {
    std::fs::read_to_string(format!("inputs/day{day}.txt"))
        .expect(format!("Failed to open input for day {day}").as_str())
}

pub trait DayImpl {
    type Output1;
    type Output2;

    /// Solution for part 1
    fn p1<S: AsRef<str>>(_: S) -> Self::Output1 {
        unimplemented!()
    }
    /// Solution for part 2
    fn p2<S: AsRef<str>>(_: S) -> Self::Output2 {
        unimplemented!()
    }
    /// Solution for both parts simultaneously
    fn day<S: AsRef<str>>(input: S) -> (Self::Output1, Self::Output2) {
        (Self::p1(input.as_ref()), Self::p2(input))
    }
}
