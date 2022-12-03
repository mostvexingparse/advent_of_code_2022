# Advent of code 2022

My AOC2022 solutions in (nightly) Rust.

## Project structure

One subproject for each day, with a binary for each part, shared code and tests in `lib.rs`. The subprojects also have a `README.md` wich contains the assignment and a file named `input.txt` with the input data.

The tests are derived from examples givin in the assignments.

All suprojects are part of a workspace, so they can be built or tested at once: 

~~~bash
$ cargo build --release
# or
$ cargo test
~~~

(invoked in the repository root)

All binaries are named `dayXXa` or `dayXXb` and read the input from `stdin`. For example, if you want to run part A of Day 01, you would run

~~~bash
$ ./target/release/day01a < day_01/input.txt
~~~

## Assignments

* [Day 1: Calorie Counting](day_01)
* [Day 2: Rock Paper Scissors](day_02)
* [Day 3: Rucksack Reorganization](day_03)
