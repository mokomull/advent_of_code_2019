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

    let block_tiles = futures::executor::block_on(count_block_tiles(program));
    println!("Block tiles: {}", block_tiles);
    assert_eq!(block_tiles, 309);
}

async fn count_block_tiles(program: Vec<isize>) -> usize {
    let tiles = run_game(program, |_| panic!("This program should not take input")).await;
    tiles.iter().filter(|(_, &v)| v == 2 /* block */).count()
}

async fn run_game<F: FnMut(&HashMap<(isize, isize), isize>) -> isize + Unpin + 'static>(
    program: Vec<isize>,
    mut read_input: F,
) -> HashMap<(isize, isize), isize> {
    let tiles = Arc::new(RefCell::new(HashMap::new()));
    let tiles_for_input = tiles.clone();
    let mut intcode = intcode::stream_with_io(
        program,
        Box::new(InputStream {
            fun: move || read_input(&tiles_for_input.borrow()),
        }),
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

struct InputStream<F: FnMut() -> isize + Unpin> {
    fun: F,
}

impl<F: FnMut() -> isize + Unpin> futures::Stream for InputStream<F> {
    type Item = isize;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut futures::task::Context<'_>,
    ) -> futures::task::Poll<Option<Self::Item>> {
        futures::task::Poll::Ready(Some((self.fun)()))
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::do_main("../inputs/day_13.txt");
    }
}
