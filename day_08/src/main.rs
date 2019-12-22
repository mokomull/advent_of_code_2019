fn main() {
    do_main(std::fs::File::open("inputs/day_08.txt").expect("could not open input"));
}

fn do_main<T: std::io::Read>(input: T) {
    use std::io::BufRead;

    let mut reader = std::io::BufReader::new(input);
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("could not read a line from input");

    let mut layers = split_into_layers(line.trim().as_bytes(), 25, 6);
    layers.sort_by_key(|l| l.iter().filter(|&&pixel| pixel == b'0').count());
    let layer = layers.first().expect("somehow there were no layers");
    let ones = layer.iter().filter(|&&pixel| pixel == b'1').count();
    let twos = layer.iter().filter(|&&pixel| pixel == b'2').count();
    println!("# of ones * # of twos is {}", ones * twos);
    assert_eq!(ones * twos, 2210);
}

fn split_into_layers(pixels: &[u8], width: usize, height: usize) -> Vec<Vec<u8>> {
    assert_eq!(pixels.len() % (width * height), 0);
    pixels
        .chunks_exact(width * height)
        .map(Into::into)
        .collect()
}

#[cfg(test)]
mod test {
    #[test]
    fn split_into_layers() {
        assert_eq!(
            super::split_into_layers(b"123456789012", 3, 2),
            vec![
                vec![b'1', b'2', b'3', b'4', b'5', b'6'],
                vec![b'7', b'8', b'9', b'0', b'1', b'2']
            ]
        );
    }

    #[test]
    fn main() {
        super::do_main(std::fs::File::open("../inputs/day_08.txt").unwrap());
    }
}
