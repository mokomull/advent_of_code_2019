use std::convert::{TryFrom, TryInto};

fn main() {
    println!("Hello, world!");
}

struct Map {
    asteroids: Vec<Vec<bool>>,
}

impl TryFrom<&str> for Map {
    type Error = ();

    fn try_from(input: &str) -> Result<Map, Self::Error> {
        let mut asteroids = Vec::new();

        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                let asteroid = match c {
                    '#' => true,
                    '.' => false,
                    _ => return Err(()),
                };
                row.push(asteroid);
            }
            asteroids.push(row);
        }
        
        Ok(Map { asteroids })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parser() {
        let map: Map = ".#..#
.....
#####
....#
...##
".try_into().expect("could not parse map");
    }
}