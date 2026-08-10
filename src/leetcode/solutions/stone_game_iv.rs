struct Solution;

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let mut is_loss_position = vec![false; (n + 1) as usize];
        for i in 1..=n as usize {
            let mut j: usize = 1;
            let mut j_square = j.pow(2);
            while j_square <= i {
                if !is_loss_position[i - j_square] {
                    is_loss_position[i] = true;
                    break;
                }
                j += 1;
                j_square = j.pow(2);
            }
        }
        is_loss_position[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(true, Solution::winner_square_game(1))
    }

    #[test]
    fn example2() {
        assert_eq!(false, Solution::winner_square_game(2))
    }

    #[test]
    fn example3() {
        assert_eq!(true, Solution::winner_square_game(4))
    }
}
