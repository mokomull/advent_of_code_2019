fn main() {
    println!("Hello, world!");
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            run_thrusters(
                &[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
                &[4, 3, 2, 1, 0]
            ),
            43210
        );

        assert_eq!(
            run_thrusters(
                &[
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                ],
                &[0, 1, 2, 3, 4]
            ),
            54321
        );

        assert_eq!(
            run_thrusters(
                &[
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
                ],
                &[1, 0, 4, 3, 2]
            ),
            65210
        );
    }
}
