struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_special_integers(nums: Vec<i32>) -> i32 {
        let mut positions: HashMap<i32, Vec<usize>> = HashMap::new();

        for (index, value) in nums.into_iter().enumerate() {
            positions.entry(value).or_default().push(index);
        }

    positions
        .values()
        .filter(|positions| positions.len() >= 3 && Self::has_equally_spaced_occurrences(positions))
        .count() as i32
    }

    fn has_equally_spaced_occurrences(positions: &[usize]) -> bool {
            positions
                .windows(3)
                .all(|window| window[1] - window[0] == window[2] - window[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_a_value_with_at_least_three_occurrences() {
        assert_eq!(
            Solution::count_special_integers(vec![1, 2, 1, 3, 1, 4, 1]),
            1
        );
    }

    #[test]
    fn does_not_count_when_only_a_subset_is_equally_spaced() {
        assert_eq!(Solution::count_special_integers(vec![1, 1, 2, 1, 3, 1]), 0);
    }

    #[test]
    fn does_not_count_when_no_triple_is_equally_spaced() {
        assert_eq!(Solution::count_special_integers(vec![1, 1, 2, 1, 1]), 0);
    }

    #[test]
    fn handles_values_with_fewer_than_three_occurrences() {
        assert_eq!(Solution::count_special_integers(vec![1, 2, 1, 2]), 0);
    }

    #[test]
    fn long() {
        assert_eq!(Solution::count_special_integers(vec![12,19,96,36,75,45,46,21,65,100,94,79,31,65,73,9,13,75,98,2,68,28,92,25,37,22,97,69,1,93,13,28,65,91,61,47,90,76,24,57,87,14,24,46,49,55,28,66,30,53,15,98,41,16,60,15,67,18,64,98,47,91,37,72,93,6,11,42,64,15,50,37,59,77,95,40,88,57,29,71,53,38,55,25,9,61,87,48,23,2,87,63,87,14,51,57,67,85,87,11,16,19,51,52,89,76,14,95,22,9,47,26,44,26,77,52,16,30,55,93,64,36,98,92,99,75,76,35,97,83]), 0);
    }
}
