struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut best = vec![0; n + 1];

        for step in 1..=n {
            best[step] = (1..=3)
                .filter(|&jump| jump <= step)
                .map(|jump| best[step - jump] + costs[step - 1] + (jump * jump) as i32)
                .min()
                .unwrap();
        }

        best[n]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example1() {
        let n: i32 = 4;
        let costs: Vec<i32> = vec![1, 2, 3, 4];
        assert_eq!(Solution::climb_stairs(n, costs), 13);
    }

    #[test]
    fn example2() {
        let n: i32 = 4;
        let costs: Vec<i32> = vec![5, 1, 6, 2];

        assert_eq!(Solution::climb_stairs(n, costs), 11);
    }

    #[test]
    fn example3() {
        let n: i32 = 3;
        let costs: Vec<i32> = vec![9, 8, 3];
        assert_eq!(Solution::climb_stairs(n, costs), 12);
    }

    #[test]
    fn one_step() {
        assert_eq!(Solution::climb_stairs(1, vec![7]), 8);
    }
}
