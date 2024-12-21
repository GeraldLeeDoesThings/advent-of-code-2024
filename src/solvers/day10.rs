pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut trailheads: Vec<(usize, usize)> = Vec::new();
        let map: Vec<Vec<u8>> = Vec::from_iter(input.lines().map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        }));
        let mut ways_to_reach: Vec<Vec<Option<usize>>> = Vec::from_iter(map.iter().map(|v| {
            v.iter()
                .map(|h| match *h {
                    9 => Some(1),
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
            cache: &mut Vec<Vec<Option<usize>>>,
        ) -> usize {
            let diffs: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

            if let Some(cached) = cache
                .get(at.0)
                .map(|v| v.get(at.1).map(|inner| *inner))
                .flatten()
                .flatten()
            {
                return cached;
            }

            let current_height = map[at.0][at.1];
            let mut total_ways: usize = 0;
            for (dy, dx) in diffs {
                if let (Some(ny), Some(nx)) =
                    (at.0.checked_add_signed(dx), at.1.checked_add_signed(dy))
                {
                    if let Some(height) = map.get(ny).map(|row| row.get(nx).map(|h| *h)).flatten() {
                        if height == current_height + 1 {
                            total_ways += search((ny, nx), map, cache);
                        }
                    }
                }
            }
            cache[at.0][at.1] = Some(total_ways);

            total_ways
        }

        let acc: usize = trailheads
            .iter()
            .map(|pos| search(*pos, &map, &mut ways_to_reach))
            .sum();
        acc.to_string()
    }
}
