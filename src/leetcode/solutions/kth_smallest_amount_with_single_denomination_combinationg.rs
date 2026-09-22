struct Solution;

impl Solution {
    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        let k = k as u64;
        let hi = k * *coins.iter().min().unwrap() as u64;

        let coins = coins
            .iter()
            .map(|&c| c as u64)
            .filter(|&c| !coins.iter().any(|&d| d as u64 != c && c % d as u64 == 0))
            .collect::<Vec<_>>();

        // (lcm, sign)
        let terms = coins.iter().fold(vec![(1u64, -1i64)], |terms, &c| {
            let added = terms
                .iter()
                .map(|&(l, s)| (lcm(l, c), -s))
                .filter(|&(l, _)| l <= hi);
            coalesce(terms.iter().copied().chain(added).collect())
        });
        let terms = coalesce(terms.into_iter().chain([(1, 1)]).collect());

        // how many numbers <= x
        let count = |x: u64| terms.iter().map(|&(l, s)| s * (x / l) as i64).sum::<i64>();

        let k = k as i64;
        std::iter::successors(Some((1u64, hi)), |&(lo, hi)| {
            (lo < hi).then(|| {
                let mid = lo + (hi - lo) / 2;
                if count(mid) < k {
                    (mid + 1, hi)
                } else {
                    (lo, mid)
                }
            })
        })
        .last()
        .unwrap()
        .0 as i64
    }
}

fn coalesce(mut rows: Vec<(u64, i64)>) -> Vec<(u64, i64)> {
    rows.sort_unstable_by_key(|&(l, _)| l);
    rows.chunk_by(|a, b| a.0 == b.0)
        .map(|g| (g[0].0, g.iter().map(|&(_, s)| s).sum::<i64>()))
        .filter(|&(_, s)| s != 0)
        .collect()
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn gcd(mut u: u64, mut v: u64) -> u64 {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }
    let k = (u | v).trailing_zeros();
    u >>= u.trailing_zeros();
    let mut k_v = v.trailing_zeros();
    loop {
        v >>= k_v;
        let (diff, neg_diff) = u.overflowing_sub(v);
        if diff == 0 {
            return u << k;
        }
        k_v = diff.trailing_zeros();
        u = u.min(v);
        v = if neg_diff { diff.wrapping_neg() } else { diff };
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode::solutions::kth_smallest_amount_with_single_denomination_combinationg::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::find_kth_smallest(vec![3, 6, 9], 3), 9);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::find_kth_smallest(vec![5, 2], 7), 12);
    }
}
