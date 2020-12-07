use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

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
    assert_eq!(part1, 483766);
}

fn how_much_ore_to_make(how_many: usize, what: &str, reactions: &[Reaction]) -> usize {
    let mut needed = [(what.to_owned(), how_many), ("ORE".into(), 0)]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>();
    let mut extra = HashMap::new();

    while needed.len() > 1 {
        let this_element = needed
            .keys()
            .filter(|&element| element != "ORE")
            .cloned()
            .next()
            .unwrap();

        let this_count = needed.remove(&this_element).unwrap();

        let target = reactions
            .iter()
            .filter(|&reaction| reaction.output == this_element)
            .next()
            .expect("universe of reactions doesn't know how to make this");

        while *extra.get(&this_element).unwrap_or(&0) < this_count {
            *extra.entry(this_element.clone()).or_insert(0) += target.output_count;

            for (element, count) in target.inputs.iter() {
                *needed.entry(element.clone()).or_insert(0) += count;
            }
        }

        *extra.get_mut(&this_element).unwrap() -= this_count;
    }

    *needed.get("ORE").unwrap()
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

    fn parse_string(input: &str) -> Vec<super::Reaction> {
        input
            .lines()
            .map(|line| super::parse_reaction(line))
            .collect::<Vec<_>>()
    }

    #[test]
    fn calculate_ore() {
        let example = parse_string(
            "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL",
        );
        assert_eq!(super::how_much_ore_to_make(1, "FUEL", &example), 31);

        let example = parse_string(
            "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL",
        );
        assert_eq!(super::how_much_ore_to_make(1, "FUEL", &example), 165);
    }

    #[test]
    fn main() {
        super::do_main("../inputs/day_14.txt");
    }
}
