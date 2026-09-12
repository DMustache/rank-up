struct Solution;

impl Solution {
    pub fn min_days(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }

        let n = n as usize;
        let mut memoisation = vec![0; n + 1];
        let mut score = 1;

        while score <= n {
            let mut best = i32::MAX;
            let mut streak_score = 0;
            let mut streak_days = 0;

            while streak_score < score {
                streak_days += 1;
                streak_score += streak_days;

                if streak_score > score {
                    break;
                }

                let days = streak_days
                    + if streak_score == score {
                        0
                    } else {
                        1 + memoisation[score - streak_score] as usize
                    };
                best = best.min(days as i32);
            }

            memoisation[score] = best;
            score += 1;
        }

        memoisation[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::min_days(2), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::min_days(9), 6);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::min_days(12), 7);
    }
}
