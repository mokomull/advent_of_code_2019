fn main() {
    do_main();
}

fn do_main() {
    let mut count_1 = 0;
    let mut count_2 = 0;

    for i in 123257..=647015 {
        if check(i) {
            count_1 += 1;
        }

        if check_part_2(i) {
            count_2 += 1;
        }
    }

    println!("Valid passwords: {}", count_1);
    assert_eq!(count_1, 2220);

    println!("Valid passwords (part 2): {}", count_2);
    assert_eq!(count_2, 1515);
}

fn check(i: usize) -> bool {
    let stringy = format!("{}", i);

    adjacent_digits(&stringy) && in_order(&stringy)
}

fn check_part_2(i: usize) -> bool {
    let stringy = format!("{}", i);

    exactly_two_same_digits(&stringy) && in_order(&stringy)
}

fn adjacent_digits(i: &str) -> bool {
    for (i, j) in i.chars().zip(i.chars().skip(1)) {
        if i == j {
            return true;
        }
    }

    false
}

fn exactly_two_same_digits(i: &str) -> bool {
    let mut digit_count = std::collections::HashMap::new();

    for c in i.chars() {
        *digit_count.entry(c).or_insert(0) += 1;
    }

    digit_count.iter().any(|(&_c, &count)| count == 2)
}

fn in_order(i: &str) -> bool {
    let mut last = i
        .chars()
        .nth(0)
        .expect("in_order check for an empty string");

    for c in i.chars().skip(1) {
        if c < last {
            return false;
        }
        last = c;
    }

    true
}

#[cfg(test)]
mod test {
    #[test]
    fn check() {
        assert!(super::check(122345));
        assert!(super::check(111123));
        assert!(super::in_order("135679"));
        assert!(super::check(111111));
        assert!(!super::check(223450));
        assert!(!super::check(123789));
    }

    #[test]
    fn main() {
        super::do_main();
    }

    #[test]
    fn too_many_adjacent_digits() {
        assert!(!super::exactly_two_same_digits("123444"));
        assert!(!super::check_part_2(123444));
        assert!(super::check_part_2(111122));
        assert!(!super::check_part_2(223450));
        assert!(!super::check_part_2(123789));
    }
}
