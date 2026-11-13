use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as i64;
        let mut result = 0i64;
        // сколько операций нужно, чтобы окно [left..=right] стало неубывающим
        let mut cost = 0i64;
        let mut right = nums.len() - 1;
        // индексы префиксных максимумов окна: спереди ближайший к left, значения растут к концу.
        // каждый рекорд «держит» на своей высоте отрезок до следующего рекорда (или до right)
        let mut records: VecDeque<usize> = VecDeque::new();

        for left in (0..nums.len()).rev() {
            // новый левый элемент поднимает до себя все отрезки рекордов, которые не выше него
            while let Some(&index) = records.front() {
                if nums[index] > nums[left] {
                    break;
                }
                records.pop_front();
                let next = records.front().copied().unwrap_or(right + 1);
                cost += (nums[left] - nums[index]) as i64 * (next - index) as i64;
            }
            records.push_front(left);

            // сжимаем окно справа: правый элемент был поднят до последнего рекорда
            while cost > k {
                let last = *records.back().unwrap();
                cost -= (nums[last] - nums[right]) as i64;
                if last == right {
                    records.pop_back();
                }
                right -= 1;
            }

            // все подмассивы, начинающиеся в left и заканчивающиеся не дальше right, подходят
            result += (right - left + 1) as i64;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example1() {
        let nums: Vec<i32> = vec![6, 3, 1, 2, 4, 4];
        assert_eq!(Solution::count_non_decreasing_subarrays(nums, 7), 17);
    }

    #[test]
    fn example2() {
        let nums: Vec<i32> = vec![6, 3, 1, 3, 6];
        assert_eq!(Solution::count_non_decreasing_subarrays(nums, 4), 12);
    }
}
