use std::{error::Error, io::stdin};

use day_01::*;

fn main() -> Result<(), Box<dyn Error>>{
    let results = parse_input(&mut stdin().lock())?;

    if let Some(max) = results.iter().max() {
        println!("{}", max);
    } else {
        eprintln!("no values!");
    }

    Ok(())
}