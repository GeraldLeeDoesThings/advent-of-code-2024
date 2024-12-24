use std::usize;

pub struct Solver {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Into<(isize, isize)> for Direction {
    fn into(self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

impl Into<usize> for &Direction {
    fn into(self) -> usize {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value % 4 {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    fn rotate_cw(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn rotate_ccw(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn index(&self) -> usize {
        self.into()
    }
}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        // TODO: Implement
        let map: Vec<Vec<char>> = Vec::from_iter(input.lines().map(|line| line.chars().collect()));
        let mut best: Vec<Vec<[Option<usize>; 4]>> = Vec::from_iter(
            input
                .lines()
                .map(|line| line.chars().map(|_| [None; 4]).collect()),
        );
        let mut maybe_start_pos = None;
        let mut maybe_end_pos = None;
        for (y, row) in map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'S' {
                    maybe_start_pos = Some((x, y));
                } else if *c == 'E' {
                    maybe_end_pos = Some((x, y));
                }
            }
        }
        let start = maybe_start_pos.unwrap();
        let end = maybe_end_pos.unwrap();
        let mut explore_queue: Vec<(usize, usize, usize, Direction)> =
            Vec::from([(start.0, start.1, 0, Direction::East)]);

        while let Some((x, y, cost, direction)) = explore_queue.pop() {
            if map[y][x] == '#' {
                continue;
            }

            let maybe_current_best = best[y][x][direction.index()];
            if let Some(current_best) = maybe_current_best {
                if cost < current_best {
                    best[y][x][direction.index()] = Some(cost);
                } else {
                    continue;
                }
            } else {
                best[y][x][direction.index()] = Some(cost);
            }

            let diffs: (isize, isize) = direction.into();
            explore_queue.push((
                x.checked_add_signed(diffs.0).unwrap(),
                y.checked_add_signed(diffs.1).unwrap(),
                cost + 1,
                direction,
            ));
            explore_queue.push((x, y, cost + 1000, direction.rotate_cw()));
            explore_queue.push((x, y, cost + 1000, direction.rotate_ccw()));
        }

        best[end.1][end.0]
            .iter()
            .fold(usize::MAX, |acc, best| acc.min(best.unwrap()))
            .to_string()
    }
}
