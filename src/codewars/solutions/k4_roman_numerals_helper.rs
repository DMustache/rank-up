// https://www.codewars.com/kata/51b66044bce5799a7f000003/train/rust

const ROMAN_VALUES: [(&str, u32); 13] = [
    ("M", 1000),
    ("CM", 900),
    ("D", 500),
    ("CD", 400),
    ("C", 100),
    ("XC", 90),
    ("L", 50),
    ("XL", 40),
    ("X", 10),
    ("IX", 9),
    ("V", 5),
    ("IV", 4),
    ("I", 1),
];

pub fn to_roman(r: u32) -> String {
    let mut remaining = r;
    let mut roman = String::with_capacity(15);

    for &(symbol, value) in &ROMAN_VALUES {
        while remaining >= value {
            roman.push_str(symbol);
            remaining -= value;
        }
    }

    roman
}

pub fn from_roman(s: &str) -> u32 {
    let mut result = 0;

    let mut sample = s;

    while !sample.is_empty() {
        for &(symbol, value) in ROMAN_VALUES.iter() {
            if sample.starts_with(symbol) {
                result += value;
                sample = &sample[symbol.len()..];
                break;
            }
        }
    }
    result
}

#[cfg(test)]
mod example_tests {
    use super::{from_roman, to_roman};

    fn assert_to_roman(r: u32, expected: &str) {
        let actual = to_roman(r);
        assert_eq!(
            actual, expected,
            "\nYour result (left) did not match the expected output (right)"
        )
    }

    fn assert_from_roman(s: &str, expected: u32) {
        let actual = from_roman(s);
        assert_eq!(
            actual, expected,
            "\nYour result (left) did not match the expected output (right)"
        )
    }

    #[test]
    fn _1_to_roman() {
        assert_to_roman(1000, "M");
        assert_to_roman(4, "IV");
        assert_to_roman(1, "I");
        assert_to_roman(1990, "MCMXC");
        assert_to_roman(2008, "MMVIII");
    }

    #[test]
    fn _2_from_roman() {
        assert_from_roman("XXI", 21);
        assert_from_roman("I", 1);
        assert_from_roman("IV", 4);
        assert_from_roman("MMVIII", 2008);
        assert_from_roman("MDCLXVI", 1666);
    }
}
