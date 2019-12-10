fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
use Direction::*;

#[derive(Debug, Eq, PartialEq)]
struct Instruction {
    dir: Direction,
    count: isize,
}

fn parse_sequence(input: &str) -> Vec<Instruction> {
    let mut result = Vec::new();

    for i in input.split(',') {
        let dir = match i.chars().next() {
            Some('U') => Up,
            Some('R') => Right,
            Some('D') => Down,
            Some('L') => Left,
            _ => panic!("Unknown instruction: {:?}", i),
        };
        let count = i[1..].parse().expect("non-integer count");
        result.push(Instruction { dir, count });
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parser() {
        fn instruction(dir: Direction, count: isize) -> Instruction {
            Instruction { dir, count }
        }
        assert_eq!(
            parse_sequence("R8,U5,L5,D3"),
            vec![
                instruction(Right, 8),
                instruction(Up, 5),
                instruction(Left, 5),
                instruction(Down, 3),
            ]
        )
    }
}
