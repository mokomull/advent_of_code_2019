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
        let mut asteroids: Vec<_> = self
            .asteroids()
            .into_iter()
            .filter(|&(x2, y2)| (x2, y2) != (x, y))
            .collect();
        asteroids.sort_by_key(|&(x2, y2)| {
            let dx = x2 as isize - x as isize;
            let dy = y2 as isize - y as isize;
            // Sort by "clockwise", then sort the same angle closest-first
            (Rational::new(dx, dy), dy.abs())
        });
        let mut asteroids: std::collections::VecDeque<_> = asteroids.into();
        let mut last_zapped = None;

        for _ in 0..n {
            last_zapped = Some(
                asteroids
                    .pop_front()
                    .expect("no more asteroids can be zapped"),
            );
            for _ in 0..asteroids.len() {
                if Rational::new(
                    asteroids.front().unwrap().0 as isize - x as isize,
                    asteroids.front().unwrap().1 as isize - y as isize,
                ) != Rational::new(
                    last_zapped.unwrap().0 as isize - x as isize,
                    last_zapped.unwrap().1 as isize - y as isize,
                ) {
                    break;
                }

                // Move all of the asteroids that were shadowed by last_zapped to the back of the queue.
                let to_move = asteroids.pop_front().unwrap();
                asteroids.push_back(to_move);
            }
        }

        last_zapped.expect("did not zap any asteroids")
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

#[derive(Eq, PartialEq)]
struct Rational {
    dx: isize,
    dy: isize,
}

impl Rational {
    pub fn new(dx: isize, dy: isize) -> Self {
        let factor = gcd(dx.abs() as usize, dy.abs() as usize) as isize;
        Self {
            dx: dx / factor,
            dy: dy / factor,
        }
    }
}

impl std::cmp::Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.dx == other.dx && self.dy == other.dy {
            return std::cmp::Ordering::Equal;
        }

        // Anything on the the left-side of the origin is "clockwise" of anything on the right half.
        if self.dx < 0 && other.dx > 0 {
            return std::cmp::Ordering::Greater;
        } else if self.dx > 0 && other.dx < 0 {
            return std::cmp::Ordering::Less;
        }

        // Anything directly above the origin is "counter-clockwise" of everything
        if self.dx == 0 && self.dy < 0 {
            return std::cmp::Ordering::Less;
        }
        if other.dx == 0 && other.dy < 0 {
            return std::cmp::Ordering::Greater;
        }

        // Anything directly below the origin is "clockwise" of the right side, and "counter-clockwise" of the left side
        if self.dx == 0 {
            if other.dx > 0 {
                return std::cmp::Ordering::Greater;
            } else {
                return std::cmp::Ordering::Less;
            }
        }
        if other.dx == 0 {
            if self.dx > 0 {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        }

        // Otherwise, we're in the same half-plane.  Within the half-plane, dy/dx increases, from
        // -\infty to 0 to \infty.  Since the sign of self.dx is the same as the sign as other.dx,
        // we can turn dy1/dx1 <?> dy2/dx2 into dy1 * dx2 <?> dy2 * dx1 -- we're effectively
        // multiplying both sides by dx1 * dx2, which is always positive.
        if self.dy * other.dx < other.dy * self.dx {
            return std::cmp::Ordering::Less;
        }
        std::cmp::Ordering::Greater
    }
}

impl std::cmp::PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(std::cmp::Ord::cmp(self, other));
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

        assert_eq!(map.nth_zapped(1), (11, 12));
        assert_eq!(map.nth_zapped(2), (12, 1));
        assert_eq!(map.nth_zapped(3), (12, 2));
        assert_eq!(map.nth_zapped(10), (12, 8));
        assert_eq!(map.nth_zapped(20), (16, 0));
        assert_eq!(map.nth_zapped(50), (16, 9));
        assert_eq!(map.nth_zapped(100), (10, 16));
        assert_eq!(map.nth_zapped(199), (9, 6));
        assert_eq!(map.nth_zapped(200), (8, 2));
        assert_eq!(map.nth_zapped(201), (10, 9));
        assert_eq!(map.nth_zapped(299), (11, 1));
    }

    #[test]
    fn rational_cmp() {
        assert!(Rational::new(1, 1) == Rational::new(2, 2));
        assert!(Rational::new(2, 2) < Rational::new(-2, -2));
    }

    #[test]
    fn main() {
        do_main(&std::fs::read_to_string("../inputs/day_10.txt").unwrap());
    }
}
