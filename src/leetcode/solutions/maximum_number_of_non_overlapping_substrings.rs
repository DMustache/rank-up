use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        let chars = {
            let mut map: HashMap<char, Vec<usize>> = HashMap::new();

            for (index, ch) in s.char_indices() {
                map.entry(ch).or_default().push(index);
            }

            map
        };

        let mut intervals = Vec::new();

        for positions in chars.values() {
            let left = positions[0];
            let mut right = *positions.last().unwrap();

            let mut index = left;

            while index <= right {
                let current = s[index..].chars().next().unwrap();
                let current_positions = &chars[&current];

                if current_positions[0] < left {
                    break;
                }

                right = right.max(*current_positions.last().unwrap());
                index += 1;
            }

            if index > right {
                intervals.push((left, right));
            }
        }

        intervals.sort_unstable_by_key(|&(_, right)| right);

        let mut answer = Vec::new();

        let mut previous_right = 0;

        for (left, right) in intervals {
            if answer.is_empty() || left > previous_right {
                answer.push(s[left..=right].to_string());
                previous_right = right;
            }
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut actual = Solution::max_num_of_substrings("adefaddaccc".to_string());
        actual.sort();

        let mut expected = vec!["e".to_string(), "f".to_string(), "ccc".to_string()];
        expected.sort();

        assert_eq!(actual, expected);
    }

    #[test]
    fn example_2() {
        let mut actual = Solution::max_num_of_substrings("abbaccd".to_string());
        actual.sort();

        let mut expected = vec!["d".to_string(), "bb".to_string(), "cc".to_string()];
        expected.sort();

        assert_eq!(actual, expected);
    }
}
