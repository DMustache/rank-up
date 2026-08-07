//! A lazy prime-number stream implemented with a segmented sieve.
//!
//! Checking every candidate by trial division becomes too slow when the
//! iterator is advanced tens of millions of times. This implementation uses
//! the following algorithm:
//!
//! 1. `base_primes` is created once with the ordinary Sieve of Eratosthenes.
//!    It contains every prime up to `65_536`, which is enough to mark all
//!    composite `u32` candidates because `sqrt(u32::MAX) < 65_536`.
//! 2. The iterator yields `2` separately. Every later segment contains only
//!    odd numbers, so even numbers never need to be stored or tested.
//! 3. `fill_segment` creates a block of one million possible odd numbers and
//!    initially marks all of them as prime.
//! 4. For every base prime, marking starts at `max(prime * prime, first
//!    multiple in the segment)`. Only odd multiples are marked, because the
//!    segment contains only odd candidates.
//! 5. `next` walks the marked block and yields the entries still marked prime.
//!    When the block is exhausted, the next block is generated on demand.
//!
//! The base sieve uses `O(sqrt(n))` memory. The streaming part uses constant
//! memory for each segment, while retaining only the base primes. Composite
//! marking is approximately `O(n log log n)` overall, without storing all
//! numbers up to the requested prime.

struct PrimeNumbers {
    current: u32,
    base_primes: Vec<u32>,
    segment_start: u32,
    segment: Vec<bool>,
    segment_index: usize,
}

impl Iterator for PrimeNumbers {
    type Item = u32;

    /// Returns the next prime, generating a new segment when necessary.
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == 1 {
            self.current = 2;
            return Some(2);
        }

        loop {
            if self.segment_index == self.segment.len() {
                self.fill_segment()?;
            }

            if self.segment[self.segment_index] {
                self.current = self
                    .segment_start
                    .checked_add((self.segment_index as u32) * 2)?;
                self.segment_index += 1;
                return Some(self.current);
            }

            self.segment_index += 1;
        }
    }
}

impl PrimeNumbers {
    /// Builds and marks the next block of odd candidate numbers.
    fn fill_segment(&mut self) -> Option<()> {
        const SEGMENT_LENGTH: usize = 1_000_000;

        self.segment_start = self
            .segment_start
            .checked_add((self.segment.len() as u32) * 2)?;
        self.segment = vec![true; SEGMENT_LENGTH];
        self.segment_index = 0;

        for &prime in &self.base_primes {
            if prime == 2 {
                continue;
            }

            let prime_square = prime.checked_mul(prime)?;
            if prime_square >= self.segment_start + (SEGMENT_LENGTH as u32 * 2) {
                break;
            }

            let first_multiple = ((self.segment_start + prime - 1) / prime).max(prime);
            let mut multiple = first_multiple * prime;
            if multiple < prime_square {
                multiple = prime_square;
            }
            if multiple % 2 == 0 {
                multiple += prime;
            }

            while multiple < self.segment_start + (SEGMENT_LENGTH as u32 * 2) {
                self.segment[((multiple - self.segment_start) / 2) as usize] = false;
                multiple += prime * 2;
            }
        }

        Some(())
    }
}

/// Generates the fixed set of primes used to mark composite segment values.
fn base_primes() -> Vec<u32> {
    const LIMIT: usize = (u16::MAX as usize) + 1;
    let mut composite = vec![false; LIMIT + 1];
    let mut result = Vec::new();

    for number in 2..=LIMIT {
        if composite[number] {
            continue;
        }

        result.push(number as u32);
        if number <= LIMIT / number {
            for multiple in (number * number..=LIMIT).step_by(number) {
                composite[multiple] = true;
            }
        }
    }

    result
}

/// Creates an iterator that produces prime numbers in ascending order.
///
/// The first segment is not allocated until the first prime after `2` is
/// requested, so constructing the iterator only builds the small base sieve.
fn stream() -> impl Iterator<Item = u32> {
    PrimeNumbers {
        current: 1,
        base_primes: base_primes(),
        segment_start: 3,
        segment: Vec::new(),
        segment_index: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ntest::timeout;

    fn test_segment(start: u32, numbers: [u32; 10]) {
        let mut prime_iterator = stream();
        for _ in 0..start {
            prime_iterator.next();
        }
        for i in numbers {
            assert_eq!(
                Some(i),
                prime_iterator.next(),
                "\nYour result (left) did not match the expected output (right)"
            );
        }
    }

    #[test]
    #[timeout(2000)]
    fn tests() {
        println!("testing segment from 0");
        test_segment(0, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);

        println!("testing segment from 10");
        test_segment(10, [31, 37, 41, 43, 47, 53, 59, 61, 67, 71]);

        println!("testing segment from 100");
        test_segment(100, [547, 557, 563, 569, 571, 577, 587, 593, 599, 601]);

        println!("testing segment from 1,000");
        test_segment(
            1_000,
            [7927, 7933, 7937, 7949, 7951, 7963, 7993, 8009, 8011, 8017],
        );
    }

    #[test]
    #[timeout(12_000)]
    fn test_50_millionth_prime() {
        let mut prime_iterator = stream();

        for _ in 0..49_999_999 {
            prime_iterator.next();
        }

        assert_eq!(Some(982_451_653), prime_iterator.next());
    }
}
