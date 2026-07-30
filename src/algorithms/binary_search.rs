/// # Binary Search
/// input: sorted array of integers
/// output: index of item or fail result
pub fn binary_search<T: Ord>(array: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = array.len();

    while low < high {
        let middle = low + (high - low) / 2;

        match array[middle].cmp(&target) {
            std::cmp::Ordering::Less => low = middle + 1,
            std::cmp::Ordering::Equal => return Some(middle),
            std::cmp::Ordering::Greater => high = middle,
        }
    }

    None
}

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        match binary_search(&nums, &target) {
            Some(index) => index as i32,
            None => -1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Solution, binary_search};

    #[test]
    fn test_binary_search() {
        let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(binary_search(&array, &3), Some(2));
    }

    #[test]
    fn test_binary_search_not_found() {
        let array = [-1, 0, 3, 5, 9, 12];
        assert_eq!(Solution::search(array.to_vec(), 9), 4);
    }
}
