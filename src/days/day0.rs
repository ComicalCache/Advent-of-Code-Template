use super::DayImpl;

pub struct Day0;

impl DayImpl for Day0 {
    type Output1 = usize;
    type Output2 = usize;

    fn p1<S: AsRef<str>>(input: S) -> Self::Output1 {
        let res = input
            .as_ref()
            .lines()
            .next()
            .expect("Expected day 0 to not have empty input");
        usize::from_str_radix(res, 10).expect("Expected day 0 to have valid numbers as input")
    }

    fn p2<S: AsRef<str>>(input: S) -> Self::Output2 {
        let res = input
            .as_ref()
            .lines()
            .next_back()
            .expect("Expected day 0 to not have empty input");
        usize::from_str_radix(res, 10).expect("Expected day 0 to have valid numbers as input")
    }
}
