fn main() {
    do_main("inputs/day_07.txt");
}

fn do_main(path: &str) {
    let input = std::fs::read_to_string(path).expect("could not read input");
    let program = intcode::parse_opcodes(&input);

    let max_output = find_max(&program);
    println!("Maximum output reached: {}", max_output);
    assert_eq!(max_output, 440880);
}

fn run_thrusters(program: &[isize], phase_settings: &[isize]) -> isize {
    let mut signal = 0;

    for &phase in phase_settings {
        let (_, output) = intcode::run_with_io(program.into(), vec![phase, signal].into());
        assert!(output.len() == 1);
        signal = output[0]
    }

    signal
}

fn find_max(program: &[isize]) -> isize {
    use itertools::Itertools;

    let mut max_output = None;
    for phase_settings in (0..=4).permutations(5) {
        let output = run_thrusters(program, &phase_settings);
        if max_output.is_none() || output > max_output.unwrap() {
            max_output = Some(output);
        }
    }

    max_output.expect("did not produce any output")
}

fn run_thrusters_loop(program: &[isize], phase_settings: &[isize]) -> isize {
    use futures::sink::SinkExt;
    use futures::stream::StreamExt;
    use intcode::Status;

    let mut thrusters = Vec::new();
    for &setting in phase_settings {
        let (mut sender, receiver) = futures::channel::mpsc::channel::<isize>(2);
        sender
            .try_send(setting)
            .expect("could not send phase setting");
        thrusters.push((
            sender,
            intcode::stream_with_io(program.into(), Box::new(receiver)),
        ))
    }
    thrusters[0]
        .0
        .try_send(0)
        .expect("could not start thruster 0");

    let f = async {
        let mut read_thruster = 0;
        let mut write_thruster = 1;
        let mut finished = 0;
        let mut last_output = None;

        loop {
            read_thruster %= thrusters.len();
            write_thruster %= thrusters.len();

            let data = thrusters[read_thruster].1.next().await;

            match data.expect("didn't get data") {
                Status::Output(o) => {
                    thrusters[write_thruster]
                        .0
                        .send(o)
                        .await
                        .expect("could not send to the next thruster");
                    last_output = Some(o);
                }
                Status::Terminated(_) => finished += 1,
            }

            if finished == thrusters.len() {
                break last_output.expect("never received any output");
            }

            read_thruster += 1;
            write_thruster += 1;
        }
    };

    futures::executor::block_on(f)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples() {
        let prog = [
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(run_thrusters(&prog, &[4, 3, 2, 1, 0]), 43210);
        assert_eq!(find_max(&prog), 43210);

        let prog = [
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(run_thrusters(&prog, &[0, 1, 2, 3, 4]), 54321);
        assert_eq!(find_max(&prog), 54321);

        let prog = [
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(run_thrusters(&prog, &[1, 0, 4, 3, 2]), 65210);
        assert_eq!(find_max(&prog), 65210);
    }

    #[test]
    fn loop_thrusters() {
        let prog = [
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(run_thrusters_loop(&prog, &[9, 8, 7, 6, 5]), 139629729);

        let prog = [
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(run_thrusters_loop(&prog, &[9, 7, 8, 5, 6]), 18216);
    }

    #[test]
    fn main() {
        do_main("../inputs/day_07.txt");
    }
}
