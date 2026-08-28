struct Solution;

use std::{
    collections::HashMap,
    hash::{BuildHasherDefault, Hasher},
};

#[derive(Default)]
struct CharHasher(u64);

impl Hasher for CharHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0 = bytes.first().copied().unwrap_or(0) as u64;
    }

    fn write_u8(&mut self, value: u8) {
        self.0 = value as u64;
    }
}

type CharMap = HashMap<u8, usize, BuildHasherDefault<CharHasher>>;

impl Solution {
    pub fn lex_greater_permutation(s: String, target: String) -> String {
        let target = target.as_bytes();
        let mut count = [0usize; 26];

        for ch in s.bytes() {
            count[(ch - b'a') as usize] += 1;
        }

        let mut result = Vec::with_capacity(target.len());
        let mut i = 0;

        while i < target.len() {
            let ch = (target[i] - b'a') as usize;

            if count[ch] == 0 {
                break;
            }

            count[ch] -= 1;
            result.push(target[i]);
            i += 1;
        }

        loop {
            if i < target.len() {
                let current = (target[i] - b'a') as usize;

                if let Some(next) = (current + 1..26).find(|&ch| count[ch] > 0) {
                    count[next] -= 1;
                    result.push(b'a' + next as u8);

                    for (ch, _) in count.iter().enumerate() {
                        result.extend(std::iter::repeat_n(b'a' + ch as u8, count[ch]));
                    }

                    return String::from_utf8(result).unwrap();
                }
            }

            if i == 0 {
                return String::new();
            }

            i -= 1;
            let restored = result.pop().unwrap();
            count[(restored - b'a') as usize] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "bca",
            Solution::lex_greater_permutation("abc".to_string(), "bba".to_string())
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "eelt",
            Solution::lex_greater_permutation("leet".to_string(), "code".to_string())
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            "",
            Solution::lex_greater_permutation("baba".to_string(), "bbaa".to_string())
        );
    }
}
