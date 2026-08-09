use std::{collections::HashMap, mem::swap};

fn add_mod(left: u64, right: u64, modulo: u64) -> u64 {
    ((left as u128 + right as u128) % modulo as u128) as u64
}

fn mul_mod(left: u64, right: u64, modulo: u64) -> u64 {
    ((left as u128 * right as u128) % modulo as u128) as u64
}

fn shifted_xor(
    m: u64,
    n: u64,
    shift: u64,
    t: u64,
    cache: &mut HashMap<(u64, u64, u64), u64>,
) -> u64 {
    add_mod(
        mul_mod(mul_mod(m, n, t), shift, t),
        sum_xor(m, n, t, cache),
        t,
    )
}

fn sum_xor(mut m: u64, mut n: u64, t: u64, cache: &mut HashMap<(u64, u64, u64), u64>) -> u64 {
    if m == 0 || n == 0 {
        return 0;
    }
    if m > n {
        swap(&mut m, &mut n);
    }
    if n == 1 {
        return 0;
    }
    let key = (m, n, u64::MAX);
    if let Some(value) = cache.get(&key) {
        return *value;
    }

    let bit = 1 << (63 - (n - 1).leading_zeros());
    if m > bit {
        let low = sum_xor(bit, bit, t, cache);
        let low_high = shifted_xor(bit, n - bit, bit, t, cache);
        let high_low = shifted_xor(m - bit, bit, bit, t, cache);
        let high_high = sum_xor(m - bit, n - bit, t, cache);
        let result = add_mod(
            add_mod(low, low_high, t),
            add_mod(high_low, high_high, t),
            t,
        );
        cache.insert(key, result);
        return result;
    }

    let result = add_mod(
        sum_xor(m, bit, t, cache),
        shifted_xor(m, n - bit, bit, t, cache),
        t,
    );
    cache.insert(key, result);
    result
}

fn triangular_mod(n: u64, t: u64) -> u64 {
    let n = n as u128;
    ((n * (n + 1) / 2) % t as u128) as u64
}

fn shifted_block(
    m: u64,
    n: u64,
    shift: u64,
    l: u64,
    t: u64,
    cache: &mut HashMap<(u64, u64, u64), u64>,
) -> u64 {
    if m == 0 || n == 0 {
        return 0;
    }
    if shift >= l {
        return add_mod(
            mul_mod(mul_mod(m, n, t), shift - l, t),
            sum_xor(m, n, t, cache),
            t,
        );
    }

    elder_age_inner(m, n, l - shift, t, cache)
}

fn elder_age(m: u64, n: u64, l: u64, t: u64) -> u64 {
    let mut cache = HashMap::new();
    elder_age_inner(m, n, l, t, &mut cache)
}

fn elder_age_inner(
    mut m: u64,
    mut n: u64,
    l: u64,
    t: u64,
    cache: &mut HashMap<(u64, u64, u64), u64>,
) -> u64 {
    if m == 0 || n == 0 {
        return 0;
    }

    if m > n {
        swap(&mut m, &mut n);
    }

    if n == 1 {
        return 0;
    }
    if n.is_power_of_two() {
        if l >= n - 1 {
            return 0;
        }
        let q = n - 1 - l;

        return mul_mod(m, triangular_mod(q, t), t);
    }

    let key = (m, n, l);
    if let Some(value) = cache.get(&key) {
        return *value;
    }

    let bit = 1u64 << (63 - (n - 1).leading_zeros());

    if m <= bit {
        let result = add_mod(
            elder_age_inner(m, bit, l, t, cache),
            shifted_block(m, n - bit, bit, l, t, cache),
            t,
        );
        cache.insert(key, result);
        return result;
    }

    let low = elder_age_inner(bit, bit, l, t, cache);
    let low_high = shifted_block(bit, n - bit, bit, l, t, cache);
    let high_low = shifted_block(m - bit, bit, bit, l, t, cache);
    let high_high = elder_age_inner(m - bit, n - bit, l, t, cache);

    let result = add_mod(
        add_mod(low, low_high, t),
        add_mod(high_low, high_high, t),
        t,
    );
    cache.insert(key, result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(elder_age(8, 5, 1, 100), 5);
        assert_eq!(elder_age(8, 8, 0, 100007), 224);
        assert_eq!(elder_age(25, 31, 0, 100007), 11925);
        assert_eq!(elder_age(5, 45, 3, 1000007), 4323);
        assert_eq!(elder_age(31, 39, 7, 2345), 1586);
        assert_eq!(elder_age(545, 435, 342, 1000007), 808451);
    }

    #[test]
    #[ignore = "heavy"]
    fn huge_test() {
        // You need to run this test very quickly before attempting the actual tests :)
        assert_eq!(
            elder_age(28827050410, 35165045587, 7109602, 13719506),
            5456283
        );
    }
}
