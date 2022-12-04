use std::collections::HashSet;
use std::io::{stdin, BufRead};

struct Assignment(HashSet<usize>, HashSet<usize>);

fn parse_assignment(line: String) -> Assignment {
    let section_limits: Vec<usize> = line
        .split_terminator(&['-', ','])
        .map(|s| s.parse::<usize>().expect("parsing failed"))
        .collect();

    assert!(section_limits.len() == 4, "expected 4 values");

    Assignment (
        HashSet::from_iter(section_limits[0]..(section_limits[1] + 1)),
        HashSet::from_iter(section_limits[2]..(section_limits[3] + 1)),
    )
}

fn main() {
    let section_assignments: Vec<Assignment> = stdin().lock().lines()
        .map(|result| result.expect("error while reading from stdin"))
        .map(parse_assignment)
        .collect();

    let subset_count = section_assignments.iter()
        .filter(|Assignment (a, b)| a.is_subset(b) || b.is_subset(a))
        .count();

    println!("Part A: {}", subset_count);

    let overlap_count = section_assignments.iter()
        .filter(|Assignment (a, b)| a.intersection(b).count() != 0)
        .count();

    println!("Part B: {}", overlap_count);
}
