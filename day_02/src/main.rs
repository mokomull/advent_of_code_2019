fn main() {
    do_main("inputs/day_02.txt");
}

fn do_main(filename: &str) {
    let input = std::fs::read_to_string(filename).expect("input not found");
    let mut opcodes = intcode::parse_opcodes(&input);
    opcodes[1] = 12;
    opcodes[2] = 2;
    let result = intcode::run(opcodes);
    println!("Position 0 contains: {}", result[0]);
    assert_eq!(result[0], 3101878);

    let original_opcodes = intcode::parse_opcodes(&input);
    let mut needle = None;

    'a: for noun in 0..=99 {
        for verb in 0..=99 {
            let mut opcodes = original_opcodes.clone();
            opcodes[1] = noun;
            opcodes[2] = verb;
            if intcode::run(opcodes)[0] == 19690720 {
                needle = Some((noun, verb));
                break 'a;
            }
        }
    }

    let (noun, verb) = needle.expect("no satisfactory inputs found");
    println!("Necessary input is: {}", 100 * noun + verb);
    assert_eq!((noun, verb), (84, 44));
}

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::do_main("../inputs/day_02.txt");
    }
}
