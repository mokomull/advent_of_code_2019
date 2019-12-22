fn main() {
    println!("Hello, world!");
}

fn split_into_layers(pixels: &[u8], width: usize, height: usize) -> Vec<Vec<u8>> {
    assert!(pixels.len() % (width * height) == 0);
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
}
