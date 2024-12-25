use std::collections::HashSet;

pub struct Solver {}

const BOARD_SIZE: usize = 71;

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        // TODO: Implement
        let mut blocked: [[bool; BOARD_SIZE]; BOARD_SIZE] = [[false; BOARD_SIZE]; BOARD_SIZE];
        for line in input.lines().take(1024) {
            let mut parts = line.split(",");
            let x: usize = parts.next().unwrap().parse().unwrap();
            let y: usize = parts.next().unwrap().parse().unwrap();
            blocked[y][x] = true;
        }

        let mut explore_queue: Vec<(usize, usize)> = Vec::from([(0, 0)]);
        let mut steps: usize = 0;
        loop {
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
                break;
            }
            explore_queue = new_explore_queue.iter().map(|p| *p).collect();
            assert!(explore_queue.len() > 0);
        }

        steps.to_string()
    }
}
