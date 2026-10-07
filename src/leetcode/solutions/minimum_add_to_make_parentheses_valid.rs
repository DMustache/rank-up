use std::ops::{Add, AddAssign, Sub, SubAssign};

struct Solution;

#[repr(Rust, packed(16))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct U10 {
    low: u8,
    high: u8,
}

impl U10 {
    const ZERO: Self = Self { low: 0, high: 0 };
    const ONE: Self = Self { low: 1, high: 0 };

    fn new(value: u16) -> Self {
        assert!(value <= 1023, "U10 overflow");
        Self {
            low: value as u8,
            high: (value >> 8) as u8,
        }
    }

    fn as_u16(self) -> u16 {
        u16::from(self.low) | (u16::from(self.high) << 8)
    }
}

impl Add for U10 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.as_u16() + rhs.as_u16())
    }
}

impl Sub for U10 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(
            self.as_u16()
                .checked_sub(rhs.as_u16())
                .expect("U10 underflow"),
        )
    }
}

impl AddAssign for U10 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for U10 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl From<U10> for i32 {
    fn from(value: U10) -> Self {
        i32::from(value.as_u16())
    }
}

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut brackets_to_close = U10::ZERO;
        let mut result = U10::ZERO;

        for bracket in s.bytes() {
            match bracket {
                b'(' => brackets_to_close += U10::ONE,
                b')' => {
                    if brackets_to_close == U10::ZERO {
                        result += U10::ONE;
                    } else {
                        brackets_to_close -= U10::ONE;
                    }
                }
                _ => unreachable!(),
            }
        }

        i32::from(result + brackets_to_close)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u10_arithmetic_and_size() {
        assert_eq!(std::mem::size_of::<U10>(), 2);
        assert_eq!(i32::from(U10::new(255) + U10::ONE), 256);
        assert_eq!(i32::from(U10::new(512) - U10::ONE), 511);
        assert_eq!(i32::from(U10::new(1023)), 1023);
    }

    #[test]
    fn example1() {
        assert_eq!(Solution::min_add_to_make_valid("())".to_string()), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_add_to_make_valid("(((".to_string()), 3);
    }

    #[test]
    fn already_valid_and_unmatched_parentheses() {
        for (input, expected) in [
            ("()", 0),
            ("(())", 0),
            (")(", 2),
            ("()))((", 4),
            ("(()))(", 2),
        ] {
            assert_eq!(
                Solution::min_add_to_make_valid(input.to_string()),
                expected,
                "{input}"
            );
        }
    }
}
