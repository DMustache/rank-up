struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(numbers1: Vec<i32>, numbers2: Vec<i32>) -> f64 {
        if numbers1.len() > numbers2.len() {
            return Solution::find_median_sorted_arrays(numbers2, numbers1);
        }

        let mut lower = 0;
        let mut higher = numbers1.len();

        while lower <= higher {
            let middle1 = (lower + higher) / 2;
            let middle2 = (numbers1.len() + numbers2.len() + 1) / 2 - middle1;

            let left1 = if middle1 == 0 {
                f64::NEG_INFINITY
            } else {
                numbers1[middle1 - 1] as f64
            };

            let right1 = if middle1 == numbers1.len() {
                f64::INFINITY
            } else {
                numbers1[middle1] as f64
            };

            let left2 = if middle2 == 0 {
                f64::NEG_INFINITY
            } else {
                numbers2[middle2 - 1] as f64
            };

            let right2 = if middle2 == numbers2.len() {
                f64::INFINITY
            } else {
                numbers2[middle2] as f64
            };

            if left1 <= right2 && left2 <= right1 {
                if (numbers1.len() + numbers2.len()) % 2 == 0 {
                    return (f64::max(left1, left2) + f64::min(right1, right2)) / 2.0;
                } else {
                    return f64::max(left1, left2);
                }
            }

            if left1 > right2 {
                higher = middle1 - 1;
            } else {
                lower = middle1 + 1;
            }
        }

        return 0.;
    }
}
