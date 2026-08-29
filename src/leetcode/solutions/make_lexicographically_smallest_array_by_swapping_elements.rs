struct Solution;

impl Solution {
    fn transform_group(group: &[(i32, usize)], result: &mut [i32]) {
        let mut indices: Vec<usize> = group.iter().map(|&(_, index)| index).collect();
        indices.sort_unstable();

        for (index, &(value, _)) in indices.into_iter().zip(group) {
            result[index] = value;
        }
    }

    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut sorted: Vec<(i32, usize)> = nums
            .iter()
            .copied()
            .enumerate()
            .map(|(index, value)| (value, index))
            .collect();
        sorted.sort_unstable();

        let mut result = nums;
        let mut group_start = 0;

        while group_start < sorted.len() {
            let mut group_end = group_start + 1;

            while group_end < sorted.len()
                && i64::from(sorted[group_end].0) - i64::from(sorted[group_end - 1].0)
                    <= i64::from(limit)
            {
                group_end += 1;
            }

            Self::transform_group(&sorted[group_start..group_end], &mut result);

            group_start = group_end;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![1, 5, 3, 9, 8], 2),
            vec![1, 3, 5, 8, 9]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![1, 7, 6, 18, 2, 1], 3),
            vec![1, 6, 7, 18, 1, 2]
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![1, 7, 28, 19, 10], 3),
            vec![1, 7, 28, 19, 10]
        );
    }
}
