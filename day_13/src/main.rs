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
    let (_tx, rx) = futures::channel::mpsc::channel::<isize>(1);
    let mut intcode = intcode::stream_with_io(program, Box::new(rx));
    let mut tiles = std::collections::HashMap::new();

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

        tiles.insert((x, y), tile);

        intcode = rest;
    }

    tiles.iter().filter(|(&k, &v)| v == 2 /* block */).count()
}

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::do_main("../inputs/day_13.txt");
    }
}
