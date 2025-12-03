use std::fmt::Display;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(about = "Simple Advent of Code runner", long_about = None)]
pub struct Args {
    /// Day to solve
    #[arg(short, long, value_name = "DAY", value_enum)]
    pub day: DayOpt,

    /// Part to solve
    #[arg(short, long, value_name = "PART", value_enum, default_value_t = PartOpt::All)]
    pub part: PartOpt,

    /// File to use as input (e.g. for debugging)
    #[arg(short, long, value_name = "PATH")]
    pub file: Option<String>,
}

#[derive(Clone, PartialEq, Eq, ValueEnum)]
pub enum PartOpt {
    /// Part 1
    P1,
    /// Part 2
    P2,
    /// Both parts
    All,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum DayOpt {
    /// Day 0 (example)
    D0 = 0,
}

impl Display for DayOpt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day {:02}", *self as usize)
    }
}
