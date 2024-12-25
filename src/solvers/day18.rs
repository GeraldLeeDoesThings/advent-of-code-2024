use std::collections::HashSet;

pub struct Solver {}

const BOARD_SIZE: usize = 71;

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        // TODO: Implement
        let mut blocked: [[bool; BOARD_SIZE]; BOARD_SIZE] = [[false; BOARD_SIZE]; BOARD_SIZE];
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let mut lo: usize = 0;
        let mut hi: usize = lines.len();
        let mut cursor: usize = 0;

        fn is_connected(mut blocked: [[bool; BOARD_SIZE]; BOARD_SIZE]) -> Option<usize> {
            let mut explore_queue: Vec<(usize, usize)> = Vec::from([(0, 0)]);
            let mut steps: usize = 0;
            while !explore_queue.is_empty() {
                let mut new_explore_queue: HashSet<(usize, usize)> = HashSet::new();
                for (x, y) in &explore_queue {
                    blocked[*y][*x] = true;
                    let diffs: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
                    for (dx, dy) in diffs {
                        if let (Some(nx), Some(ny)) =
                            (x.checked_add_signed(dx), y.checked_add_signed(dy))
                        {
                            if blocked
                                .get(ny)
                                .is_some_and(|row| row.get(nx).is_some_and(|blocked| !blocked))
                            {
                                new_explore_queue.insert((nx, ny));
                            }
                        }
                    }
                }
                steps += 1;
                if new_explore_queue.contains(&(BOARD_SIZE - 1, BOARD_SIZE - 1)) {
                    return Some(steps);
                }
                explore_queue = new_explore_queue.iter().map(|p| *p).collect();
            }
            None
        }

        while lo < hi {
            let target = (lo + hi) / 2;
            while cursor < target {
                cursor += 1;
                let mut parts = lines[cursor].split(",");
                let x: usize = parts.next().unwrap().parse().unwrap();
                let y: usize = parts.next().unwrap().parse().unwrap();
                blocked[y][x] = true;
            }

            while cursor > target {
                let mut parts = lines[cursor].split(",");
                let x: usize = parts.next().unwrap().parse().unwrap();
                let y: usize = parts.next().unwrap().parse().unwrap();
                blocked[y][x] = false;
                cursor -= 1;
            }

            match is_connected(blocked.clone()) {
                Some(_) => lo = target + 1,
                None => hi = target,
            }
        }

        lines[lo].clone()
    }
}
