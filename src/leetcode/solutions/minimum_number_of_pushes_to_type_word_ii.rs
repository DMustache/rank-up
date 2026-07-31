struct Solution;

use std::collections::HashMap;

impl Solution {
    // we may to use only 2,3,4,5,6,7,8,9 numbers, no need to fill all numbers
    // we build own keypad
    pub fn minimum_pushes(word: String) -> i32 {
        let mut counts = HashMap::new();

        for c in word.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        let mut sorted_counts: Vec<(char, usize)> = counts.into_iter().collect();
        sorted_counts.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        let mut result = 0;
        for (i, (_, count)) in sorted_counts.into_iter().enumerate() {
            let pushes = i / 8 + 1;
            result += count * pushes;
        }

        return result as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let case = "abcde".to_string();
        assert_eq!(5, Solution::minimum_pushes(case))
    }

    #[test]
    fn example2() {
        let case = "xyzxyzxyzxyz".to_string();
        assert_eq!(12, Solution::minimum_pushes(case))
    }

    #[test]
    fn example3() {
        let case = "aabbccddeeffgghhiiiiii".to_string();
        assert_eq!(24, Solution::minimum_pushes(case))
    }

    #[test]
    fn example4() {
        let case = "abzaqsqcyrbzsrvamylmyxdjl".to_string();
        assert_eq!(32, Solution::minimum_pushes(case))
    }
}
