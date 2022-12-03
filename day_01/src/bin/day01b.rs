use std::{error::Error, io::stdin};

use day_01::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut results = parse_input(&mut stdin().lock())?;

    results.sort_by(|a, b| b.cmp(a));

    let result: usize = results.iter()
        .take(3)
        .sum();

    println!("{}", result);

    Ok(())
}
