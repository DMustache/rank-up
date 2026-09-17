struct Solution;

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        if arr.is_empty() {
            return -1;
        }

        let mut best = vec![i32::MAX; arr.len()];
        let mut left = 0;
        let mut sum = 0;
        let mut answer = i32::MAX;

        for right in 0..arr.len() {
            sum += arr[right];

            while sum > target && left <= right {
                sum -= arr[left];
                left += 1;
            }

            if right > 0 {
                best[right] = best[right - 1];
            }

            if sum == target {
                let length = (right - left + 1) as i32;
                if left > 0 && best[left - 1] != i32::MAX {
                    answer = answer.min(length + best[left - 1]);
                }
                best[right] = best[right].min(length);
            }
        }

        if answer == i32::MAX { -1 } else { answer }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn finds_two_shortest_non_overlapping_subarrays() {
        assert_eq!(Solution::min_sum_of_lengths(vec![3, 2, 2, 4, 3], 3), 2);
    }

    #[test]
    fn returns_minus_one_when_two_subarrays_do_not_exist() {
        assert_eq!(Solution::min_sum_of_lengths(vec![3, 2, 2, 4, 3], 6), -1);
    }

    #[test]
    fn handles_adjacent_subarrays() {
        assert_eq!(Solution::min_sum_of_lengths(vec![7, 3, 4, 7], 7), 2);
    }
}
