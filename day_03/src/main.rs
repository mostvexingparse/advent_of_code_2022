#![feature(iter_array_chunks)]
use std::collections::HashSet;
use std::io::{stdin, BufRead};

fn main() {
    let input: Vec<String> = stdin().lock().lines()
        .map(|result| result.expect("error while reading from stdin"))
        .collect();

    let part_a_result: usize = input.iter()
        .map(shared_item)
        .map(priority)
        .sum();

    println!("Part A: {}", part_a_result);

    let part_b_result: usize = input.iter()
        .array_chunks::<3>()
        .map(shared_item_in_group)
        .map(priority)
        .sum();

    println!("Part B: {}", part_b_result);
}

fn split_center(s: &String) -> (String, String) {
    let mid = s.len() / 2;
    let parts = s.split_at(mid);

    (parts.0.to_owned(), parts.1.to_owned())
}

fn priority(c: char) -> usize {
    const CHARS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    CHARS.find(c).expect("priority not found") + 1
}

fn shared_item(s: &String) -> char {
    let (a, b) = split_center(s);

    let a_set: HashSet<char> = HashSet::from_iter(a.chars());
    let b_set: HashSet<char> = HashSet::from_iter(b.chars());

    a_set
        .intersection(&b_set)
        .nth(0)
        .expect("no shared shared item found")
        .to_owned()
}

fn shared_item_in_group(group: [&String; 3]) -> char {
    let intersection = group.iter()
        .map(|s| HashSet::<char>::from_iter(s.chars()))
        .reduce(|a, b| &a & &b)
        .expect("oops.");

    intersection.iter()
        .nth(0)
        .expect("not shared item found")
        .to_owned()
}
