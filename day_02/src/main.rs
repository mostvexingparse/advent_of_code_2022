use std::io::{stdin, BufRead};

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_char(c: &char) -> Shape {
        match c {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("unknown input: '{}'", c),
        }
    }

    fn score(&self) -> usize {
        match self {
            Self::Rock     => 1,
            Self::Paper    => 2,
            Self::Scissors => 3,
        }
    }
}

enum Outcome {
    Lost(Shape),
    Draw(Shape),
    Won(Shape)
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Self::Lost(shape) => shape.score() + 0,
            Self::Draw(shape) => shape.score() + 3,
            Self::Won(shape)  => shape.score() + 6,
        }
    }
}

fn play_round(opponent: Shape, me: Shape) -> Outcome {
    match (opponent, me) {
        (Shape::Rock,      Shape::Paper)    => Outcome::Won(Shape::Paper),
        (Shape::Rock,      Shape::Rock)     => Outcome::Draw(Shape::Rock),
        (Shape::Rock,      Shape::Scissors) => Outcome::Lost(Shape::Scissors),
        (Shape::Paper,     Shape::Paper)    => Outcome::Draw(Shape::Paper),
        (Shape::Paper,     Shape::Rock)     => Outcome::Lost(Shape::Rock),
        (Shape::Paper,     Shape::Scissors) => Outcome::Won(Shape::Scissors),
        (Shape::Scissors,  Shape::Paper)    => Outcome::Lost(Shape::Paper),
        (Shape::Scissors,  Shape::Rock)     => Outcome::Won(Shape::Rock),
        (Shape::Scissors,  Shape::Scissors) => Outcome::Draw(Shape::Scissors),
    }
}

enum ExpectedOutcome {
    Loose,
    Draw,
    Win
}

impl ExpectedOutcome {
    fn from_char(c: &char) -> ExpectedOutcome {
        match c {
            'X' => Self::Loose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("unknown input: '{}'", c),
        }
    }

    fn required_shape(&self, opponent: Shape) -> Shape {
        match (self, opponent) {
            (Self::Loose, Shape::Rock)     => Shape::Scissors,
            (Self::Loose, Shape::Paper)    => Shape::Rock,
            (Self::Loose, Shape::Scissors) => Shape::Paper,
            (Self::Win,   Shape::Rock)     => Shape::Paper,
            (Self::Win,   Shape::Paper)    => Shape::Scissors,
            (Self::Win,   Shape::Scissors) => Shape::Rock,
            // Draw: just play the same thing as the opponent
            (Self::Draw, op)        => op,
        }
    }
}

fn to_char_tuple(s: String) -> (char, char) {
    (s.chars().nth(0).unwrap(), s.chars().nth(2).unwrap())
}

fn main() {
    let input: Vec<(char, char)> = stdin().lock().lines()
        .map(|result| result.expect("error while reading from stdin"))
        .map(to_char_tuple)
        .collect();

    let part_a_result: usize = input.iter()
        .map(|(a, b)| (Shape::from_char(a), Shape::from_char(b)))
        .map(|(a, b)| play_round(a, b).score())
        .sum();

    println!("Part A: {}", part_a_result);

    let part_b_result: usize = input.iter()
        .map(|(a, b)| (Shape::from_char(a), ExpectedOutcome::from_char(b)))
        .map(|(a, b)| (a, b.required_shape(a)))
        .map(|(a, b)| play_round(a, b).score())
        .sum();

    println!("Part B: {}", part_b_result);
}
