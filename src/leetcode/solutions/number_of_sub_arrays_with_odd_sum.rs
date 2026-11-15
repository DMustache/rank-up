struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut prefix_parity = 0;
        let mut prefix_counts = [1_i64, 0];
        let mut answer = 0_i64;

        for value in arr {
            prefix_parity ^= (value & 1) as usize;
            answer = (answer + prefix_counts[1 - prefix_parity]) % MOD;
            prefix_counts[prefix_parity] += 1;
        }

        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn examples() {
        assert_eq!(Solution::num_of_subarrays(vec![1, 3, 5]), 4);
        assert_eq!(Solution::num_of_subarrays(vec![2, 4, 6]), 0);
        assert_eq!(Solution::num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7]), 16);
    }

    #[test]
    fn handles_zero_and_negative_values() {
        assert_eq!(Solution::num_of_subarrays(vec![0, -1, 2]), 4);
    }

    #[test]
    fn reduces_large_counts_modulo() {
        assert_eq!(Solution::num_of_subarrays(vec![1; 100_000]), 500_049_986);
    }
}
