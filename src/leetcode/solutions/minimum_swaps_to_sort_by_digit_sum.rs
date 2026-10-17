struct Solution;

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let digit_sum = |value: i32| -> i32 {
            let mut value = value;
            let mut result = 0;

            while value > 0 {
                result += value % 10;
                value /= 10;
            }
            result
        };

        let mut entries: Vec<(i32, i32, usize)> = nums
            .iter()
            .enumerate()
            .map(|(index, &value)| (digit_sum(value), value, index))
            .collect();
        let len = entries.len();

        Self::quicksort(&mut entries, 0, len);

        let mut visited = vec![false; len];
        let mut swaps = 0;

        for start in 0..len {
            if visited[start] {
                continue;
            }

            let mut current = start;
            let mut cycle_len = 0;
            while !visited[current] {
                visited[current] = true;
                current = entries[current].2;
                cycle_len += 1;
            }
            swaps += cycle_len - 1;
        }

        swaps
    }

    fn quicksort(nums: &mut Vec<(i32, i32, usize)>, mut low: usize, mut high: usize) {
        while high - low > 1 {
            let partition = Self::partition(nums, low, high);

            if partition - low < high - (partition + 1) {
                Self::quicksort(nums, low, partition);
                low = partition + 1;
            } else {
                Self::quicksort(nums, partition + 1, high);
                high = partition;
            }
        }
    }

    fn partition(nums: &mut [(i32, i32, usize)], low: usize, high: usize) -> usize {
        let pivot_index = high - 1;
        nums.swap(low + (high - low) / 2, pivot_index);
        let pivot = nums[pivot_index];
        let mut i = low;

        for j in low..pivot_index {
            if nums[j] < pivot {
                nums.swap(i, j);
                i += 1;
            }
        }
        nums.swap(i, pivot_index);
        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::min_swaps(vec![37, 100]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_swaps(vec![22, 14, 33, 7]), 0);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::min_swaps(vec![18, 43, 34, 16]), 2);
    }

    #[test]
    fn equal_digit_sums_use_value_as_tiebreaker() {
        assert_eq!(Solution::min_swaps(vec![21, 12]), 1);
    }

    #[test]
    fn three_element_cycle_needs_two_swaps() {
        assert_eq!(Solution::min_swaps(vec![3, 1, 2]), 2);
    }
}
