use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lock = stdin.lock();

    let mut total_fuel = 0;
    let mut total_fuel_including_fuel_for_the_fuel = 0;

    for line in lock.lines() {
        let line = line.expect("file was not UTF-8");
        let mass = line.parse().expect("could not parse line");

        let mut new_fuel = calculate_fuel(mass);
        total_fuel += new_fuel;
        total_fuel_including_fuel_for_the_fuel += new_fuel;

        loop {
            new_fuel = calculate_fuel(new_fuel);
            if new_fuel <= 0 {
                break;
            }
            total_fuel_including_fuel_for_the_fuel += new_fuel;
        }
    }

    println!("Total fuel: {}", total_fuel);
    println!(
        "Total fuel, including fuel for the fuel: {}",
        total_fuel_including_fuel_for_the_fuel
    );
}

fn calculate_fuel(mass: isize) -> isize {
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
