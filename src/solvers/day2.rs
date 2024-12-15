pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut num_safe: usize = 0;
        for report in input.lines() {
            let levels: Vec<usize> = report
                .split(" ")
                .map(|str_level| str_level.parse::<usize>().unwrap())
                .collect();
            'skip: for skip in 0..levels.len() {
                let mut prev: Option<usize> = None;
                let mut increasing = false;
                let mut decreasing = false;
                for level in levels
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != skip)
                    .map(|(_, v)| *v)
                {
                    if let Some(prev) = prev {
                        if prev > level {
                            decreasing = true;
                        } else if prev < level {
                            increasing = true;
                        }
                        let diff = prev.abs_diff(level);
                        if diff < 1 || diff > 3 {
                            continue 'skip;
                        }
                        if increasing && decreasing {
                            continue 'skip;
                        }
                    }
                    prev = Some(level);
                }
                num_safe += 1;
                break;
            }
        }
        num_safe.to_string()
    }
}
