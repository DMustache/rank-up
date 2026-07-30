mod solution {
    struct RangeFinder<'a> {
        index: usize,
        arr: &'a [i32],
    }

    impl Iterator for RangeFinder<'_> {
        type Item = (i32, Option<i32>);

        fn next(&mut self) -> Option<Self::Item> {
            if self.index == self.arr.len() {
                return None;
            }

            let lo = self.index;
            while self.index < self.arr.len() - 1
                && self.arr[self.index + 1] == self.arr[self.index] + 1
            {
                self.index += 1
            }

            let hi = self.index;
            self.index += 1;

            if hi - lo > 1 {
                Some((self.arr[lo], Some(self.arr[hi])))
            } else {
                if hi - lo == 1 {
                    self.index -= 1
                }
                Some((self.arr[lo], None))
            }
        }
    }

    impl<'a> RangeFinder<'a> {
        fn new(a: &'a [i32]) -> Self {
            RangeFinder { index: 0, arr: a }
        }
    }

    pub fn range_extraction(a: &[i32]) -> String {
        RangeFinder::new(a)
            .map(|(lower, higher)| match higher {
                Some(higher) => format!("{lower}-{higher}"),
                None => lower.to_string(),
            })
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            solution::range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ]),
            "-6,-3-1,3-5,7-11,14,15,17-20"
        );
        assert_eq!(
            solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20]),
            "-3--1,2,10,15,16,18-20"
        );
    }
}
