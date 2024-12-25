use std::collections::HashMap;

pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        // TODO: Implement
        let mut patterns: Vec<String> = Vec::new();

        fn is_formable(
            patterns: &Vec<String>,
            pattern: &str,
            cache: &mut HashMap<String, usize>,
        ) -> usize {
            if pattern.is_empty() {
                return 1;
            }
            if let Some(cached) = cache.get(pattern) {
                return *cached;
            }
            let mut formable: usize = 0;
            for p in patterns {
                if pattern.starts_with(p) {
                    formable += is_formable(patterns, &pattern[p.len()..], cache)
                }
            }
            cache.insert(pattern.to_string(), formable);
            formable
        }

        let mut num_formable: usize = 0;

        for line in input.lines() {
            if patterns.len() == 0 {
                patterns.extend(line.split(", ").map(|pattern| pattern.to_string()));
            } else if !line.is_empty() {
                num_formable += is_formable(&patterns, line, &mut HashMap::new());
            }
        }

        num_formable.to_string()
    }
}
