#![feature(iter_array_chunks)]
use std::io::BufRead;

/// PART A: find the shared items in each rucksack and return their summed priorities
pub fn get_shared_item_sum(input: &mut dyn BufRead) -> usize {
    input.lines()
        .filter_map(|result| result.ok())
        .filter(|line| !line.trim().is_empty())
        .map(|line| get_shared_item(&line))
        .map(get_item_priority)
        .sum()
}

/// PART B: find the shared items in each group of three and return their summed priorities
pub fn get_grouped_item_sum(input: &mut dyn BufRead) -> usize {
    // FIXME: this only works when the number of lines is divisible by 3
    // (which is the case with the provided input data).
    // For correct results with every input, `into_remainder()`
    // needs to be checked.
    input.lines()
        .filter_map(|result| result.ok())
        .filter(|line| !line.is_empty())
        .array_chunks::<3>()
        .map(get_shared_item_in_group)
        .map(get_item_priority)
        .sum()
}

/// Splits the rucksack content in the middle an returns it as a tuple
fn split_rucksack_content(content: &str) -> (String, String) {
    let split_point = content.len() / 2;
    let parts = content.split_at(split_point);

    (parts.0.to_owned(), parts.1.to_owned())
}

/// Get the priority for a rucksack item
fn get_item_priority(item: char) -> usize {
    let pivot = if item < 'a' {
        'A'
    } else {
        'a'
    } as usize;

    let modifier = if item < 'a' {
        27
    } else {
        1
    } as usize;

    item as usize - pivot + modifier
}

/// Retrieve the item that occurs in both compartments
fn get_shared_item(content: &str) -> char {
    let (part_a, part_b) = split_rucksack_content(content);
    
    for c in part_a.chars() {
        if part_b.contains(c) {
            return c;
        }
    }

    panic!("no item occurs in both compartments");
}

/// Find the item that is carried by all group members
fn get_shared_item_in_group(group: [String; 3]) -> char {
    let [group_a, group_b, group_c] = group;

    for c in group_a.chars() {
        if group_b.contains(c) && group_c.contains(c) {
            return c;
        }
    }

    panic!("group has no shared items");
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, BufRead};
    use stringreader::StringReader;
    use super::*;

    #[test]
    fn test_part_a() {
        const EXPECTED: usize = 157;
        let mut reader = get_input_reader();
        
        let result = get_shared_item_sum(&mut reader);

        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn test_part_b() {
        const EXPECTED: usize = 70;
        let mut reader = get_input_reader();

        let result = get_grouped_item_sum(&mut reader);

        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn test_split_rucksack_content() {
        for line in get_input_data() {
            let result = split_rucksack_content(&line);
            assert_eq!(result.0.len(), result.1.len());
        }        
    }

    #[test]
    fn test_get_item_priority() {
        let data = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        for (index, item) in data.chars().enumerate() {
            let expected = index + 1;
            let actual = get_item_priority(item);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_get_shared_item() {
        let data = get_input_data();
        let results = "pLPvts".chars();

        for (content, result) in data.iter().zip(results) {
            assert_eq!(get_shared_item(content), result);
        }
    }

    #[test]
    fn test_get_shared_item_in_group() {
        let data = get_input_data();
        let results = "rZ".chars();

        for (group, result) in data.iter().array_chunks::<3>().zip(results) {
            let input = [group[0].to_owned(), group[1].to_owned(), group[2].to_owned()];
            assert_eq!(get_shared_item_in_group(input), result);
        }
    }

    const INPUT: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    fn get_input_reader<'a>() -> BufReader<StringReader<'a>> {
        BufReader::new(StringReader::new(INPUT))
    }

    fn get_input_data() -> Vec<String> {
        let reader = get_input_reader();

        reader.lines()
            .filter_map(|line| line.ok())
            .filter(|line| !line.is_empty())
            .collect()
    }
}
