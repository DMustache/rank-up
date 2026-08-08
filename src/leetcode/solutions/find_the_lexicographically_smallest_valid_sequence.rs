struct Solution;

impl Solution {
    pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();

        if word2.len() > word1.len() {
            return Vec::new();
        }

        let suffix_match_indexes = Self::build_suffix_match_indexes(word1, word2);
        let mut result = Vec::with_capacity(word2.len());
        let mut word1_index = 0;
        let mut mismatch_used = false;

        for word2_index in 0..word2.len() {
            while word1_index < word1.len() {
                if word1[word1_index] == word2[word2_index] {
                    result.push(word1_index as i32);
                    word1_index += 1;
                    break;
                }

                let suffix_start = suffix_match_indexes[word2_index + 1];
                let can_use_mismatch =
                    !mismatch_used && suffix_start != usize::MAX && suffix_start > word1_index;

                if can_use_mismatch {
                    result.push(word1_index as i32);
                    word1_index += 1;
                    mismatch_used = true;
                    break;
                }

                word1_index += 1;
            }

            if result.len() != word2_index + 1 {
                return Vec::new();
            }
        }

        result
    }

    fn build_suffix_match_indexes(word1_bytes: &[u8], word2_bytes: &[u8]) -> Vec<usize> {
        let mut suffix_match_indexes = vec![usize::MAX; word2_bytes.len() + 1];
        suffix_match_indexes[word2_bytes.len()] = word1_bytes.len();

        let mut word1_end = word1_bytes.len();

        for word2_index in (0..word2_bytes.len()).rev() {
            while word1_end > 0 && word1_bytes[word1_end - 1] != word2_bytes[word2_index] {
                word1_end -= 1;
            }

            if word1_end == 0 {
                break;
            }

            let matched_index = word1_end - 1;
            suffix_match_indexes[word2_index] = matched_index;
            word1_end = matched_index;
        }

        suffix_match_indexes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::valid_sequence("vbcca".to_string(), "abc".to_string()),
            vec![0, 1, 2]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::valid_sequence("bacdc".to_string(), "abc".to_string()),
            vec![1, 2, 4]
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::valid_sequence("aaaaaa".to_string(), "aaabc".to_string()),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            Solution::valid_sequence("abc".to_string(), "ab".to_string()),
            vec![0, 1]
        );
    }
}
