use std::collections::HashSet;

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

fn intersect(a: &[Instruction], b: &[Instruction]) -> Vec<(isize, isize)> {
    let seen_a = follow(a);
    let seen_b = follow(b);
    seen_a.intersection(&seen_b).cloned().collect()
}

fn follow(path: &[Instruction]) -> HashSet<(isize, isize)> {
    let (mut x, mut y) = (0, 0);
    let mut result = HashSet::new();

    for i in path {
        for _ in 0..i.count {
            match i.dir {
                Right => x += 1,
                Left => x -= 1,
                Up => y += 1,
                Down => y -= 1,
            }
            result.insert((x, y));
        }
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

    #[test]
    fn intersect() {
        let mut x = super::intersect(
            &parse_sequence("R8,U5,L5,D3"),
            &parse_sequence("U7,R6,D4,L4"),
        );
        x.sort();
        assert_eq!(x, vec![(3, 3), (6, 5)])
    }
}
