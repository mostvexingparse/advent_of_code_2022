use std::{collections::HashSet, io::{stdin, Read}};
use itertools::Itertools;

fn has_unique_chars(s: &str) -> bool {
    let hash: HashSet<char> = HashSet::from_iter(s.chars());
    hash.len() == s.len()
}

fn find_marker(input: &str, len: usize) -> Option<usize> {
    for i in 0..(input.len() - len) {
        let window: String = input.chars().dropping(i).take(len).collect();
        if has_unique_chars(&window) {
            return Some(i + window.len())
        }
    }

    None
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    println!("Part A: {}", find_marker(&input, 4).unwrap());
    println!("Part B: {}", find_marker(&input, 14).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_unique_chars() {
        assert_eq!(has_unique_chars("aaaa"), false);
        assert_eq!(has_unique_chars("abca"), false);
        assert_eq!(has_unique_chars("abcde"), true);
    }

    #[test]
    fn test_part_a() {
        assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), Some(7));
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), Some(5));
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), Some(6));
        assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), Some(10));
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), Some(11));
    }

    #[test]
    fn test_part_b() {
        assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), Some(19));
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), Some(23));
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), Some(23));
        assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), Some(29));
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), Some(26));
    }
}
