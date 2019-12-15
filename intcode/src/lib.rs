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
    let mut output = Vec::new();

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
            3 => {
                let destination = get_write_index_at(&opcodes, ip, 1);
                opcodes[destination] = input.pop_front().expect("insufficient input provided");
                ip += 2;
            }
            4 => {
                let source = get_read_operand_at(&opcodes, ip, 1);
                output.push(source);
                ip += 2;
            }
            5 => {
                let source = get_read_operand_at(&opcodes, ip, 1);
                if source != 0 {
                    ip = source.try_into().expect("invalid jump address");
                } else {
                    ip += 2;
                }
            }
            6 => {
                let source = get_read_operand_at(&opcodes, ip, 1);
                if source == 0 {
                    ip = source.try_into().expect("invalid jump address");
                } else {
                    ip += 2;
                }
            }
            7 => {
                let (source1, source2, destination) = get_operands_3(&opcodes, &mut ip);
                if source1 < source2 {
                    opcodes[destination] = 1;
                } else {
                    opcodes[destination] = 0;
                }
            }
            8 => {
                let (source1, source2, destination) = get_operands_3(&opcodes, &mut ip);
                if source1 == source2 {
                    opcodes[destination] = 1;
                } else {
                    opcodes[destination] = 0;
                }
            }
            99 => break,
            x => panic!("unexpected opcode found in position {}: {}", ip, x),
        }
    }

    (opcodes, output)
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
    fn io() {
        assert_eq!(
            super::run_with_io(vec!(3, 0, 4, 0, 99), vec!(12345).into()),
            (vec![12345, 0, 4, 0, 99], vec![12345])
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

    #[test]
    fn jumps_and_tests() {
        /* Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not). */
        let prog = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(super::run_with_io(prog.clone(), vec![8].into()).1, vec![1]);
        assert_eq!(super::run_with_io(prog.clone(), vec![7].into()).1, vec![0]);
        assert_eq!(super::run_with_io(prog.clone(), vec![9].into()).1, vec![0]);
        assert_eq!(super::run_with_io(prog.clone(), vec![-1].into()).1, vec![0]);

        /* Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not). */
        let prog = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(super::run_with_io(prog.clone(), vec![8].into()).1, vec![0]);
        assert_eq!(super::run_with_io(prog.clone(), vec![7].into()).1, vec![1]);
        assert_eq!(super::run_with_io(prog.clone(), vec![9].into()).1, vec![0]);
        assert_eq!(super::run_with_io(prog.clone(), vec![-1].into()).1, vec![1]);

        /* Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not). */
        let prog = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(super::run_with_io(prog.clone(), vec![8].into()).1, vec![1]);
        assert_eq!(super::run_with_io(prog.clone(), vec![7].into()).1, vec![0]);
        assert_eq!(super::run_with_io(prog.clone(), vec![9].into()).1, vec![0]);
        assert_eq!(super::run_with_io(prog.clone(), vec![-1].into()).1, vec![0]);

        /* Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not). */
        let prog = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(super::run_with_io(prog.clone(), vec![8].into()).1, vec![0]);
        assert_eq!(super::run_with_io(prog.clone(), vec![7].into()).1, vec![1]);
        assert_eq!(super::run_with_io(prog.clone(), vec![9].into()).1, vec![0]);
        assert_eq!(super::run_with_io(prog.clone(), vec![-1].into()).1, vec![1]);

        /* Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero: */
        let prog = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(super::run_with_io(prog.clone(), vec![0].into()).1, vec![0]);
        assert_eq!(super::run_with_io(prog.clone(), vec![1].into()).1, vec![1]);
        assert_eq!(super::run_with_io(prog.clone(), vec![-1].into()).1, vec![1]);

        let prog = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(super::run_with_io(prog.clone(), vec![0].into()).1, vec![0]);
        assert_eq!(super::run_with_io(prog.clone(), vec![1].into()).1, vec![1]);
        assert_eq!(super::run_with_io(prog.clone(), vec![-1].into()).1, vec![1]);
    }
}
