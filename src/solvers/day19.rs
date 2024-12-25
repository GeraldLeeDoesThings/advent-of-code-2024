pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        // TODO: Implement
        let mut patterns: Vec<String> = Vec::new();

        fn is_formable(patterns: &Vec<String>, pattern: &str) -> bool {
            if pattern.is_empty() {
                return true;
            }
            for p in patterns {
                if pattern.starts_with(p) && is_formable(patterns, &pattern[p.len()..]) {
                    return true;
                }
            }
            false
        }

        let mut num_formable: usize = 0;

        for line in input.lines() {
            if patterns.len() == 0 {
                patterns.extend(line.split(", ").map(|pattern| pattern.to_string()));
            } else if !line.is_empty() && is_formable(&patterns, line) {
                num_formable += 1;
            }
        }

        num_formable.to_string()
    }
}
