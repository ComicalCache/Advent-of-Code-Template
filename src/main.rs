#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::wildcard_imports, clippy::enum_glob_use)]

mod args;
mod days;

use args::{Args, DayOpt, PartOpt};
use clap::Parser;
use days::*;
use DayOpt::*;

macro_rules! exec_days {
    ($args:ident, $( $day:ident, $sol:ident ),*) => {
        match $args.day {
            $(
            $day => {
                let file = if let Some(path) = $args.file {
                    std::fs::read_to_string(&path)
                        .expect(format!("No file at `{path}`").as_str())
                } else {
                    days::read_file($day as usize)
                };
                let file = file.as_str();

                println!("{}", $day);
                if $args.part == PartOpt::P1 || $args.part == PartOpt::All {
                    println!("  Part 1 = {}", $sol::p1(file));
                }
                if $args.part == PartOpt::P2 || $args.part == PartOpt::All {
                    println!("  Part 2 = {}", $sol::p2(file));
                }
            }
            )*
        }
    };
}

fn main() {
    let args = Args::parse();
    exec_days!(args, D0, day0);
}
