struct Solution;

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for len in (1..=n).rev() {
            for start in 0..=(n - len) {
                let arr = &nums[start..(start + len)];

                let product = Self::prod(arr);
                let lcm_gcd =
                    Self::lcm(arr).and_then(|lcm| lcm.checked_mul(i64::from(Self::gcd(arr))));
                if product.is_some() && product == lcm_gcd {
                    return arr.len() as i32;
                }
            }
        }
        0
    }

    fn prod(arr: &[i32]) -> Option<i64> {
        arr.iter().try_fold(1_i64, |result, &element| {
            result.checked_mul(i64::from(element))
        })
    }

    fn gcd(arr: &[i32]) -> i32 {
        let mut result = arr[0];
        for &value in &arr[1..] {
            let (mut a, mut b) = (result, value);
            while b != 0 {
                (a, b) = (b, a % b);
            }
            result = a;
        }
        result
    }

    fn lcm(arr: &[i32]) -> Option<i64> {
        arr.iter().copied().map(i64::from).try_fold(1_i64, |a, b| {
            if a == 0 || b == 0 {
                return Some(0);
            }
            let (mut x, mut y) = (a, b);
            while y != 0 {
                (x, y) = (y, x % y);
            }
            (a / x).checked_mul(b)
        })
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example1() {
        let nums: Vec<i32> = vec![1, 2, 1, 2, 1, 1, 1];
        assert_eq!(Solution::max_length(nums), 5);
    }
    #[test]
    fn example2() {
        let nums: Vec<i32> = vec![2, 3, 4, 5, 6];
        assert_eq!(Solution::max_length(nums), 3);
    }

    #[test]
    fn example3() {
        let nums: Vec<i32> = vec![1, 2, 3, 1, 4, 5, 1];
        assert_eq!(Solution::max_length(nums), 5);
    }

    #[test]
    fn common_divisor_and_large_product() {
        assert_eq!(Solution::max_length(vec![2, 4, 8]), 2);
        assert_eq!(Solution::max_length(vec![2; 70]), 2);
    }
}
