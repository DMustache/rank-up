struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut x = x;
        let mut reversed_half = 0;
        while x > reversed_half {
            let last_dight = x % 10;
            reversed_half = reversed_half * 10 + last_dight;
            x /= 10;
        }

        x == reversed_half || x == reversed_half / 10
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let case = 121;

        assert_eq!(true, Solution::is_palindrome(case))
    }

    #[test]
    fn example1() {
        let case = -121;

        assert_eq!(false, Solution::is_palindrome(case))
    }

    #[test]
    fn example2() {
        let case = 10;

        assert_eq!(false, Solution::is_palindrome(case))
    }
}
