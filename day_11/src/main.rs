use futures::stream::StreamExt;
use std::collections::HashMap;

fn main() {
    do_main("inputs/day_11.txt");
}

fn do_main(path: &str) {
    let program =
        intcode::parse_opcodes(&std::fs::read_to_string(path).expect("could not read input"));
    let painted_panels = count_painted_panels(&program);
    println!("The robot painted {} panels", painted_panels);
    assert_eq!(painted_panels, 1732);

    print_painted_panels(&program);
}

fn count_painted_panels(program: &[isize]) -> usize {
    let panels = futures::executor::block_on(paint_panels(program, 0));
    panels.len()
}

fn print_painted_panels(program: &[isize]) {
    let panels = futures::executor::block_on(paint_panels(program, 1));
    let min_x = panels
        .keys()
        .map(|&(x, _)| x)
        .min()
        .expect("no panels were painted");
    let max_x = panels.keys().map(|&(x, _)| x).max().unwrap();
    let min_y = panels.keys().map(|&(_, y)| y).min().unwrap();
    let max_y = panels.keys().map(|&(_, y)| y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = match panels.get(&(x, y)) {
                Some(0) | None => '.',
                Some(1) => '#',
                x => panic!("Unexpected color: {:?}", x),
            };
            print!("{}", c);
        }
        println!();
    }
}

async fn paint_panels(
    program: &[isize],
    first_panel_color: isize,
) -> HashMap<(isize, isize), isize> {
    let (mut tx, rx) = futures::channel::mpsc::channel::<isize>(1);
    let (mut x, mut y) = (0, 0);
    let (mut dx, mut dy) = (0, -1); // originally pointed *up*
    let mut panels = HashMap::<(isize, isize), isize>::new();
    let mut intcode = intcode::stream_with_io(program.iter().cloned().collect(), Box::new(rx));

    panels.insert((0, 0), first_panel_color);

    loop {
        let this_color = *panels.get(&(x, y)).unwrap_or(&0);
        tx.try_send(this_color)
            .expect("too much queued in the mpsc channel");
        let (paint_this, rest) = intcode.into_future().await;
        let (direction, rest) = rest.into_future().await;

        let paint_this =
            match paint_this.expect("intcode interpreter stopped before signaling termination") {
                intcode::Status::Terminated(_) => break,
                intcode::Status::Output(x) => x,
            };

        panels.insert((x, y), paint_this);

        let (new_dx, new_dy) = match direction.expect("intcode program did not yield enough data") {
            intcode::Status::Output(0) => {
                // turn left:
                // [ 0 -1    * ( dx
                //   1  0 ]      dy )
                (-dy, dx)
            }
            intcode::Status::Output(1) => {
                // turn right:
                // [  0 1    * ( dx
                //   -1 0 ]      dy )
                (dy, -dx)
            }
            x => panic!("Unexpected direction: {:?}", x),
        };
        dx = new_dx;
        dy = new_dy;

        x += dx;
        y += dy;

        intcode = rest;
    }

    panels
}

#[cfg(test)]
mod test {
    #[test]
    fn test_main() {
        super::do_main("../inputs/day_11.txt");
    }
}
