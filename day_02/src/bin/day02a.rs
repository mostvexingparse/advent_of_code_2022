use std::io::{stdin, BufRead};

use day_02::*;

fn main() {
    let result: usize = stdin().lock().lines()
        .map(|line| line.expect("error while reading from stdin"))
        .filter(|line| !line.is_empty())
        .map(|s| {
            let strs = s.split_at(1);
            (strs.0.to_owned(), strs.1.to_owned())
        })
        .map(|(a, b)| (RpsShape::from_str(&a).unwrap(), RpsShape::from_str(&b).unwrap()))
        .map(|(a, b)| play_round(a, b).to_score())
        .sum();

    println!("{}", result);
}
