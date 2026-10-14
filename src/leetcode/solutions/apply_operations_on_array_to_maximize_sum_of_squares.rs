struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut bit_counts = [0; 31];

        for num in nums {
            for (bit, count) in bit_counts.iter_mut().enumerate() {
                *count += (num >> bit) & 1;
            }
        }

        let mut answer = 0_i64;
        for _ in 0..k {
            let mut value = 0_i64;
            for (bit, count) in bit_counts.iter_mut().enumerate() {
                if *count > 0 {
                    value |= 1_i64 << bit;
                    *count -= 1;
                }
            }
            answer = (answer + value * value) % MOD;
        }

        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::max_sum(vec![2, 6, 5, 8], 2), 261);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::max_sum(vec![4, 5, 4, 7], 3), 90);
    }

    #[test]
    fn combines_bits_into_the_largest_value() {
        assert_eq!(Solution::max_sum(vec![1, 2], 1), 9);
    }

    #[test]
    fn applies_modulo_to_large_squares() {
        assert_eq!(Solution::max_sum(vec![1_000_000_000, 1_000_000_000], 2), 98);
    }
}
