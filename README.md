# Advent of code 2022

My AOC2022 solutions in (nightly) Rust.

## Project structure

One subproject for each day, with a binary handling both parts. The subprojects also have a `README.md` which contains the assignment, a file named `input_test.txt` with the test data from the example and `input.txt` with the "real" data required for solving the assignment.

All subprojects are part of a workspace, so they can be built at once: 

~~~bash
$ cargo build --release
~~~

(invoked in the repository root)

All binaries read the input from `stdin`. For example, if you want to run the solution for day 01, you would run

~~~bash
$ ./target/release/day_01 < day_01/input.txt
~~~

## Assignments

* [Day 1: Calorie Counting](day_01)
* [Day 2: Rock Paper Scissors](day_02)
* [Day 3: Rucksack Reorganization](day_03)
* [Day 4: Camp Cleanup](day_04)
