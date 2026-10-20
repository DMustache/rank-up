impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut answer = 0;

        for k in 1..=n {
            if !n.is_multiple_of(k) {
                continue;
            }

            let mut previous_max = None;
            let mut sortable = true;

            for block in nums.chunks(k) {
                let drops = block.windows(2).filter(|pair| pair[0] > pair[1]).count();
                if drops > 1 || (drops == 1 && block[k - 1] > block[0]) {
                    sortable = false;
                    break;
                }

                let block_min = *block.iter().min().unwrap();
                let block_max = *block.iter().max().unwrap();
                if previous_max.is_some_and(|max| max > block_min) {
                    sortable = false;
                    break;
                }
                previous_max = Some(block_max);
            }

            if sortable {
                answer += k as i32;
            }
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::sortable_integers(vec![3, 1, 2]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::sortable_integers(vec![7, 6, 5]), 0);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::sortable_integers(vec![5, 8]), 3);
    }
}
