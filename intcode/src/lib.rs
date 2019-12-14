use std::collections::VecDeque;
use std::convert::TryInto;

pub fn parse_opcodes(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().expect("non-integer found"))
        .collect()
}

pub fn run(opcodes: Vec<usize>) -> Vec<usize> {
    let (memory, _output) = run_with_io(opcodes, VecDeque::new());
    memory
}

// Returns (memory, output)
pub fn run_with_io(
    mut opcodes: Vec<usize>,
    mut input: VecDeque<usize>,
) -> (Vec<usize>, Vec<usize>) {
    let mut ip = 0;

    loop {
        match opcodes[ip] % 100 {
            1 => {
                let (source1, source2, destination) = get_operands_3(&opcodes, &mut ip);
                opcodes[destination] = source1 + source2;
            }
            2 => {
                let (source1, source2, destination) = get_operands_3(&opcodes, &mut ip);
                opcodes[destination] = source1 * source2;
            }
            99 => break,
            x => panic!("unexpected opcode found in position {}: {}", ip, x),
        }
    }

    (opcodes, vec![])
}

fn get_operands_3(opcodes: &[usize], ip: &mut usize) -> (usize, usize, usize) {
    let source1 = get_read_operand_at(opcodes, *ip, 1);
    let source2 = get_read_operand_at(opcodes, *ip, 2);
    let destination = get_write_index_at(opcodes, *ip, 3);

    *ip += 4;

    (source1, source2, destination)
}

fn get_read_operand_at(opcodes: &[usize], ip: usize, idx: usize) -> usize {
    let source_idx = opcodes[ip + idx];
    match opcodes[ip] / 10usize.pow((idx + 1).try_into().unwrap()) % 10 {
        0 => opcodes[source_idx],
        1 => source_idx,
        x => panic!("Invalid parameter mode {} at ip {}", x, ip),
    }
}

fn get_write_index_at(opcodes: &[usize], ip: usize, idx: usize) -> usize {
    let destination_idx = opcodes[ip + idx];
    match opcodes[ip] / 10usize.pow((idx + 1).try_into().unwrap()) % 10 {
        0 => destination_idx,
        x => panic!("Invalid destination parameter mode {} at ip {}", x, ip),
    }
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

    #[test]
    fn parameter_modes() {
        assert_eq!(
            super::run(vec![1002, 4, 3, 4, 33]),
            vec![
                1002,
                4,
                3,
                4,
                33 /* memory at 4 */ * 3 /* immediate 3 */
            ]
        );
    }
}
