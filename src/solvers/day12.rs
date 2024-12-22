use std::collections::HashSet;

pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let map: Vec<Vec<char>> = Vec::from_iter(input.lines().map(|line| line.chars().collect()));
        let mut visited: Vec<Vec<bool>> = Vec::from_iter(
            input
                .lines()
                .map(|line| line.chars().map(|_| false).collect()),
        );

        fn explore_region(
            y: usize,
            x: usize,
            map: &Vec<Vec<char>>,
            visited: &mut Vec<Vec<bool>>,
        ) -> usize {
            let adjacent_diffs: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
            let symbol: char = map[y][x];
            let mut region: HashSet<(usize, usize)> = HashSet::new();
            let mut explore_queue: Vec<(usize, usize)> = Vec::from([(y, x)]);
            let mut perimeter = 0;

            while let Some((y, x)) = explore_queue.pop() {
                if region.contains(&(y, x)) {
                    continue;
                }

                if map
                    .get(y)
                    .is_some_and(|row| row.get(x).is_some_and(|c| *c == symbol))
                {
                    region.insert((y, x));
                    visited[y][x] = true;
                    for (dy, dx) in adjacent_diffs {
                        if let (Some(ny), Some(nx)) =
                            (y.checked_add_signed(dy), x.checked_add_signed(dx))
                        {
                            if !region.contains(&(ny, nx)) {
                                explore_queue.push((ny, nx));
                            }
                        } else {
                            perimeter += 1;
                        }
                    }
                } else {
                    perimeter += 1;
                }
            }

            // println!("{} {:?} {}", symbol, region, perimeter);
            region.len() * perimeter
        }

        let mut acc = 0;

        for (y, row) in map.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if !visited[y][x] {
                    acc += explore_region(y, x, &map, &mut visited);
                }
            }
        }

        acc.to_string()
    }
}
