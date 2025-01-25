use std::iter::zip;

pub struct Solver {}

#[derive(Debug)]
struct Profile {
    heights: [u8; 5],
}

impl Profile {
    fn new() -> Profile {
        Profile { heights: [0u8; 5] }
    }

    fn parse_line(&mut self, line: &str) -> bool {
        if line.len() != 5 {
            return false;
        }
        for (i, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                '#' => self.heights[i] += 1,
                _ => return false,
            }
        }
        true
    }

    fn fits_with(&self, other: &Profile) -> bool {
        !zip(self.heights, other.heights).any(|(x, y)| x + y > 5)
    }
}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut locks: Vec<Profile> = Vec::new();
        let mut keys: Vec<Profile> = Vec::new();
        let mut defining = true;
        let mut is_lock = false;
        let mut working = Profile::new();
        let mut working_lines = 0;

        for line in input.lines() {
            if line.is_empty() {
                assert_eq!(working_lines, 5);
                defining = true;
                if is_lock {
                    locks.push(working);
                } else {
                    keys.push(working);
                }
                working = Profile::new();
                working_lines = 0;
                continue;
            }
            if defining {
                if line.chars().all(|c| c == '#') {
                    is_lock = true;
                } else {
                    assert!(line.chars().all(|c| c == '.'));
                    is_lock = false;
                }
                defining = false;
                continue;
            }
            if working_lines == 5 {
                continue;
            }
            working.parse_line(line);
            working_lines += 1;
        }

        if working_lines == 5 {
            if is_lock {
                locks.push(working);
            } else {
                keys.push(working);
            }
        }

        let num_fits: usize = keys
            .iter()
            .flat_map(|key| {
                locks
                    .iter()
                    .map(|lock| if lock.fits_with(key) { 1 } else { 0 })
            })
            .sum();
        num_fits.to_string()
    }
}
