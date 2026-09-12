use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn count_special_integers(nums: Vec<i32>) -> i32 {
        let mut positions: HashMap<i32, Vec<usize>> = HashMap::new();

        for (index, value) in nums.into_iter().enumerate() {
            positions.entry(value).or_default().push(index);
        }

        positions
            .values()
            .filter(|positions| {
                positions.len() == 3 && positions[1] - positions[0] == positions[2] - positions[1]
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_values_with_equally_spaced_occurrences() {
        assert_eq!(Solution::count_special_integers(vec![1, 2, 1, 2, 1]), 1);
    }

    #[test]
    fn ignores_values_with_non_equal_spacing() {
        assert_eq!(Solution::count_special_integers(vec![1, 1, 2, 1, 2, 2]), 0);
    }

    #[test]
    fn ignores_values_that_do_not_occur_exactly_three_times() {
        assert_eq!(Solution::count_special_integers(vec![1, 1, 1, 1, 2, 2]), 0);
    }

    #[test]
    fn handles_an_empty_array() {
        assert_eq!(Solution::count_special_integers(vec![]), 0);
    }
}
