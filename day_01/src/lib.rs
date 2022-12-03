use std::{error::Error, io::BufRead};

pub fn parse_input(reader: &mut dyn BufRead) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut result = Vec::new();
    let mut line_sum: usize = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        if line.is_empty() {
            result.push(line_sum);
            line_sum = 0;
        } else {
            line_sum += line.trim().parse::<usize>()?;
        }
    }

    if line_sum != 0 {
        result.push(line_sum);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;
    use stringreader::StringReader;

    #[test]
    fn test_part_a() {
        const EXPECTED: usize = 24000;

        let values = get_parsed_values();

        let result: usize = values.iter()
            .max()
            .unwrap()
            .clone();

        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn test_part_b() {
        const EXPECTED: usize = 45000;

        let mut values = get_parsed_values();
        values.sort_by(|a, b| b.cmp(a));

        let result: usize = values.iter()
            .take(3)
            .sum();

        assert_eq!(result, EXPECTED);
    }

    fn get_parsed_values<'a>() -> Vec<usize> {
        let stringreader = StringReader::new(INPUT);
        let mut reader = BufReader::new(stringreader);

        let parsed = parse_input(&mut reader);

        assert_eq!(parsed.is_ok(), true);

        let results = parsed.unwrap();

        assert_eq!(results.len(), 5);

        results
    }

    const INPUT: &'static str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
}
