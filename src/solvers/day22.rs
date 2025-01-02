use std::{collections::HashSet, isize};

pub struct Solver {}

fn next_secret(secret: usize) -> usize {
    let mut new_secret = secret ^ (secret << 6) % 16777216;
    new_secret = new_secret ^ (new_secret >> 5) % 16777216;
    new_secret ^ (new_secret << 11) % 16777216
}

fn last_digit(secret: usize) -> isize {
    (secret - (secret / 10) * 10) as isize
}

struct Secret {
    value: usize,
}

impl Iterator for Secret {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.value;
        self.value = next_secret(self.value);
        Some(result)
    }
}

struct SequenceCounter {
    max: usize,
    counts: [[[[usize; 19]; 19]; 19]; 19],
}

impl SequenceCounter {
    fn new() -> SequenceCounter {
        Self {
            max: 0,
            counts: [[[[0; 19]; 19]; 19]; 19],
        }
    }

    fn observe(&mut self, v1: isize, v2: isize, v3: isize, v4: isize, price: usize) {
        let i: [usize; 4] = [v1, v2, v3, v4].map(|v| (v + 9) as usize);
        self.counts[i[0]][i[1]][i[2]][i[3]] += price;
        self.max = self.counts[i[0]][i[1]][i[2]][i[3]].max(self.max);
    }
}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut counter = SequenceCounter::new();
        let prices: Vec<Vec<u8>> = input
            .lines()
            .map(|line| {
                let secret: Secret = Secret {
                    value: line.parse().unwrap(),
                };
                secret.take(2001).map(|v| last_digit(v) as u8).collect()
            })
            .collect();

        let price_diffs: Vec<Vec<i8>> = prices
            .iter()
            .map(|prices| {
                prices
                    .windows(2)
                    .map(|pair| pair[1] as i8 - pair[0] as i8)
                    .collect()
            })
            .collect();

        for (single_price, diffs) in prices.iter().zip(price_diffs) {
            let mut seen = HashSet::new();
            for (price, diff) in single_price.iter().skip(4).zip(diffs.windows(4)) {
                assert!(diff.len() == 4);
                let diffs = (diff[0], diff[1], diff[2], diff[3]);
                if seen.contains(&diffs) {
                    continue;
                }
                seen.insert(diffs);
                counter.observe(
                    diff[0].into(),
                    diff[1].into(),
                    diff[2].into(),
                    diff[3].into(),
                    *price as usize,
                );
            }
        }

        counter.max.to_string()
    }
}
