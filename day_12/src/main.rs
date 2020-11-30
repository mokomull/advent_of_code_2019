use std::convert::TryFrom;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Moon {
    x: isize,
    y: isize,
    z: isize,
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
        })
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parser() {
        assert_eq!(
            Moon::try_from("<x=-1, y=0, z=2>"),
            Ok(Moon { x: -1, y: 0, z: 2 })
        );
        assert_eq!(
            Moon::try_from("<x=2, y=-10, z=-7>"),
            Ok(Moon {
                x: 2,
                y: -10,
                z: -7
            })
        );
        /*
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>
        */
    }
}
