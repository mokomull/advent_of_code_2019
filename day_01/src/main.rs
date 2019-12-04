use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lock = stdin.lock();

    let mut total_fuel = 0;

    for line in lock.lines() {
        let line = line.expect("file was not UTF-8");
        let mass = line.parse().expect("could not parse line");

        total_fuel += calculate_fuel(mass);
    }

    println!("Total fuel: {}", total_fuel);
}

fn calculate_fuel(mass: usize) -> usize {
    mass / 3 - 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_fuel() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }
}
