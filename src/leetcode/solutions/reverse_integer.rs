struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut result: i32 = 0;

        while x != 0 {
            let digit = x % 10;
            result = match result
                .checked_mul(10)
                .and_then(|value| value.checked_add(digit))
            {
                Some(value) => value,
                None => return 0,
            };
            x /= 10;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let case = 123;
        assert_eq!(321, Solution::reverse(case))
    }

    #[test]
    fn example2() {
        let case = -123;
        assert_eq!(-321, Solution::reverse(case))
    }

    #[test]
    fn example3() {
        let case = 120;
        assert_eq!(21, Solution::reverse(case))
    }

    #[test]
    fn overflow_returns_zero() {
        let case = 1534236469;
        assert_eq!(0, Solution::reverse(case))
    }
}
