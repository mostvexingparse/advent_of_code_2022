use itertools::Itertools;
use regex::Regex;
use std::io::{stdin, Read};

mod stacks;
use stacks::{parse_stacks, Stacks};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let (stack_input, movement_input) = input.split_once("\n\n").unwrap();

    let movements: Vec<Movement> = movement_input.lines().map(Movement::from_str).collect();

    println!(
        "Part A: {}",
        process(stack_input, &movements, crate_mover_9000)
    );

    println!(
        "Part B: {}",
        process(stack_input, &movements, crate_mover_9001)
    );
}

fn crate_mover_9000(stacks: &mut Stacks, mvmnt: &Movement) {
    for _ in 0..mvmnt.count {
        let krate = stacks.get_mut(&mvmnt.from).unwrap().pop().unwrap();
        stacks.get_mut(&mvmnt.to).unwrap().push(krate);
    }
}

fn crate_mover_9001(stacks: &mut Stacks, mvmnt: &Movement) {
    let rem = stacks.get(&mvmnt.from).unwrap().len() - mvmnt.count;

    stacks
        .get_mut(&mvmnt.from)
        .unwrap()
        .drain(rem..)
        .collect_vec()
        .iter()
        .for_each(|s| stacks.get_mut(&mvmnt.to).unwrap().push(s.to_string()));
}

fn process<CraneFunc>(stack_input: &str, movements: &Vec<Movement>, crane_func: CraneFunc) -> String
where
    CraneFunc: Fn(&mut Stacks, &Movement),
{
    let mut stacks = parse_stacks(stack_input);

    movements
        .into_iter()
        .for_each(|movement| crane_func(&mut stacks, movement));

    stacks
        .keys()
        .sorted()
        .map(|key| stacks.get(key).unwrap().last().unwrap())
        .join("")
}

struct Movement {
    count: usize,
    from: usize,
    to: usize,
}

impl Movement {
    fn from_str(s: &str) -> Movement {
        let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        let cap = re.captures(s).unwrap();

        Movement {
            count: cap[1].parse().unwrap(),
            from: cap[2].parse().unwrap(),
            to: cap[3].parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = concat!(
        "    [D]    \n",
        "[N] [C]    \n",
        "[Z] [M] [P]\n",
        " 1   2   3 \n",
        "\n",
        "move 1 from 2 to 1\n",
        "move 3 from 1 to 3\n",
        "move 2 from 2 to 1\n",
        "move 1 from 1 to 2\n",
    );

    #[test]
    fn test_crate_mover_9000() {
        let (stack_input, movement_input) = INPUT.split_once("\n\n").unwrap();

        let movements: Vec<Movement> = movement_input.lines().map(Movement::from_str).collect();

        let result = process(stack_input, &movements, crate_mover_9000);

        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_crate_mover_9001() {
        let (stack_input, movement_input) = INPUT.split_once("\n\n").unwrap();

        let movements: Vec<Movement> = movement_input.lines().map(Movement::from_str).collect();

        let result = process(stack_input, &movements, crate_mover_9001);

        assert_eq!(result, "MCD");
    }
}
