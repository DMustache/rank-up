struct Solution;

impl Solution {
    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let total = (n + k - 1) as usize;
        let choose = (2 * k) as usize;

        let mut factorial = vec![1_i64; total + 1];
        for value in 1..=total {
            factorial[value] = factorial[value - 1] * value as i64 % MOD;
        }

        let inverse_factorial_choose = Self::mod_pow(factorial[choose], MOD - 2);
        let inverse_factorial_rest = Self::mod_pow(factorial[total - choose], MOD - 2);

        (factorial[total] * inverse_factorial_choose % MOD * inverse_factorial_rest % MOD) as i32
    }

    fn mod_pow(mut base: i64, mut exponent: i64) -> i64 {
        const MOD: i64 = 1_000_000_007;
        let mut result = 1;

        while exponent > 0 {
            if exponent % 2 == 1 {
                result = result * base % MOD;
            }
            base = base * base % MOD;
            exponent /= 2;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::number_of_sets(4, 2), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::number_of_sets(3, 1), 3);
    }

    #[test]
    fn two_segments_for_five_points() {
        assert_eq!(Solution::number_of_sets(5, 2), 15);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::number_of_sets(30, 7), 796297179);
    }
}
