use std::convert::{TryFrom, TryInto};

fn main() {
    do_main(&std::fs::read_to_string("inputs/day_10.txt").expect("could not read input"));
}

fn do_main(input: &str) {
    let map: Map = input.try_into().expect("could not parse input");
    println!("Most asteroids visible: {}", map.most_visible().0);
    assert_eq!(map.most_visible().0, 230);
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

impl Map {
    pub fn contains_asteroid(&self, x: usize, y: usize) -> bool {
        self.asteroids[y][x]
    }

    pub fn asteroids(&self) -> impl IntoIterator<Item = (usize, usize)> {
        let asteroid_vec: Vec<_> = self
            .asteroids
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, &col)| if col { Some((x, y)) } else { None })
            })
            .collect();
        asteroid_vec
    }

    pub fn count_from(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        for (x2, y2) in self.asteroids() {
            if (x, y) == (x2, y2) {
                continue;
            }

            if self.can_see(x, y, x2, y2) {
                count += 1;
            }
        }

        count
    }

    pub fn most_visible(&self) -> (usize, (usize, usize)) {
        self.asteroids()
            .into_iter()
            .map(|(x, y)| (self.count_from(x, y), (x, y)))
            .max()
            .expect("no asteroids were found")
    }

    pub fn nth_zapped(&self, n: usize) -> (usize, usize) {
        let (_, (x, y)) = self.most_visible();
        let mut map = self.clone();
        let mut asteroids: Vec<_> = self.asteroids().into_iter().collect();
        asteroids.sort_by_key(|&(x2, y2)| Rational::new(x2 as isize - x as isize, y2 as isize - y as isize));
        unimplemented!()
    }

    fn can_see(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        let dx = x2 as isize - x1 as isize;
        let dy = y2 as isize - y1 as isize;

        // step by the minimal rise/run that will reach (2) from (1)
        let factor = gcd(dx.abs() as usize, dy.abs() as usize);
        let dx = dx / factor as isize;
        let dy = dy / factor as isize;

        let (mut x, mut y) = (x1 as isize, y1 as isize);

        while (x, y) != (x2 as isize - dx, y2 as isize - dy) {
            x += dx;
            y += dy;

            if self.contains_asteroid(x as usize, y as usize) {
                return false;
            }
        }

        true
    }
}

#[derive(PartialEq)]
struct Rational {
    dx: isize,
    dy: isize,
}

impl Rational {
    pub fn new(dx: isize, dy: isize) -> Self {
        let factor = gcd(dx.abs() as usize, dy.abs() as usize);
        Self {
            dx: dx / factor as isize,
            dy: dy / factor as isize,
        }
    }
}

impl std::cmp::PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if a > b {
        return gcd(b, a);
    }

    if a == 0 {
        return b;
    }

    gcd(b % a, a)
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
"
        .try_into()
        .expect("could not parse map");

        assert!(!map.can_see(3, 4, 1, 0));
        assert!(!map.can_see(1, 0, 3, 4));
        assert!(map.can_see(3, 4, 4, 0));
        assert!(map.can_see(4, 0, 3, 4));
        assert_eq!(map.count_from(3, 4), 8);
        assert_eq!(map.most_visible().0, 8);

        let map: Map = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
"
        .try_into()
        .unwrap();
        assert_eq!(map.most_visible().0, 33);

        let map: Map = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
"
        .try_into()
        .unwrap();
        assert_eq!(map.most_visible().0, 35);

        let map: Map = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."
            .try_into()
            .unwrap();
        assert_eq!(map.most_visible().0, 41);

        let map: Map = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
"
        .try_into()
        .unwrap();
        assert_eq!(map.most_visible().0, 210);
    }

    #[test]
    fn zapping() {
        let map: Map = ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##
"
        .try_into()
        .unwrap();
        assert_eq!(map.nth_zapped(1), (8, 1));
    }

    #[test]
    fn main() {
        do_main(&std::fs::read_to_string("../inputs/day_10.txt").unwrap());
    }
}
