fn main() {
    do_main("inputs/day_07.txt");
}

fn do_main(path: &str) {
    let input = std::fs::read_to_string(path).expect("could not read input");
    let program = intcode::parse_opcodes(&input);

    let max_output = find_max(&program);
    println!("Maximum output reached: {}", max_output)
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
}
