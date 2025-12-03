pub fn p1(input: &str) -> usize {
    let res = input
        .lines()
        .next()
        .expect("Expected day 0 to not have empty input");
    res.parse()
        .expect("Expected day 0 to have valid numbers as input")
}

pub fn p2(input: &str) -> usize {
    let res = input
        .lines()
        .next_back()
        .expect("Expected day 0 to not have empty input");
    res.parse()
        .expect("Expected day 0 to have valid numbers as input")
}
