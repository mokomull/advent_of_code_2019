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
    let layers = split_into_layers(line.trim().as_bytes(), 25, 6);

    let mut layers_sorted_by_number_of_0s = layers.clone();
    layers_sorted_by_number_of_0s.sort_by_key(|l| l.iter().filter(|&&pixel| pixel == b'0').count());
    let layer = layers_sorted_by_number_of_0s
        .first()
        .expect("somehow there were no layers");
    let ones = layer.iter().filter(|&&pixel| pixel == b'1').count();
    let twos = layer.iter().filter(|&&pixel| pixel == b'2').count();
    println!("# of ones * # of twos is {}", ones * twos);
    assert_eq!(ones * twos, 2210);

    let mut layer = layers[0].clone();
    for lower_layer in layers.iter().skip(1) {
        for i in 0..layer.len() {
            if layer[i] == b'2'
            /* transparent */
            {
                layer[i] = lower_layer[i];
            }
        }
    }

    for row in 0..6 {
        for col in 0..25 {
            match layer[row * 25 + col] {
                b'0' => print!(" "),
                b'1' => print!("#"),
                x => panic!("Unknown pixel {} at {}, {}", x, row, col),
            }
        }
        println!("");
    }
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
