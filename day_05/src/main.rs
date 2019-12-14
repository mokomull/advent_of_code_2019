fn main() {
    do_main("inputs/day_05.txt");
}

fn do_main(path: &str) {
    let program =
        intcode::parse_opcodes(&std::fs::read_to_string(path).expect("could not read input"));
    let input = vec![1]; // the flavortext says there is exactly one input, value 1

    let (_memory, output) = intcode::run_with_io(program, input.into());

    // all but the last output should be 0, as stated in the flavortext
    assert!(output.iter().take(output.len() - 1).all(|&x| x == 0));
    let diagnostic_code = output.last().expect("empty output");
    println!("Diagnostic code: {}", diagnostic_code);
    assert_eq!(*diagnostic_code, 13294380);
}

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::do_main("../inputs/day_05.txt");
    }
}
