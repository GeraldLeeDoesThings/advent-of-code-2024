use std::collections::HashSet;

pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut trailheads: Vec<(usize, usize)> = Vec::new();
        let map: Vec<Vec<u8>> = Vec::from_iter(input.lines().map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        }));
        let mut num_trailheads: usize = 0;
        let mut ways_to_reach: Vec<Vec<Option<HashSet<usize>>>> =
            Vec::from_iter(map.iter().map(|v| {
                v.iter()
                    .map(|h| match *h {
                        9 => {
                            let single = HashSet::from([num_trailheads]);
                            num_trailheads += 1;
                            Some(single)
                        }
                        _ => None,
                    })
                    .collect()
            }));
        for (row_index, row) in map.iter().enumerate() {
            for (col, h) in row.iter().enumerate() {
                if *h == 0 {
                    trailheads.push((row_index, col));
                }
            }
        }

        fn search(
            at: (usize, usize),
            map: &Vec<Vec<u8>>,
            cache: &mut Vec<Vec<Option<HashSet<usize>>>>,
        ) -> HashSet<usize> {
            let diffs: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

            if let Some(cached) = cache
                .get(at.0)
                .map(|v| v.get(at.1))
                .flatten()
                .map(|inner| inner.clone())
                .flatten()
            {
                return cached.clone();
            }

            let current_height = map[at.0][at.1];
            let mut total_ways: HashSet<usize> = HashSet::new();
            for (dy, dx) in diffs {
                if let (Some(ny), Some(nx)) =
                    (at.0.checked_add_signed(dx), at.1.checked_add_signed(dy))
                {
                    if let Some(height) = map.get(ny).map(|row| row.get(nx).map(|h| *h)).flatten() {
                        if height == current_height + 1 {
                            total_ways.extend(search((ny, nx), map, cache));
                        }
                    }
                }
            }
            cache[at.0][at.1] = Some(total_ways.clone());

            total_ways
        }

        let acc: usize = trailheads
            .iter()
            .map(|pos| search(*pos, &map, &mut ways_to_reach).len())
            .sum();
        acc.to_string()
    }
}
