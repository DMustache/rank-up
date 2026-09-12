struct Solution;

use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, PartialEq)]
struct ModNum(i64);

impl ModNum {
    fn new(value: i64) -> Self {
        Self(value.rem_euclid(1_000_000_007))
    }
}

impl Add for ModNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ModNum::new(self.0 + rhs.0)
    }
}

impl Mul for ModNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        ModNum::new(self.0 * rhs.0)
    }
}

impl Sub for ModNum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        ModNum::new(self.0 - rhs.0)
    }
}

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let mut ending = [ModNum::new(0); 26];
        let mut total = ModNum::new(0);

        for byte in s.bytes() {
            let i = (byte - b'a') as usize;
            let new_ending = total + ModNum::new(1);

            total = total + new_ending - ending[i];
            ending[i] = new_ending;
        }

        total.0 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::distinct_subseq_ii("abc".to_string()), 7);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::distinct_subseq_ii("aba".to_string()), 6);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::distinct_subseq_ii("aaa".to_string()), 3);
    }
}
