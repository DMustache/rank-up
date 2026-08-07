struct PrimeNumbers {
    current: u32,
    primes: Vec<u32>,
}

impl Iterator for PrimeNumbers {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.current = match self.current {
                1 => 2,
                2 => 3,
                current => current.checked_add(2)?,
            };

            let limit = (self.current as f64).sqrt() as u32;
            let mut is_prime = true;

            for &prime in &self.primes {
                if prime > limit {
                    break;
                }
                if self.current % prime == 0 {
                    is_prime = false;
                    break;
                }
            }

            if is_prime {
                self.primes.push(self.current);
                return Some(self.current);
            }
        }
    }
}

fn stream() -> impl Iterator<Item = u32> {
    PrimeNumbers {
        current: 1,
        primes: Vec::new(),
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
    #[ignore = "heavy"]
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
}
