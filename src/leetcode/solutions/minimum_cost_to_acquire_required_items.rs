struct Solution;

impl Solution {
    pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
        let base = need1.min(need2) as i64;
        let abs_diff = (need1 - need2).abs() as i64;

        base * (cost1 as i64 + cost2 as i64).min(cost_both as i64)
            + abs_diff * (if need1 > need2 { cost1 } else { cost2 } as i64).min(cost_both as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let cost1: i32 = 3;
        let cost2: i32 = 2;
        let cost_both: i32 = 1;
        let need1: i32 = 3;
        let need2: i32 = 2;
        assert_eq!(
            Solution::minimum_cost(cost1, cost2, cost_both, need1, need2),
            3
        );
    }

    #[test]
    fn example2() {
        let cost1: i32 = 5;
        let cost2: i32 = 4;
        let cost_both: i32 = 15;
        let need1: i32 = 2;
        let need2: i32 = 3;
        assert_eq!(
            Solution::minimum_cost(cost1, cost2, cost_both, need1, need2),
            22
        );
    }

    #[test]
    fn example3() {
        let cost1: i32 = 5;
        let cost2: i32 = 4;
        let cost_both: i32 = 15;
        let need1: i32 = 0;
        let need2: i32 = 0;
        assert_eq!(
            Solution::minimum_cost(cost1, cost2, cost_both, need1, need2),
            0
        );
    }
}
