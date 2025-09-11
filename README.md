# Advent of Code Template
Simple Advent of Code template. Use `-h` for brief or `--help` for more elaborate usage information.

## Add new day

> Check out the existing "Day 0" example.

To add a new day (in the following, all occurances of `N` are to be replaced with the respective day's number):
1. Add a new enum variant to `DayOpt` in `args.rs` using the naming convention `DayN` (no leading 0s!). Make sure that each day has its respective day number as value (Day7 = 7)!
2. Copy the `day0.rs` template (rename the file!), implement the solution and put your input in `inputs/dayN.txt` (no leading 0s!)
3. Add `dayN` and `DayN` to the `days!` macro in `days.rs`
4. Add the new `DayN` to the `exec_days!` macro in `main.rs`
