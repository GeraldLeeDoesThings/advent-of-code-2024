pub struct Solver {}

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

struct Robot {
    x: isize,
    y: isize,
    vel_x: isize,
    vel_y: isize,
    location_cache: Vec<(isize, isize)>,
    period: Option<usize>,
}

impl Robot {
    fn new(x: isize, y: isize, vel_x: isize, vel_y: isize) -> Robot {
        Robot {
            x: x,
            y: y,
            vel_x: vel_x,
            vel_y: vel_y,
            location_cache: Vec::from([(x, y)]),
            period: None,
        }
    }

    fn solve(&mut self, step: usize) -> (isize, isize) {
        // Should have a &self version that does not update the cache
        if let Some(period) = self.period {
            self.location_cache[step % period]
        } else if let Some(cached) = self.location_cache.get(step) {
            *cached
        } else {
            while self.location_cache.len() <= step {
                let mut next = *self.location_cache.last().unwrap();
                next.0 += self.vel_x;
                next.1 += self.vel_y;
                if next.0 < 0 {
                    next.0 += WIDTH;
                } else if next.0 >= WIDTH {
                    next.0 -= WIDTH;
                }

                if next.1 < 0 {
                    next.1 += HEIGHT;
                } else if next.1 >= HEIGHT {
                    next.1 -= HEIGHT;
                }

                if next == (self.x, self.y) {
                    self.period = Some(self.location_cache.len());
                    break;
                } else {
                    self.location_cache.push(next);
                }
            }
            self.solve(step)
        }
    }
}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut robots: Vec<Robot> = Vec::new();

        for line in input.lines() {
            let mut line_parts = line.split(" ");
            let mut pos_parts = line_parts.next().unwrap()[2..].split(",");
            let mut vel_parts = line_parts.next().unwrap()[2..].split(",");

            let pos: (isize, isize) = (
                pos_parts.next().unwrap().parse().unwrap(),
                pos_parts.next().unwrap().parse().unwrap(),
            );

            let vel: (isize, isize) = (
                vel_parts.next().unwrap().parse().unwrap(),
                vel_parts.next().unwrap().parse().unwrap(),
            );

            robots.push(Robot::new(pos.0, pos.1, vel.0, vel.1));
        }

        let mut quad_counts: [usize; 4] = [0; 4];

        for (x, y) in robots.iter_mut().map(|robot| robot.solve(100)) {
            let mut x_offset = None;
            let mut y_offset = None;
            if x < WIDTH / 2 {
                x_offset = Some(0);
            } else if x > WIDTH / 2 {
                x_offset = Some(2);
            }

            if y < HEIGHT / 2 {
                y_offset = Some(0);
            } else if y > HEIGHT / 2 {
                y_offset = Some(1);
            }

            if let (Some(x_offset), Some(y_offset)) = (x_offset, y_offset) {
                quad_counts[x_offset + y_offset] += 1;
            }
        }

        quad_counts
            .iter()
            .fold(1, |acc, count| acc * count)
            .to_string()
    }
}
