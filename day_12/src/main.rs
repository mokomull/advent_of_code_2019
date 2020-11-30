use std::convert::TryFrom;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Moon {
    x: isize,
    y: isize,
    z: isize,
    vel_x: isize,
    vel_y: isize,
    vel_z: isize,
}

impl TryFrom<&str> for Moon {
    type Error = ();

    fn try_from(input: &str) -> Result<Moon, ()> {
        lazy_static! {
            static ref RE: Regex = Regex::new("^<x=(-?[0-9]+), y=(-?[0-9]+), z=(-?[0-9]+)>$")
                .expect("regex compile failed");
        };

        let captures = RE.captures(input).ok_or(())?;
        let get_int = |i: usize| captures.get(i).unwrap().as_str().parse().or(Err(()));
        Ok(Moon {
            x: get_int(1)?,
            y: get_int(2)?,
            z: get_int(3)?,
            vel_x: 0,
            vel_y: 0,
            vel_z: 0,
        })
    }
}

impl Moon {
    fn apply_gravity(&mut self, other: &Moon) {
        fn helper(velocity: &mut isize, coord: isize, other: isize) {
            if coord > other {
                *velocity -= 1;
            } else if coord < other {
                *velocity += 1;
            }
        }
        helper(&mut self.vel_x, self.x, other.x);
        helper(&mut self.vel_y, self.y, other.y);
        helper(&mut self.vel_z, self.z, other.z);
    }

    fn apply_velocity(&mut self) {
        self.x += self.vel_x;
        self.y += self.vel_y;
        self.z += self.vel_z;
    }
}

fn main() {
    do_main(&std::fs::read_to_string("inputs/day_12.txt").expect("could not read input"));
}

fn do_main(input: &str) {
    use std::io::BufRead;
    let mut moons: Vec<_> = std::io::Cursor::new(input)
        .lines()
        .map(|l| {
            let line = l.unwrap();
            Moon::try_from(line.as_str()).expect(&format!("could not parse {:?}", line))
        })
        .collect();

    for i in 0..moons.len() {
        for j in 0..moons.len() {
            moons[i].apply_gravity(&moons[j]);
        }
    }

    for mut moon in moons {
        moon.apply_velocity();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parser() {
        assert_eq!(
            Moon::try_from("<x=-1, y=0, z=2>"),
            Ok(Moon {
                x: -1,
                y: 0,
                z: 2,
                vel_x: 0,
                vel_y: 0,
                vel_z: 0
            })
        );
        assert_eq!(
            Moon::try_from("<x=2, y=-10, z=-7>"),
            Ok(Moon {
                x: 2,
                y: -10,
                z: -7,
                vel_x: 0,
                vel_y: 0,
                vel_z: 0
            })
        );
        /*
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>
        */
    }

    #[test]
    fn gravity() {
        let mut moon1 = Moon::try_from("<x=-1, y=0, z=2>").unwrap();
        let mut moon2 = Moon::try_from("<x=2, y=-10, z=2>").unwrap();

        moon1.apply_gravity(&moon2);
        moon2.apply_gravity(&moon1);

        assert_eq!(
            moon1,
            Moon {
                x: -1,
                y: 0,
                z: 2,
                vel_x: 1,
                vel_y: -1,
                vel_z: 0,
            }
        );
        assert_eq!(
            moon2,
            Moon {
                x: 2,
                y: -10,
                z: 2,
                vel_x: -1,
                vel_y: 1,
                vel_z: 0,
            }
        )
    }
}
