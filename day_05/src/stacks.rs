use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::streaming::char,
    combinator::map,
    multi::{many0, separated_list1},
    sequence::{delimited, terminated},
    IResult,
};
use std::collections::HashMap;

pub type Stacks = HashMap<usize, Vec<String>>;

pub fn parse_stacks(input: &str) -> Stacks {
    let mut stacks = HashMap::new();
    let (_, slot_matrix) = parse_slots_block(input).unwrap();

    assert!(slot_matrix.len() > 0);
    let width = slot_matrix[0].len();

    for slot_vec in slot_matrix.iter().rev() {
        for i in 0..width {
            let key = i + 1;
            
            if !stacks.contains_key(&key) {
                stacks.insert(key, Vec::new());
            }

            if let Slot::Krate(krate) = &slot_vec[i] {
                stacks.get_mut(&key).unwrap().push(krate.to_owned());
            }
        }
    }

    stacks
}

#[derive(PartialEq, Debug)]
enum Slot {
    Krate(String),
    Empty,
}

fn parse_slot(input: &str) -> IResult<&str, Slot> {
    let krate = map(
        delimited(char('['), take(1 as usize), char(']')), 
        |c| Slot::Krate(String::from(c))
    );
    let empty = map(tag("   "), |_| Slot::Empty);

    alt((krate, empty))(input)
}

fn parse_slots_line(input: &str) -> IResult<&str, Vec<Slot>> {
    separated_list1(tag(" "), parse_slot)(input)
}

fn parse_slots_block(input: &str) -> IResult<&str, Vec<Vec<Slot>>> {
    many0(terminated(parse_slots_line, char('\n')))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_krate_slot() {
        let result = parse_slot("[A]");

        assert!(result.is_ok());

        let (rem, slot) = result.unwrap();

        assert_eq!(rem, "");
        assert_eq!(slot, Slot::Krate(String::from("A")));
    }

    #[test]
    fn test_empty_slot() {
        let result = parse_slot("   ");

        assert!(result.is_ok());

        let (rem, slot) = result.unwrap();

        assert_eq!(rem, "");
        assert_eq!(slot, Slot::Empty);
    }

    #[test]
    fn test_slots_line_1() {
        let result = parse_slots_line("[Z] [M] [P]");

        assert!(result.is_ok());

        let (rem, slots) = result.unwrap();

        assert_eq!(rem, "");
        assert_eq!(slots.len(), 3);
        assert_eq!(slots[0], Slot::Krate(String::from("Z")));
        assert_eq!(slots[1], Slot::Krate(String::from("M")));
        assert_eq!(slots[2], Slot::Krate(String::from("P")));
    }

    #[test]
    fn test_slots_line_2() {
        let result = parse_slots_line("[N] [C]    ");

        assert!(result.is_ok());

        let (rem, slots) = result.unwrap();

        assert_eq!(rem, "");
        assert_eq!(slots.len(), 3);
        assert_eq!(slots[0], Slot::Krate(String::from("N")));
        assert_eq!(slots[1], Slot::Krate(String::from("C")));
        assert_eq!(slots[2], Slot::Empty);
    }

    #[test]
    fn test_slots_line_3() {
        let result = parse_slots_line("    [D]    ");

        assert!(result.is_ok());

        let (rem, slots) = result.unwrap();

        assert_eq!(rem, "");
        assert_eq!(slots.len(), 3);
        assert_eq!(slots[0], Slot::Empty);
        assert_eq!(slots[1], Slot::Krate(String::from("D")));
        assert_eq!(slots[2], Slot::Empty);
    }

    #[test]
    fn test_slots_block() {
        let input = concat!(
            "    [D]    \n",
            "[N] [C]    \n",
            "[Z] [M] [P]\n",
            " 1   2   3 \n",
        );

        let result = parse_slots_block(input);

        assert!(result.is_ok());

        let (rem, slot_matrix) = result.unwrap();

        assert_eq!(rem, " 1   2   3 \n");
        
        assert_eq!(slot_matrix.len(), 3);
        assert_eq!(slot_matrix[0].len(), 3);
        assert_eq!(slot_matrix[1].len(), 3);
        assert_eq!(slot_matrix[2].len(), 3);

        assert_eq!(slot_matrix[0][0], Slot::Empty);
        assert_eq!(slot_matrix[0][1], Slot::Krate(String::from("D")));
        assert_eq!(slot_matrix[0][2], Slot::Empty);

        assert_eq!(slot_matrix[1][0], Slot::Krate(String::from("N")));
        assert_eq!(slot_matrix[1][1], Slot::Krate(String::from("C")));
        assert_eq!(slot_matrix[1][2], Slot::Empty);

        assert_eq!(slot_matrix[2][0], Slot::Krate(String::from("Z")));
        assert_eq!(slot_matrix[2][1], Slot::Krate(String::from("M")));
        assert_eq!(slot_matrix[2][2], Slot::Krate(String::from("P")));
    }
}
