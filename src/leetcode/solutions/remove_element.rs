struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let mut s = [3, 2, 2, 3].to_vec();
        let p = 3;
        assert_eq!(2, Solution::remove_element(&mut s, p));
    }

    #[test]
    fn example2() {
        let mut s = [0, 1, 2, 2, 3, 0, 4, 2].to_vec();
        let p = 2;
        assert_eq!(5, Solution::remove_element(&mut s, p));
    }
}
