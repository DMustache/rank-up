struct Solution;

impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut count = [0;10];
        for dight in digits {
            count[dight as usize] += 1;
        }

        let mut total = 0;

        for num in (100..=999).step_by(2) {
            let mut num_count = [0;10];
            let mut temp = num;

            while temp > 0 {
                num_count[(temp % 10) as usize] += 1;
                temp /= 10;
            }

            let mut possible = true;
            for i in 0..10 {
                if num_count[i] > count[i] {
                    possible = false;
                    break;
                }
            }

            if possible {total += 1;}
        }

        total
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(12, Solution::total_numbers(vec![1,2,3,4]))
    }

    #[test]
    fn example2() {
        assert_eq!(1, Solution::total_numbers(vec![6,6,6]))
    }

    #[test]
    fn example3() {
        assert_eq!(2, Solution::total_numbers(vec![0, 2,2]))
    }

    #[test]
    fn exampl4() {
        assert_eq!(0, Solution::total_numbers(vec![1,3,5]))
    }
}
