fn parse_opcodes(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().expect("non-integer found"))
        .collect()
}

fn run(mut opcodes: Vec<usize>) -> Vec<usize> {
    let mut ip = 0;

    loop {
        match opcodes[ip] {
            1 => {
                let source1 = opcodes[ip + 1];
                let source2 = opcodes[ip + 2];
                let destination = opcodes[ip + 3];
                opcodes[destination] = opcodes[source1] + opcodes[source2];
                ip += 4;
            }
            2 => {
                let source1 = opcodes[ip + 1];
                let source2 = opcodes[ip + 2];
                let destination = opcodes[ip + 3];
                opcodes[destination] = opcodes[source1] * opcodes[source2];
                ip += 4;
            }
            99 => break,
            x => panic!("unexpected opcode found in position {}: {}", ip, x),
        }
    }

    opcodes
}

fn main() {
    do_main("inputs/day_02.txt");
}

fn do_main(filename: &str) {
    let input = std::fs::read_to_string(filename).expect("input not found");
    let mut opcodes = parse_opcodes(&input);
    opcodes[1] = 12;
    opcodes[2] = 2;
    let result = run(opcodes);
    println!("Position 0 contains: {}", result[0]);
    assert_eq!(result[0], 3101878);
}

#[cfg(test)]
mod test {
    #[test]
    fn parser() {
        assert_eq!(
            super::parse_opcodes("1,9,10,3,2,3,11,0,99,30,40,50"),
            vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50)
        );
    }

    #[test]
    fn run() {
        assert_eq!(
            super::run(super::parse_opcodes("1,9,10,3,2,3,11,0,99,30,40,50")),
            vec!(3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50)
        );
    }

    #[test]
    fn main() {
        super::do_main("../inputs/day_02.txt");
    }
}
