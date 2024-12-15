pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut num_safe: usize = 0;
        'report: for report in input.lines() {
            let mut prev: Option<usize> = None;
            let mut increasing = false;
            let mut decreasing = false;
            for level in report.split(" ").map(|str_level| str_level.parse::<usize>().unwrap()) {
                if let Some(prev) = prev {
                    if prev > level {
                        decreasing = true;
                    }
                    else if prev < level {
                        increasing = true;
                    }
                    let diff = prev.abs_diff(level);
                    if diff < 1 || diff > 3 {
                        continue 'report;
                    }
                    if increasing && decreasing {
                        continue 'report;
                    }
                }
                prev = Some(level);
            }
            num_safe += 1;
        }
        num_safe.to_string()
    }
}
