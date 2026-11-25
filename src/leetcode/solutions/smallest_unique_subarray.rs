use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn smallest_unique_subarray(nums: Vec<i32>) -> i32 {
        const BASE1: u64 = 1_000_000_007;
        const BASE2: u64 = 1_000_000_009;

        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let mut hashes = vec![(0u64, 0u64); n + 1];
        let mut powers = vec![(1u64, 1u64); n + 1];
        for (i, &num) in nums.iter().enumerate() {
            let value = num as u64 + 1;
            hashes[i + 1] = (
                hashes[i].0.wrapping_mul(BASE1).wrapping_add(value),
                hashes[i].1.wrapping_mul(BASE2).wrapping_add(value),
            );
            powers[i + 1] = (
                powers[i].0.wrapping_mul(BASE1),
                powers[i].1.wrapping_mul(BASE2),
            );
        }

        let has_unique = |len: usize| {
            let mut counts = HashMap::new();
            for end in len..=n {
                let start = end - len;
                let key = (
                    hashes[end]
                        .0
                        .wrapping_sub(hashes[start].0.wrapping_mul(powers[len].0)),
                    hashes[end]
                        .1
                        .wrapping_sub(hashes[start].1.wrapping_mul(powers[len].1)),
                );
                *counts.entry(key).or_insert(0) += 1;
            }
            counts.values().any(|&count| count == 1)
        };

        let (mut low, mut high) = (1, n);
        while low < high {
            let mid = low + (high - low) / 2;
            if has_unique(mid) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        low as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let nums: Vec<i32> = vec![3, 3, 3];
        assert_eq!(Solution::smallest_unique_subarray(nums), 3);
    }
    #[test]
    fn example2() {
        let nums: Vec<i32> = vec![2, 1, 2, 3, 3];
        assert_eq!(Solution::smallest_unique_subarray(nums), 1);
    }
    #[test]
    fn example3() {
        let nums: Vec<i32> = vec![1, 1, 2, 2, 1];
        assert_eq!(Solution::smallest_unique_subarray(nums), 2);
    }

    #[test]
    fn large_repeated_array() {
        assert_eq!(
            Solution::smallest_unique_subarray(vec![1; 100_000]),
            100_000
        );
    }
}
