use std::io::{stdin, BufRead};

fn read_calories_per_elf(reader: &mut dyn BufRead) -> Vec<usize> {
    let mut result = Vec::new();
    let mut calories_per_elf: usize = 0;

    for line in reader.lines().filter_map(|line| line.ok()) {
        if line.is_empty() {
            result.push(calories_per_elf);
            calories_per_elf = 0;
        } else {
            calories_per_elf += line.parse::<usize>()
                .expect("oops. parsing failed.");
        }
    }

    if calories_per_elf != 0 {
        result.push(calories_per_elf);
    }

    result.sort_by(|a, b| b.cmp(a));

    result
}

fn main() {
    let calories_per_elf = read_calories_per_elf(&mut stdin().lock());

    let part_a_result = calories_per_elf[0];
    println!("Part A: {}", part_a_result);

    let part_b_result: usize = calories_per_elf.iter().take(3).sum();
    println!("Part B: {}", part_b_result);
}
