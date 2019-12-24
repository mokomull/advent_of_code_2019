use std::collections::VecDeque;
use std::convert::TryInto;

use futures::{Stream, StreamExt};

#[derive(Debug)]
pub enum Status {
    Terminated(Vec<isize>),
    Output(isize),
}

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
pub fn run_with_io(opcodes: Vec<isize>, input: VecDeque<isize>) -> (Vec<isize>, Vec<isize>) {
    let status = stream_with_io(opcodes, Box::new(futures::stream::iter(input)));

    let mut output = Vec::new();
    let mut memory = Vec::new();

    for s in futures::executor::block_on_stream(Box::pin(status)) {
        match s {
            Status::Terminated(x) => memory = x,
            Status::Output(x) => output.push(x),
        }
    }

    (memory, output)
}

pub fn stream_with_io(
    opcodes: Vec<isize>,
    input: Box<dyn Stream<Item = isize> + Unpin>,
) -> impl Stream<Item = Status> + Unpin {
    Box::pin(futures::stream::unfold(
        (opcodes, input, 0, 0, false),
        next_opcode,
    ))
}

async fn next_opcode(
    (mut opcodes, mut input, mut ip, mut relative_base, done): (
        Vec<isize>,
        Box<dyn Stream<Item = isize> + Unpin>,
        usize,
        isize,
        bool,
    ),
) -> Option<(
    Status,
    ((
        Vec<isize>,
        Box<dyn Stream<Item = isize> + Unpin>,
        usize,
        isize,
        bool,
    )),
)> {
    if done {
        return None;
    }

    loop {
        match opcodes[ip] % 100 {
            1 => {
                let (source1, source2, destination) =
                    get_operands_3(&mut opcodes, &mut ip, relative_base);
                opcodes[destination] = source1 + source2;
            }
            2 => {
                let (source1, source2, destination) =
                    get_operands_3(&mut opcodes, &mut ip, relative_base);
                opcodes[destination] = source1 * source2;
            }
            3 => {
                let destination = get_write_index_at(&mut opcodes, ip, 1, relative_base);
                opcodes[destination] = input.next().await.expect("insufficient input provided");
                ip += 2;
            }
            4 => {
                let source = get_read_operand_at(&mut opcodes, ip, 1, relative_base);
                return Some((
                    Status::Output(source),
                    (opcodes, input, ip + 2, relative_base, false),
                ));
            }
            5 => {
                let (comparison, target) = get_operands_2(&mut opcodes, &mut ip, relative_base);
                if comparison != 0 {
                    ip = target.try_into().expect("invalid jump address");
                }
            }
            6 => {
                let (comparison, target) = get_operands_2(&mut opcodes, &mut ip, relative_base);
                if comparison == 0 {
                    ip = target.try_into().expect("invalid jump address");
                }
            }
            7 => {
                let (source1, source2, destination) =
                    get_operands_3(&mut opcodes, &mut ip, relative_base);
                if source1 < source2 {
                    opcodes[destination] = 1;
                } else {
                    opcodes[destination] = 0;
                }
            }
            8 => {
                let (source1, source2, destination) =
                    get_operands_3(&mut opcodes, &mut ip, relative_base);
                if source1 == source2 {
                    opcodes[destination] = 1;
                } else {
                    opcodes[destination] = 0;
                }
            }
            9 => {
                let source = get_read_operand_at(&mut opcodes, ip, 1, relative_base);
                relative_base += source;
                ip += 2;
            }
            99 => {
                return Some((
                    Status::Terminated(opcodes),
                    (vec![], input, ip, relative_base, true),
                ))
            }
            x => panic!("unexpected opcode found in position {}: {}", ip, x),
        }
    }
}

fn get_operands_3(
    opcodes: &mut Vec<isize>,
    ip: &mut usize,
    relative_base: isize,
) -> (isize, isize, usize) {
    let source1 = get_read_operand_at(opcodes, *ip, 1, relative_base);
    let source2 = get_read_operand_at(opcodes, *ip, 2, relative_base);
    let destination = get_write_index_at(opcodes, *ip, 3, relative_base);

    *ip += 4;

    (source1, source2, destination)
}

fn get_operands_2(
    opcodes: &mut Vec<isize>,
    ip: &mut usize,
    relative_base: isize,
) -> (isize, isize) {
    let source1 = get_read_operand_at(opcodes, *ip, 1, relative_base);
    let source2 = get_read_operand_at(opcodes, *ip, 2, relative_base);

    *ip += 3;

    (source1, source2)
}

fn get_read_operand_at(
    opcodes: &mut Vec<isize>,
    ip: usize,
    idx: usize,
    relative_base: isize,
) -> isize {
    let source_idx = opcodes[ip + idx];
    match opcodes[ip] / 10isize.pow((idx + 1).try_into().unwrap()) % 10 {
        0 => {
            let source_idx: usize = source_idx.try_into().expect("un-indexable memory offset");
            if source_idx >= opcodes.len() {
                opcodes.resize(source_idx + 1, 0);
            }
            opcodes[source_idx]
        }
        1 => source_idx,
        2 => {
            let source_idx: usize = (source_idx + relative_base)
                .try_into()
                .expect("un-indexable memory offset");
            opcodes[source_idx]
        }
        x => panic!("Invalid parameter mode {} at ip {}", x, ip),
    }
}

fn get_write_index_at(
    opcodes: &mut Vec<isize>,
    ip: usize,
    idx: usize,
    _relative_base: isize,
) -> usize {
    let destination_idx = opcodes[ip + idx];
    let index = match opcodes[ip] / 10isize.pow((idx + 1).try_into().unwrap()) % 10 {
        0 => destination_idx
            .try_into()
            .expect("un-indexable memory offset"),
        x => panic!("Invalid destination parameter mode {} at ip {}", x, ip),
    };
    if index >= opcodes.len() {
        opcodes.resize(index + 1, 0);
    }
    index
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

    #[test]
    fn relative_base() {
        let prog = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let (_, output) = super::run_with_io(prog.clone(), vec![].into());
        assert_eq!(prog, output);

        let prog = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let (_, output) = super::run_with_io(prog, vec![].into());
        assert_eq!(output.len(), 1);
        /* "should output a 16-digit number" */
        assert!(output[0] >= 1000_0000_0000_0000);
        assert!(output[0] <= 9999_9999_9999_9999);

        let prog = vec![104, 1125899906842624, 99];
        let (_, output) = super::run_with_io(prog.clone(), vec![].into());
        /* "should output the large number in the middle" */
        assert_eq!(output, vec![prog[1]]);
    }
}
