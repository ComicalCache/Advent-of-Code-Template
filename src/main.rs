mod args;
use args::{Args, DayOpt, PartOpt};
use clap::Parser;

mod days;
use days::*;

macro_rules! exec_days {
    ($args:ident, $( $day:ident ),*) => {
        match $args.day {
            $(
            DayOpt::$day => {
                let file = if let Some(path) = $args.file {
                    std::fs::read_to_string(&path)
                        .expect(format!("Failed to open `{path}`").as_str())
                } else {
                    days::read_file(DayOpt::$day as usize)
                };
                match $args.part {
                    PartOpt::Part1 => println!("{}\n  Part 1 = {}", DayOpt::$day, days::$day::p1(file)),
                    PartOpt::Part2 => println!("{}\n  Part 2 = {}", DayOpt::$day, days::$day::p2(file)),
                    PartOpt::All => {
                        let (p1, p2) = $day::day(file);
                        println!("{}\n  Part 1 = {p1}\n  Part 2 = {p2}", DayOpt::$day);
                    }
                }
            }
            )*
        }
    };
}

fn main() {
    let args = Args::parse();

    exec_days!(args, Day0)
}
