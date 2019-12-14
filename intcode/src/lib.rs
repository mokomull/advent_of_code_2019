pub fn parse_opcodes(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().expect("non-integer found"))
        .collect()
}

pub fn run(mut opcodes: Vec<usize>) -> Vec<usize> {
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

#[cfg(test)]
mod tests {
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
}
