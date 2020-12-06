use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use futures::stream::StreamExt;

fn main() {
    do_main("inputs/day_13.txt");
}

fn do_main(path: &str) {
    let program =
        intcode::parse_opcodes(&std::fs::read_to_string(path).expect("could not read input"));

    let block_tiles = futures::executor::block_on(count_block_tiles(program.clone()));
    println!("Block tiles: {}", block_tiles);
    assert_eq!(block_tiles, 309);

    let score = futures::executor::block_on(get_score(program));
    println!("Score: {}", score);
}

async fn count_block_tiles(program: Vec<isize>) -> usize {
    let tiles = run_game(program, |_| panic!("This program should not take input")).await;
    tiles.iter().filter(|(_, &v)| v == 2 /* block */).count()
}

async fn get_score(mut program: Vec<isize>) -> isize {
    // the problem statement says that the first memory cell should be '2' to play for free.
    program[0] = 2;

    let tiles = run_game(program, |tiles| {
        // find the ball and the paddle within the display
        let ball = tiles
            .iter()
            .filter(|(&_location, &kind)| kind == 4)
            .next()
            .expect("the ball was not found")
            .0;
        let paddle = tiles
            .iter()
            .filter(|(&_location, &kind)| kind == 3)
            .next()
            .expect("the ball was not found")
            .0;

        // and move the joystick in the direction of the ball
        if ball.0 < paddle.0 {
            -1
        } else if ball.0 > paddle.0 {
            1
        } else {
            0
        }
    })
    .await;
    tiles
        .get(&(-1, 0))
        .expect("the score was not written by the intcode program")
        .clone()
}

async fn run_game<F: FnMut(&HashMap<(isize, isize), isize>) -> isize + Unpin + 'static>(
    program: Vec<isize>,
    mut read_input: F,
) -> HashMap<(isize, isize), isize> {
    let tiles = Arc::new(RefCell::new(HashMap::new()));
    let tiles_for_input = tiles.clone();
    let mut intcode = intcode::stream_with_io(
        program,
        Box::new(futures::stream::poll_fn(move |_ctx| {
            futures::task::Poll::Ready(Some(read_input(&tiles_for_input.borrow())))
        })),
    );

    loop {
        let (x, rest) = intcode.into_future().await;
        // check the first opcode to see if it terminated
        let x = match x.expect("intcode interpreter ended unexpectedly") {
            intcode::Status::Terminated(_) => break,
            intcode::Status::Output(x) => x,
        };

        // and if it didn't terminate, assume that the other two to at least produce *something*
        let (y, rest) = rest.into_future().await;
        let (tile, rest) = rest.into_future().await;

        let y = match y.expect("intcode interpreter ended unexpectedly") {
            intcode::Status::Terminated(_) => break,
            intcode::Status::Output(y) => y,
        };
        let tile = match tile.expect("intcode interpreter ended unexpectedly") {
            intcode::Status::Terminated(_) => break,
            intcode::Status::Output(tile) => tile,
        };

        tiles.borrow_mut().insert((x, y), tile);

        intcode = rest;
    }

    Arc::try_unwrap(tiles)
        .expect("the intcode interpreter should no longer reference tiles")
        .into_inner()
}

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::do_main("../inputs/day_13.txt");
    }
}
