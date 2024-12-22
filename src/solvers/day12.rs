use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Orientation {
    Top,
    Bottom,
    Left,
    Right,
}

impl Into<(isize, isize)> for Orientation {
    fn into(self) -> (isize, isize) {
        match self {
            Orientation::Top => (-1, 0),
            Orientation::Bottom => (1, 0),
            Orientation::Left => (0, -1),
            Orientation::Right => (0, 1),
        }
    }
}

impl Orientation {
    fn orthogonal(&self) -> Orientation {
        match self {
            Orientation::Top => Orientation::Right,
            Orientation::Bottom => Orientation::Left,
            Orientation::Left => Orientation::Top,
            Orientation::Right => Orientation::Bottom,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Fence {
    position: (isize, isize),
    orientation: Orientation,
}

impl Fence {
    fn is_connected_to(&self, other: &Fence) -> bool {
        if self.orientation != other.orientation {
            return false;
        }
        let (dy, dx) = self.orientation.orthogonal().into();
        let mut adj_pos = (self.position.0 + dy, self.position.1 + dx);

        if other.position == adj_pos {
            return true;
        }

        adj_pos = (self.position.0 - dy, self.position.1 - dx);

        other.position == adj_pos
    }
}

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
            let adjacent_diffs = [
                Orientation::Top,
                Orientation::Bottom,
                Orientation::Left,
                Orientation::Right,
            ];
            let symbol: char = map[y][x];
            let mut region: HashSet<(usize, usize)> = HashSet::new();
            let mut explore_queue: Vec<(usize, usize)> = Vec::from([(y, x)]);
            let mut fences: Vec<Fence> = Vec::new();

            while let Some((y, x)) = explore_queue.pop() {
                if region.contains(&(y, x)) {
                    continue;
                }

                assert!(map[y][x] == symbol);

                region.insert((y, x));
                visited[y][x] = true;
                for (orientation, (dy, dx)) in &adjacent_diffs.map(|o| (o, o.into())) {
                    if let (Some(ny), Some(nx)) =
                        (y.checked_add_signed(*dy), x.checked_add_signed(*dx))
                    {
                        if !region.contains(&(ny, nx)) {
                            if map
                                .get(ny)
                                .is_some_and(|row| row.get(nx).is_some_and(|c| *c == symbol))
                            {
                                explore_queue.push((ny, nx));
                            } else {
                                fences.push(Fence {
                                    position: (y as isize, x as isize),
                                    orientation: *orientation,
                                });
                            }
                        }
                    } else {
                        fences.push(Fence {
                            position: (y as isize, x as isize),
                            orientation: *orientation,
                        });
                    }
                }
            }

            let mut sides: Vec<Vec<&Fence>> = Vec::new();

            for fence in &fences {
                let mut adj_indices: Vec<usize> = Vec::new();
                for (index, side) in sides.iter().enumerate() {
                    for other in side {
                        if other.is_connected_to(fence) {
                            adj_indices.push(index);
                        }
                    }
                }

                match adj_indices.len() {
                    0 => {
                        sides.push(Vec::from([fence]));
                    }
                    1 => {
                        sides[adj_indices[0]].push(fence);
                    }
                    _ => {
                        adj_indices.sort();
                        let mut merged = sides.remove(adj_indices.pop().unwrap());
                        while let Some(index) = adj_indices.pop() {
                            merged.extend(sides.remove(index));
                        }
                        sides.push(merged);
                    }
                }
            }

            // println!("{} {:?} {}", symbol, region, sides.len());
            region.len() * sides.len()
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
