use std::collections::HashMap;

pub struct Solver {}

struct Robot {
    x: usize,
    y: usize,
}

fn push_box(at: (isize, isize), direction: (isize, isize), map: &mut Vec<Vec<char>>) {
    let mut to_push: Vec<(isize, isize)> = Vec::new();
    // println!("Pushing at: {:?} {}", at, map[at.1 as usize][at.0 as usize]);

    if direction.1 == 0 {
        match direction.0 {
            -1 => {
                to_push.push((at.0 - 1, at.1));
            }
            1 => {
                to_push.push((at.0 + 2, at.1));
            }
            _ => unreachable!(),
        }
    } else {
        to_push.push((at.0, at.1 + direction.1));
        to_push.push((at.0 + 1, at.1 + direction.1));
    }

    while let Some(push_at) = to_push.pop() {
        // println!("Push check {} {:?}", map[push_at.1 as usize][push_at.0 as usize], push_at);
        match map[push_at.1 as usize][push_at.0 as usize] {
            ']' => {
                to_push.push((push_at.0 - 1, push_at.1));
            }
            '[' => {
                push_box(push_at, direction, map);
            }
            '.' => (),
            c => unreachable!("Found {c} {:?}", push_at), // Path should be clear
        }
    }
    map[at.1 as usize][at.0 as usize] = '.';
    map[at.1 as usize][(at.0 + 1) as usize] = '.';
    assert!(map[(at.1 + direction.1) as usize][(at.0 + direction.0) as usize] == '.');
    assert!(map[(at.1 + direction.1) as usize][(at.0 + direction.0 + 1) as usize] == '.');
    map[(at.1 + direction.1) as usize][(at.0 + direction.0) as usize] = '[';
    map[(at.1 + direction.1) as usize][(at.0 + direction.0 + 1) as usize] = ']';
}

fn check_box(
    at: (isize, isize),
    direction: (isize, isize),
    map: &Vec<Vec<char>>,
    ok_cache: &mut HashMap<(isize, isize), bool>,
) -> bool {
    if let Some(cached) = ok_cache.get(&at) {
        return *cached;
    }

    // println!("Checking {:?}", at);
    // This seems like a not great way to do this
    // I should probably have written this function
    // like push_box

    if direction.1 == 0 {
        match direction.0 {
            -1 => match map[at.1 as usize][(at.0 - 1) as usize] {
                '.' => {
                    ok_cache.insert(at, true);
                    return true;
                }
                '#' => {
                    ok_cache.insert(at, false);
                    return false;
                }
                ']' => {
                    let result = check_box((at.0 - 2, at.1), direction, map, ok_cache);
                    ok_cache.insert(at, result);
                    return result;
                }
                _ => unreachable!(),
            },
            1 => match map[at.1 as usize][(at.0 + 2) as usize] {
                '.' => {
                    ok_cache.insert(at, true);
                    return true;
                }
                '#' => {
                    ok_cache.insert(at, false);
                    return false;
                }
                '[' => {
                    let result = check_box((at.0 + 2, at.1), direction, map, ok_cache);
                    ok_cache.insert(at, result);
                    return result;
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    } else {
        let l_val = map[(at.1 + direction.1) as usize][at.0 as usize];
        let r_val = map[(at.1 + direction.1) as usize][(at.0 + 1) as usize];
        // println!("L: {l_val} {:?}, R: {r_val} {:?}", (at.0, at.1 + direction.1), (at.0 + 1, at.1 + direction.1));

        match l_val {
            '[' => {
                // Exactlty one box on top
                let result = check_box((at.0, at.1 + direction.1), direction, map, ok_cache);
                ok_cache.insert(at, result);
                return result;
            }
            ']' => {
                // Check this box, and if it can move, check the right side
                let result = check_box((at.0 - 1, at.1 + direction.1), direction, map, ok_cache);
                if !result {
                    ok_cache.insert(at, result);
                    return result;
                }
            }
            '#' => {
                ok_cache.insert(at, false);
                return false;
            }
            '.' => (), // Check right side
            _ => unreachable!(),
        }

        match r_val {
            '[' => {
                let result = check_box((at.0 + 1, at.1 + direction.1), direction, map, ok_cache);
                ok_cache.insert(at, result);
                return result;
            }
            '#' => {
                ok_cache.insert(at, false);
                return false;
            }
            '.' => {
                ok_cache.insert(at, true);
                return true;
            }
            _ => unreachable!("Found {l_val} {r_val}"),
        }
    }
}

impl Robot {
    fn exec_move(&mut self, direction: char, map: &mut Vec<Vec<char>>) {
        let direction: (isize, isize) = match direction {
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            '^' => (0, -1),
            _ => unreachable!(),
        };
        let cx = self.x as isize;
        let cy = self.y as isize;
        let nx = cx + direction.0;
        let ny = cy + direction.1;
        match map[ny as usize][nx as usize] {
            '.' => (),
            '#' => return,
            '[' => {
                if check_box((nx, ny), direction, map, &mut HashMap::new()) {
                    push_box((nx, ny), direction, map);
                } else {
                    return;
                }
            }
            ']' => {
                if check_box((nx - 1, ny), direction, map, &mut HashMap::new()) {
                    push_box((nx - 1, ny), direction, map);
                } else {
                    return;
                }
            }
            _ => unreachable!(),
        }
        map[self.y][self.x] = '.';
        self.x = nx as usize;
        self.y = ny as usize;
        map[ny as usize][nx as usize] = '@';
    }
}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut map: Vec<Vec<char>> = Vec::new();
        let mut movement: Vec<char> = Vec::new();
        let mut parsing_map = true;
        for line in input.lines() {
            if line.is_empty() {
                parsing_map = false;
                continue;
            }
            if parsing_map {
                map.push(
                    line.chars()
                        .map(|c| match c {
                            '#' => ['#', '#'],
                            'O' => ['[', ']'],
                            '.' => ['.', '.'],
                            '@' => ['@', '.'],
                            _ => unreachable!(),
                        })
                        .flatten()
                        .collect(),
                );
            } else {
                movement.extend(line.chars());
            }
        }

        let mut robot: Robot = Robot { x: 0, y: 0 };

        for (y, row) in map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '@' {
                    robot = Robot { x: x, y: y };
                }
            }
        }

        for dir in movement {
            robot.exec_move(dir, &mut map);
        }

        let mut score: usize = 0;
        for (y, row) in map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '[' {
                    score += 100 * y + x;
                }
            }
        }

        score.to_string()
    }
}
