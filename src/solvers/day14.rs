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

        let mut draw: [[bool; WIDTH as usize]; HEIGHT as usize];

        let mut best: Option<(usize, usize)> = None;

        for step in 0..10403 {
            // One full period

            draw = [[false; WIDTH as usize]; HEIGHT as usize];

            for (x, y) in robots.iter_mut().map(|robot| robot.solve(step)) {
                draw[y as usize][x as usize] = true;
            }

            let mut score: usize = 0;

            for x in 1..WIDTH - 1 {
                for y in 1..HEIGHT - 1 {
                    if !draw[y as usize][x as usize] {
                        continue;
                    }
                    let diffs: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
                    let mut inner_score = 0;
                    for (dx, dy) in diffs {
                        if draw[(y + dy) as usize][(x + dx) as usize] {
                            inner_score += 1;
                        }
                    }

                    score += inner_score * inner_score;
                }
            }

            if let Some((_, best_score)) = best {
                if score > best_score {
                    best = Some((step, score));
                }
            } else {
                best = Some((step, score));
            }
        }

        draw = [[false; WIDTH as usize]; HEIGHT as usize];

        for (x, y) in robots.iter_mut().map(|robot| robot.solve(best.unwrap().0)) {
            draw[y as usize][x as usize] = true;
        }

        for row in draw {
            println!(
                "{}",
                &row.iter()
                    .map(|full| if *full { 'O' } else { ' ' })
                    .collect::<String>()
            );
        }

        best.unwrap().0.to_string()
    }
}
