pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let map: Vec<Vec<char>> = Vec::from_iter(input.lines().map(|line| line.chars().collect()));
        let mut maybe_exit: Option<(usize, usize)> = None;
        let mut maybe_start: Option<(usize, usize)> = None;
        let mut bests: Vec<Vec<Option<usize>>> = Vec::from_iter(input.lines().map(|line| {
            line.chars()
                .map(|c| if c == 'E' { Some(0) } else { None })
                .collect()
        }));

        for (y, row) in input.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                match c {
                    'E' => maybe_exit = Some((x, y)),
                    'S' => maybe_start = Some((x, y)),
                    _ => (),
                }
            }
        }

        let exit = maybe_exit.unwrap();
        let _start = maybe_start.unwrap();
        let mut explore_queue: Vec<(usize, usize, usize)> = Vec::from([(exit.0, exit.1, 0)]);
        while let Some((x, y, distance)) = explore_queue.pop() {
            let diffs: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
            for (dx, dy) in diffs {
                if let (Some(nx), Some(ny)) = (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                    if map
                        .get(ny)
                        .is_some_and(|row| row.get(nx).is_some_and(|c| *c != '#'))
                        && bests[ny][nx].is_none_or(|best| best > distance + 1)
                    {
                        bests[ny][nx] = Some(distance + 1);
                        explore_queue.push((nx, ny, distance + 1));
                    }
                }
            }
        }

        let mut decent_cheats: usize = 0;

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if bests[y][x].is_none() {
                    continue;
                }

                for dx in -20_isize..=20 {
                    for dy in -(20 - dx.abs())..=(20 - dx.abs()) {
                        if dx.abs() + dy.abs() > 20 {
                            continue;
                        }

                        if let (Some(nx), Some(ny)) =
                            (x.checked_add_signed(dx), y.checked_add_signed(dy))
                        {
                            if bests
                                .get(ny)
                                .is_some_and(|row| row.get(nx).is_some_and(|val| val.is_some()))
                            {
                                if let Some(savings) = bests[y][x].unwrap().checked_sub(
                                    bests[ny][nx].unwrap() + dx.abs() as usize + dy.abs() as usize,
                                ) {
                                    if savings >= 100 {
                                        decent_cheats += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        decent_cheats.to_string()
    }
}
