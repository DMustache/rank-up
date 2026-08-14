use std::collections::{HashMap, VecDeque};

struct Solution;

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();
        let mut left_index = 0;
        let mut max_len = 0;
        let mut positions: HashMap<char, VecDeque<usize>> = HashMap::new();

        for right_index in 0..chars.len() {
            let occurrences = positions.entry(chars[right_index]).or_default();
            while occurrences
                .front()
                .is_some_and(|&position| position < left_index)
            {
                occurrences.pop_front();
            }
            occurrences.push_back(right_index);

            if occurrences.len() > 2 {
                left_index = occurrences.pop_front().unwrap() + 1;
            }

            max_len = max_len.max(right_index - left_index + 1);

            if max_len >= chars.len() - left_index {
                return max_len as i32;
            }
        }

        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::maximum_length_substring("bcbbbcba".to_string()),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::maximum_length_substring("aaaa".to_string()), 2);
    }
}
