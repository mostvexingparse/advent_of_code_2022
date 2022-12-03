use std::io::{stdin, BufRead};

use day_02::*;

fn main() {
    let result: usize = stdin().lock().lines()
        .map(|line| line.expect("error while reading from stdin"))
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts = line.split_at(1);
            (parts.0.to_owned(), parts.1.to_owned())
        })
        .map(|(a, b)| (RpsShape::from_str(&a).unwrap(), ExpectedOutcome::from_str(&b).unwrap()))
        .map(|(a, b)| (a, b.required_shape(a)))
        .map(|(a, b)| play_round(a, b).to_score())
        .sum();

    println!("{}", result);
}
