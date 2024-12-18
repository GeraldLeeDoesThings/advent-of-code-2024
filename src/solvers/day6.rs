use std::collections::HashSet;

pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        // TODO: Implement
        let mut map: Vec<Vec<char>> = Vec::new();
        let mut blocked: HashSet<(usize, usize)> = HashSet::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut pos: (usize, usize) = (0, 0);
        let directions: [(isize, isize); 4] = [
            (0, -1),
            (1, 0),
            (0, 1),
            (-1, 0),
        ];
        let mut dir_index: usize = 0;
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Some(col) = map.get_mut(x) {
                    col.insert(y, c);
                }
                else {
                    map.insert(x, Vec::from([c]));
                }
                match c {
                    '#' => {
                        blocked.insert((x, y));
                    },
                    '^' => {
                        pos = (x, y);
                        visited.insert((x, y));
                    },
                    _ => (),
                }
            }
        }
        let max_x = map[0].len();
        let max_y = map.len();
        loop {
            visited.insert(pos.clone());
            let maybe_nx = pos.0.checked_add_signed(directions[dir_index].0);
            let maybe_ny = pos.1.checked_add_signed(directions[dir_index].1);
            if maybe_nx.is_some() && maybe_ny.is_some() {
                let npos = (maybe_nx.unwrap(), maybe_ny.unwrap());
                if npos.0 == max_x || npos.1 == max_y {
                    break;
                }
                if blocked.contains(&npos) {
                    dir_index = (dir_index + 1) % directions.len();
                }
                else {
                    pos = npos;
                }
            }
            else {
                break;
            }
        }
        visited.len().to_string()
    }
}
