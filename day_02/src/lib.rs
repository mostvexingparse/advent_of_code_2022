
#[derive(Clone, Copy)]
pub enum RpsShape {
    Rock,
    Paper,
    Scissors,
}

impl RpsShape {
    pub fn from_str(s: &str) -> Option<RpsShape> {
        match s.to_lowercase().trim() {
            "a" | "x" => Some(RpsShape::Rock),
            "b" | "y" => Some(RpsShape::Paper),
            "c" | "z" => Some(RpsShape::Scissors),
            _ => None
        }
    }

    pub fn to_score(&self) -> usize {
        match self {
            Self::Rock     => 1,
            Self::Paper    => 2,
            Self::Scissors => 3,
        }
    }
}

pub enum RpsOutcome {
    Lost(RpsShape),
    Draw(RpsShape),
    Won(RpsShape)
}

impl RpsOutcome {
    pub fn to_score(&self) -> usize {
        match self {
            Self::Lost(shape) => 0 + shape.to_score(),
            Self::Draw(shape) => 3 + shape.to_score(),
            Self::Won(shape)  => 6 + shape.to_score(),
        }
    }
}

pub fn play_round(opponent: RpsShape, me: RpsShape) -> RpsOutcome {
    match (opponent, me) {
        (RpsShape::Rock,      RpsShape::Paper)    => RpsOutcome::Won(RpsShape::Paper),
        (RpsShape::Rock,      RpsShape::Rock)     => RpsOutcome::Draw(RpsShape::Rock),
        (RpsShape::Rock,      RpsShape::Scissors) => RpsOutcome::Lost(RpsShape::Scissors),
        (RpsShape::Paper,     RpsShape::Paper)    => RpsOutcome::Draw(RpsShape::Paper),
        (RpsShape::Paper,     RpsShape::Rock)     => RpsOutcome::Lost(RpsShape::Rock),
        (RpsShape::Paper,     RpsShape::Scissors) => RpsOutcome::Won(RpsShape::Scissors),
        (RpsShape::Scissors,  RpsShape::Paper)    => RpsOutcome::Lost(RpsShape::Paper),
        (RpsShape::Scissors,  RpsShape::Rock)     => RpsOutcome::Won(RpsShape::Rock),
        (RpsShape::Scissors,  RpsShape::Scissors) => RpsOutcome::Draw(RpsShape::Scissors),
    }
}

pub enum ExpectedOutcome {
    Loose,
    Draw,
    Win
}

impl ExpectedOutcome {
    pub fn from_str(s: &str) -> Option<ExpectedOutcome> {
        match s.to_lowercase().trim() {
            "x" => Some(Self::Loose),
            "y" => Some(Self::Draw),
            "z" => Some(Self::Win),
            _   => None,
        }
    }

    pub fn required_shape(&self, opponent: RpsShape) -> RpsShape {
        match (self, opponent) {
            (Self::Loose, RpsShape::Rock)     => RpsShape::Scissors,
            (Self::Loose, RpsShape::Paper)    => RpsShape::Rock,
            (Self::Loose, RpsShape::Scissors) => RpsShape::Paper,
            (Self::Win,   RpsShape::Rock)     => RpsShape::Paper,
            (Self::Win,   RpsShape::Paper)    => RpsShape::Scissors,
            (Self::Win,   RpsShape::Scissors) => RpsShape::Rock,
            // Draw: just play the same thing as the opponent
            (Self::Draw, op)        => op,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
       let input = vec!["A Y".to_owned(), "B X".to_owned(), "C Z".to_owned()];
       let expected_score: usize = 15;

       let actual_score: usize = input.iter()
            .map(|s| {
                let strs = s.split_at(1);
                (strs.0.to_owned(), strs.1.to_owned())
            })
            .map(|(a, b)| (RpsShape::from_str(&a).unwrap(), RpsShape::from_str(&b).unwrap()))
            .map(|(a, b)| play_round(a, b).to_score())
            .sum();

        assert!(actual_score == expected_score);
    }

    #[test]
    fn test_part_b() {
        let input = vec!["A Y".to_owned(), "B X".to_owned(), "C Z".to_owned()];
        let expected_score: usize = 12;

        let actual_score: usize = input.iter()
            .map(|s| {
                let strs = s.split_at(1);
                (strs.0.to_owned(), strs.1.to_owned())
            })
            .map(|(a, b)| (RpsShape::from_str(&a).unwrap(), ExpectedOutcome::from_str(&b).unwrap()))
            .map(|(a, b)| (a, b.required_shape(a)))
            .map(|(a, b)| play_round(a, b).to_score())
            .sum();

        assert!(actual_score == expected_score);
    }
}
