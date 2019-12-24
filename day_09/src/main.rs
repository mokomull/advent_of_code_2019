fn main() {
    do_main(&std::fs::read_to_string("inputs/day_09.txt").expect("could not read input"));
}

fn do_main(input: &str) {
    let opcodes = intcode::parse_opcodes(input);
    let (_, output) = intcode::run_with_io(opcodes.clone(), vec![1].into());
    assert_eq!(output.len(), 1);
    println!("BOOST keycode: {}", output[0]);
    assert_eq!(output[0], 3460311188);

    let (_, output) = intcode::run_with_io(opcodes, vec![2].into());
    assert_eq!(output.len(), 1);
    println!("Coordinates: {}", output[0]);
    assert_eq!(output[0], 42202);
}

#[test]
fn test_main() {
    do_main(&std::fs::read_to_string("../inputs/day_09.txt").unwrap());
}
