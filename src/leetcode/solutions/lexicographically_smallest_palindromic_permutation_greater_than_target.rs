struct Solution;

impl Solution {
    pub fn lex_palindromic_permutation(s: String, target: String) -> String {
        let n = s.len();
        let half_len = || n / 2;

        let target_bytes = target.as_bytes();

        let mut counts = [0usize; 26];

        for byte in s.bytes() {
            counts[(byte - b'a') as usize] += 1;
        }

        let odd_chars: Vec<usize> = counts
            .iter()
            .enumerate()
            .filter_map(|(ch, &count)| (count % 2 == 1).then_some(ch))
            .collect();

        if odd_chars.len() != n % 2 {
            return String::new();
        }

        let middle = odd_chars.first().map(|&ch| b'a' + ch as u8);

        let mut half_counts = [0usize; 26];

        for ch in 0..26 {
            half_counts[ch] = counts[ch] / 2;
        }

        let mut exact_counts = half_counts;
        let mut exact_half = Vec::with_capacity(half_len());
        let mut exact_match = true;

        for &byte in &target_bytes[..half_len()] {
            let ch = (byte - b'a') as usize;

            if exact_counts[ch] == 0 {
                exact_match = false;
                break;
            }

            exact_counts[ch] -= 1;
            exact_half.push(byte);
        }

        if exact_match {
            let palindrome = Self::build_parlindrome(&exact_half, middle);

            if palindrome.as_bytes() > target_bytes {
                return palindrome;
            }
        }

        let next_half = Self::next_permitation(half_counts, &target_bytes[..half_len()]);

        if next_half.is_empty() {
            return String::new();
        }

        Self::build_parlindrome(&next_half, middle)
    }

    fn build_parlindrome(left: &[u8], middle: Option<u8>) -> String {
        let mut result = Vec::with_capacity(left.len() * 2 + usize::from(middle.is_some()));

        result.extend_from_slice(left);

        if let Some(some_middle) = middle {
            result.push(some_middle);
        }

        result.extend(left.iter().rev());

        String::from_utf8(result).unwrap()
    }

    fn next_permitation(mut counts: [usize; 26], target: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(target.len());

        let mut i = 0;

        while i < target.len() {
            let ch = (target[i] - b'a') as usize;

            if counts[ch] == 0 {
                break;
            }

            counts[ch] -= 1;
            result.push(target[i]);
            i += 1;
        }

        loop {
            if i < target.len() {
                let current = (target[i] - b'a') as usize;

                if let Some(next) = (current + 1..26).find(|&ch| counts[ch] > 0) {
                    counts[next] -= 1;
                    result.push(b'a' + next as u8);

                    for (ch, &count) in counts.iter().enumerate() {
                        result.extend(std::iter::repeat_n(b'a' + ch as u8, count));
                    }
                    return result;
                }
            }

            if i == 0 {
                return Vec::new();
            }

            i -= 1;
            let restored = result.pop().unwrap();
            counts[(restored - b'a') as usize] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "baab",
            solution::lex_palindromic_permutation("baba".to_string(), "abba".to_string())
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "",
            solution::lex_palindromic_permutation("baba".to_string(), "bbaa".to_string())
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            "",
            solution::lex_palindromic_permutation("abc".to_string(), "abb".to_string())
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            "aca",
            solution::lex_palindromic_permutation("aac".to_string(), "abb".to_string())
        );
    }
}
