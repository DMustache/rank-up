struct ProductOfNumbers {
    products: Vec<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        Self { products: vec![1] }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.products.clear();
            self.products.push(1);
        } else {
            self.products.push(self.products.last().unwrap() * num);
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        if k >= self.products.len() {
            0
        } else {
            let end = self.products.len() - 1;
            self.products[end] / self.products[end - k]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ProductOfNumbers;

    #[test]
    fn queries_across_zero_and_after_more_additions() {
        let mut stream = ProductOfNumbers::new();
        for num in [3, 0, 2, 5, 4] {
            stream.add(num);
        }

        assert_eq!(stream.get_product(2), 20);
        assert_eq!(stream.get_product(3), 40);
        assert_eq!(stream.get_product(4), 0);

        stream.add(8);
        assert_eq!(stream.get_product(2), 32);
        assert_eq!(stream.get_product(4), 320);
    }
}
