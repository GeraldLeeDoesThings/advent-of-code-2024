pub struct Solver {}

struct Robot {
    x: usize,
    y: usize,
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
            'O' => {
                let mut tx = nx + direction.0;
                let mut ty = ny + direction.1;
                while map[ty as usize][tx as usize] == 'O' {
                    tx = tx + direction.0;
                    ty = ty + direction.1;
                }
                if map[ty as usize][tx as usize] == '#' {
                    return;
                }
                map[ty as usize][tx as usize] = 'O';
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
                map.push(line.chars().collect());
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
                if *c == 'O' {
                    score += 100 * y + x;
                }
            }
        }

        score.to_string()
    }
}
