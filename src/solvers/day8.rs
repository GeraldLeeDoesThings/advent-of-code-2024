use std::collections::{HashMap, HashSet};

pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
        let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        let max_y = input.lines().count() as isize;
        let max_x = input.lines().nth(0).unwrap().chars().count() as isize;

        fn in_bounds(point: (isize, isize), max_x: isize, max_y: isize) -> bool {
            point.0 >= 0 && point.0 < max_x && point.1 >= 0 && point.1 < max_y
        }

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    if let Some(cont) = antennas.get_mut(&c) {
                        cont.push((x, y));
                    } else {
                        let v = Vec::from([(x, y)]);
                        assert!(antennas.insert(c, v).is_none());
                    }
                }
            }
        }
        for ants in antennas.values() {
            for (i, loc) in ants
                .iter()
                .map(|(x, y)| (*x as isize, *y as isize))
                .enumerate()
            {
                if i == ants.len() {
                    break;
                }

                for other in ants[i + 1..]
                    .iter()
                    .map(|(x, y)| (*x as isize, *y as isize))
                {
                    let diff = (loc.0 - other.0, loc.1 - other.1);
                    let mut anti_a = loc.clone();
                    while in_bounds(anti_a, max_x, max_y) {
                        antinodes.insert(anti_a);
                        anti_a = (anti_a.0 + diff.0, anti_a.1 + diff.1);
                    }

                    let mut anti_b = other.clone();
                    while in_bounds(anti_b, max_x, max_y) {
                        antinodes.insert(anti_b);
                        anti_b = (anti_b.0 - diff.0, anti_b.1 - diff.1);
                    }
                    // Second iteration of each of the above loops solves part 1
                }
            }
        }
        antinodes.len().to_string()
    }
}
