use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;
use num::Integer;

fn main() {
    do_main("inputs/day_14.txt");
}

fn do_main(path: &str) {
    let reactions = BufReader::new(File::open(path).expect("could not read input"))
        .lines()
        .map(|line| line.expect("could not read line"))
        .map(|line| parse_reaction(&line))
        .collect::<Vec<_>>();

    let part1 = how_much_ore_to_make(1, "FUEL", &reactions);
    dbg!(part1);
}

fn how_much_ore_to_make(how_many: usize, what: &str, reactions: &[Reaction]) -> usize {
    let target = reactions
        .iter()
        .filter(|&reaction| reaction.output == what)
        .next()
        .expect("universe of reactions doesn't know how to make this");

    let mut total = 0;
    for (element, count) in target.inputs.iter() {
        if element == "ORE" {
            total += count;
        } else {
            total += how_much_ore_to_make(*count, element, reactions);
        }
    }

    total * how_many / total.gcd(&how_many)
}

#[derive(Debug, Eq, PartialEq)]
struct Reaction {
    inputs: HashMap<String, usize>,
    output: String,
    output_count: usize,
}

fn parse_reaction(input: &str) -> Reaction {
    let mut inputs = HashMap::new();

    let halves = input.split(" => ").collect::<Vec<_>>();

    for (count, element) in halves[0].split_whitespace().tuples() {
        let count = count.parse().expect("input was not an integer");
        let element = element.split(",").next().unwrap().to_owned();

        inputs.insert(element, count);
    }

    let mut right_side = halves[1].split_whitespace();
    let output_count = right_side
        .next()
        .expect("RHS didn't contain a count")
        .parse()
        .expect("output count was not an integer");
    let output = right_side
        .next()
        .expect("RHS did not contain an element")
        .to_owned();

    Reaction {
        inputs,
        output,
        output_count,
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_reaction() {
        assert_eq!(
            super::parse_reaction("9 ORE => 2 A"),
            super::Reaction {
                inputs: [("ORE".into(), 9)].iter().cloned().collect(),
                output: "A".into(),
                output_count: 2,
            }
        );

        assert_eq!(
            super::parse_reaction("3 A, 4 B => 1 AB"),
            super::Reaction {
                inputs: [("A".into(), 3), ("B".into(), 4)].iter().cloned().collect(),
                output: "AB".into(),
                output_count: 1,
            }
        );
    }
}
