struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current = Vec::new();

        Self::backtrack(&candidates, target, 0, &mut current, &mut result);
        result
    }

    fn backtrack(
        candidates: &[i32],
        target: i32,
        start: usize,
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            result.push(current.clone());
            return;
        }
        if target < 0 {
            return;
        }

        for i in start..candidates.len() {
            current.push(candidates[i]);

            Self::backtrack(candidates, target - candidates[i], i, current, result);
            current.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let s = [2, 3, 6, 7].to_vec();
        let p = 7;
        assert_eq!(
            [[2, 2, 3].to_vec(), [7].to_vec()].to_vec(),
            Solution::combination_sum(s, p)
        );
    }

    #[test]
    fn example2() {
        let s = [2, 3, 5].to_vec();
        let p = 8;
        assert_eq!(
            [[2, 2, 2, 2].to_vec(), [2, 3, 3].to_vec(), [3, 5].to_vec()].to_vec(),
            Solution::combination_sum(s, p)
        );
    }

    #[test]
    fn example3() {
        let s = [2].to_vec();
        let p = 1;
        let result: Vec<Vec<i32>> = [].to_vec();
        assert_eq!(result, Solution::combination_sum(s, p))
    }
}
