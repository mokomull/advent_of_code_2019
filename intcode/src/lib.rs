use std::collections::VecDeque;
use std::convert::TryInto;

pub fn parse_opcodes(input: &str) -> Vec<isize> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().expect("non-integer found"))
        .collect()
}

pub fn run(opcodes: Vec<isize>) -> Vec<isize> {
    let (memory, _output) = run_with_io(opcodes, VecDeque::new());
    memory
}

// Returns (memory, output)
pub fn run_with_io(
    mut opcodes: Vec<isize>,
    mut input: VecDeque<isize>,
) -> (Vec<isize>, Vec<isize>) {
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

fn get_operands_3(opcodes: &[isize], ip: &mut usize) -> (isize, isize, usize) {
    let source1 = get_read_operand_at(opcodes, *ip, 1);
    let source2 = get_read_operand_at(opcodes, *ip, 2);
    let destination = get_write_index_at(opcodes, *ip, 3);

    *ip += 4;

    (source1, source2, destination)
}

fn get_read_operand_at(opcodes: &[isize], ip: usize, idx: usize) -> isize {
    let source_idx = opcodes[ip + idx];
    match opcodes[ip] / 10isize.pow((idx + 1).try_into().unwrap()) % 10 {
        0 => {
            let source_idx: usize = source_idx.try_into().expect("un-indexable memory offset");
            opcodes[source_idx]
        }
        1 => source_idx,
        x => panic!("Invalid parameter mode {} at ip {}", x, ip),
    }
}

fn get_write_index_at(opcodes: &[isize], ip: usize, idx: usize) -> usize {
    let destination_idx = opcodes[ip + idx];
    match opcodes[ip] / 10isize.pow((idx + 1).try_into().unwrap()) % 10 {
        0 => destination_idx
            .try_into()
            .expect("un-indexable memory offset"),
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

    #[test]
    fn negative() {
        assert_eq!(
            super::run(vec![1101, 100, -1, 4, 0]),
            vec![1101, 100, -1, 4, 99]
        );
    }
}
