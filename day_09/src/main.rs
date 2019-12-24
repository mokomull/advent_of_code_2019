fn main() {
    do_main(&std::fs::read_to_string("inputs/day_09.txt").expect("could not read input"));
}

fn do_main(input: &str) {
    let opcodes = intcode::parse_opcodes(input);
    let (_, output) = intcode::run_with_io(opcodes, vec![1].into());
    assert_eq!(output.len(), 1);
    println!("BOOST keycode: {}", output[0]);
}
